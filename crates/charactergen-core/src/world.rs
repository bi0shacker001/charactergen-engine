use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub parent_id: Option<String>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct Relationship {
    pub source_character_id: CharacterId,
    pub target_character_id: CharacterId,
    pub familiarity: f32,
    pub affection: f32,
    pub trust: f32,
    pub respect: f32,
    pub comfort: f32,
    pub attraction: f32,
    pub fear: f32,
    pub resentment: f32,
    pub obligation: f32,
    pub future_contact_interest: f32,
}

use crate::CharacterId;
