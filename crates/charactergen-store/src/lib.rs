use charactergen_core::{
    Character, CharacterId, CharacterLifecycle, CharacterPatch, CharacterStore, StoreError,
};
use rocksdb::{MultiThreaded, OptimisticTransactionDB, Options};
use std::{path::Path, sync::Arc};

type Database = OptimisticTransactionDB<MultiThreaded>;

#[derive(Clone)]
pub struct RocksCharacterStore {
    db: Arc<Database>,
}

impl RocksCharacterStore {
    pub fn open(path: impl AsRef<Path>) -> Result<Self, StoreError> {
        let mut options = Options::default();
        options.create_if_missing(true);
        options.set_max_background_jobs(4);
        let db = Database::open(&options, path).map_err(backend)?;
        Ok(Self { db: Arc::new(db) })
    }
}

#[async_trait::async_trait]
impl CharacterStore for RocksCharacterStore {
    async fn list(&self) -> Result<Vec<Character>, StoreError> {
        let db = Arc::clone(&self.db);
        run(move || {
            let mut characters = db
                .prefix_iterator(b"character/")
                .map(|item| {
                    let (_, value) = item.map_err(backend)?;
                    decode(&value)
                })
                .collect::<Result<Vec<_>, _>>()?;
            characters.sort_by(|left, right| left.name.cmp(&right.name));
            Ok(characters)
        })
        .await
    }

    async fn get(&self, id: CharacterId) -> Result<Option<Character>, StoreError> {
        let db = Arc::clone(&self.db);
        run(move || {
            db.get(key(id))
                .map_err(backend)?
                .map(|value| decode(&value))
                .transpose()
        })
        .await
    }

    async fn create(&self, character: Character) -> Result<Character, StoreError> {
        let db = Arc::clone(&self.db);
        run(move || {
            let key = key(character.id);
            let transaction = db.transaction();
            if transaction
                .get_for_update(&key, true)
                .map_err(backend)?
                .is_some()
            {
                return Err(StoreError::Conflict {
                    expected: character.revision,
                    actual: character.revision,
                });
            }
            transaction.put(key, encode(&character)?).map_err(backend)?;
            transaction.commit().map_err(backend)?;
            Ok(character)
        })
        .await
    }

    async fn update(
        &self,
        id: CharacterId,
        patch: CharacterPatch,
    ) -> Result<Character, StoreError> {
        let db = Arc::clone(&self.db);
        run(move || {
            let key = key(id);
            let transaction = db.transaction();
            let raw = transaction
                .get_for_update(&key, true)
                .map_err(backend)?
                .ok_or(StoreError::NotFound)?;
            let mut character = decode(&raw)?;
            patch
                .apply(&mut character)
                .map_err(|conflict| StoreError::Conflict {
                    expected: conflict.expected,
                    actual: conflict.actual,
                })?;
            transaction.put(key, encode(&character)?).map_err(backend)?;
            transaction.commit().map_err(backend)?;
            Ok(character)
        })
        .await
    }

    async fn archive(&self, id: CharacterId) -> Result<(), StoreError> {
        let db = Arc::clone(&self.db);
        run(move || {
            let key = key(id);
            let transaction = db.transaction();
            let raw = transaction
                .get_for_update(&key, true)
                .map_err(backend)?
                .ok_or(StoreError::NotFound)?;
            let mut character = decode(&raw)?;
            character.lifecycle = CharacterLifecycle::Archived;
            character.revision += 1;
            transaction.put(key, encode(&character)?).map_err(backend)?;
            transaction.commit().map_err(backend)?;
            Ok(())
        })
        .await
    }
}

async fn run<T: Send + 'static>(
    operation: impl FnOnce() -> Result<T, StoreError> + Send + 'static,
) -> Result<T, StoreError> {
    tokio::task::spawn_blocking(operation)
        .await
        .map_err(|error| StoreError::Backend(error.to_string()))?
}

fn key(id: CharacterId) -> Vec<u8> {
    format!("character/{}", id.0).into_bytes()
}

fn encode(character: &Character) -> Result<Vec<u8>, StoreError> {
    serde_json::to_vec(character).map_err(|error| StoreError::InvalidData(error.to_string()))
}

fn decode(bytes: &[u8]) -> Result<Character, StoreError> {
    serde_json::from_slice(bytes).map_err(|error| StoreError::InvalidData(error.to_string()))
}

fn backend(error: impl std::fmt::Display) -> StoreError {
    StoreError::Backend(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn persists_characters_across_reopen() {
        let directory = tempfile::tempdir().unwrap();
        let id = {
            let store = RocksCharacterStore::open(directory.path()).unwrap();
            let character = store.create(Character::stub("Mara")).await.unwrap();
            character.id
        };
        let store = RocksCharacterStore::open(directory.path()).unwrap();
        assert_eq!(store.get(id).await.unwrap().unwrap().name, "Mara");
    }

    #[tokio::test]
    async fn rejects_stale_revisions() {
        let directory = tempfile::tempdir().unwrap();
        let store = RocksCharacterStore::open(directory.path()).unwrap();
        let character = store.create(Character::stub("Mara")).await.unwrap();
        let patch = CharacterPatch {
            expected_revision: 0,
            summary: Some("Paramedic".into()),
            ..Default::default()
        };
        store.update(character.id, patch.clone()).await.unwrap();
        assert!(matches!(
            store.update(character.id, patch).await,
            Err(StoreError::Conflict { .. })
        ));
    }
}
