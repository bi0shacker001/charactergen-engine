use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use charactergen_core::{Character, CharacterId, CharacterPatch, CharacterStore, StoreError};
use charactergen_store::RocksCharacterStore;
use serde::Deserialize;
use std::sync::Arc;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    characters: Arc<dyn CharacterStore>,
}

#[derive(Deserialize)]
struct CreateCharacter {
    name: String,
    #[serde(default)]
    summary: String,
    engine_assignment: Option<String>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| "charactergen_server=info".into()),
        )
        .init();
    let data_path =
        std::env::var("CHARACTERGEN_DATA_PATH").unwrap_or_else(|_| "./charactergen.world".into());
    let store = RocksCharacterStore::open(&data_path).expect("open persistent world store");
    let app = app(AppState {
        characters: Arc::new(store),
    });
    let address = std::env::var("CHARACTERGEN_BIND").unwrap_or_else(|_| "127.0.0.1:8787".into());
    let listener = tokio::net::TcpListener::bind(&address)
        .await
        .expect("bind server");
    tracing::info!(%address, "CharacterGen server listening");
    axum::serve(listener, app).await.expect("serve requests");
}

fn app(state: AppState) -> Router {
    Router::new()
        .route("/api/health", get(health))
        .route(
            "/api/characters",
            get(list_characters).post(create_character),
        )
        .route(
            "/api/characters/{id}",
            get(get_character)
                .patch(update_character)
                .delete(archive_character),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_headers(Any)
                .allow_methods(Any),
        )
        .with_state(state)
}

async fn health() -> Json<serde_json::Value> {
    Json(serde_json::json!({ "status": "ok", "persistence": "rocksdb" }))
}

async fn list_characters(
    State(state): State<AppState>,
) -> Result<Json<Vec<Character>>, StatusCode> {
    state
        .characters
        .list()
        .await
        .map(Json)
        .map_err(store_status)
}

async fn create_character(
    State(state): State<AppState>,
    Json(input): Json<CreateCharacter>,
) -> Result<(StatusCode, Json<Character>), StatusCode> {
    let mut character = Character::stub(input.name);
    character.summary = input.summary;
    character.engine_assignment = input.engine_assignment;
    state
        .characters
        .create(character)
        .await
        .map(|created| (StatusCode::CREATED, Json(created)))
        .map_err(store_status)
}

async fn get_character(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Character>, StatusCode> {
    state
        .characters
        .get(CharacterId(id))
        .await
        .map_err(store_status)?
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_character(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(patch): Json<CharacterPatch>,
) -> Result<Json<Character>, StatusCode> {
    state
        .characters
        .update(CharacterId(id), patch)
        .await
        .map(Json)
        .map_err(store_status)
}

async fn archive_character(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    state
        .characters
        .archive(CharacterId(id))
        .await
        .map(|()| StatusCode::NO_CONTENT)
        .map_err(store_status)
}

fn store_status(error: StoreError) -> StatusCode {
    match error {
        StoreError::NotFound => StatusCode::NOT_FOUND,
        StoreError::Conflict { .. } => StatusCode::CONFLICT,
        StoreError::InvalidData(_) | StoreError::Backend(_) => {
            tracing::error!(%error, "storage operation failed");
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_store_starts_empty() {
        let directory = tempfile::tempdir().unwrap();
        let state = AppState {
            characters: Arc::new(RocksCharacterStore::open(directory.path()).unwrap()),
        };
        assert!(state.characters.list().await.unwrap().is_empty());
        let _router = app(state);
    }
}
