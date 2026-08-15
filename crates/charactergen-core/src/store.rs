use crate::{Character, CharacterId, CharacterPatch};

#[derive(Debug, thiserror::Error)]
pub enum StoreError {
    #[error("character not found")]
    NotFound,
    #[error("character revision conflict: expected {expected}, actual {actual}")]
    Conflict { expected: u64, actual: u64 },
    #[error("stored data is invalid: {0}")]
    InvalidData(String),
    #[error("storage operation failed: {0}")]
    Backend(String),
}

#[async_trait::async_trait]
pub trait CharacterStore: Send + Sync {
    async fn list(&self) -> Result<Vec<Character>, StoreError>;
    async fn get(&self, id: CharacterId) -> Result<Option<Character>, StoreError>;
    async fn create(&self, character: Character) -> Result<Character, StoreError>;
    async fn update(&self, id: CharacterId, patch: CharacterPatch)
    -> Result<Character, StoreError>;
    async fn archive(&self, id: CharacterId) -> Result<(), StoreError>;
}
