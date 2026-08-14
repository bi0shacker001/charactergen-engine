use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use charactergen_core::{Character, CharacterId, CharacterLifecycle, CharacterPatch};
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;
use uuid::Uuid;

#[derive(Clone, Default)]
struct AppState {
    characters: Arc<RwLock<HashMap<CharacterId, Character>>>,
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
    let app = app(AppState::default());
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
    Json(serde_json::json!({ "status": "ok", "persistence": "in_memory_development" }))
}

async fn list_characters(State(state): State<AppState>) -> Json<Vec<Character>> {
    let mut characters: Vec<_> = state.characters.read().await.values().cloned().collect();
    characters.sort_by(|left, right| left.name.cmp(&right.name));
    Json(characters)
}

async fn create_character(
    State(state): State<AppState>,
    Json(input): Json<CreateCharacter>,
) -> (StatusCode, Json<Character>) {
    let mut character = Character::stub(input.name);
    character.summary = input.summary;
    character.engine_assignment = input.engine_assignment;
    state
        .characters
        .write()
        .await
        .insert(character.id, character.clone());
    (StatusCode::CREATED, Json(character))
}

async fn get_character(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Character>, StatusCode> {
    state
        .characters
        .read()
        .await
        .get(&CharacterId(id))
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

async fn update_character(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(patch): Json<CharacterPatch>,
) -> Result<Json<Character>, StatusCode> {
    let mut characters = state.characters.write().await;
    let character = characters
        .get_mut(&CharacterId(id))
        .ok_or(StatusCode::NOT_FOUND)?;
    patch.apply(character).map_err(|_| StatusCode::CONFLICT)?;
    Ok(Json(character.clone()))
}

async fn archive_character(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, StatusCode> {
    let mut characters = state.characters.write().await;
    let character = characters
        .get_mut(&CharacterId(id))
        .ok_or(StatusCode::NOT_FOUND)?;
    character.lifecycle = CharacterLifecycle::Archived;
    character.revision += 1;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn new_state_starts_empty() {
        let state = AppState::default();
        assert!(state.characters.read().await.is_empty());
        let _router = app(state);
    }
}
