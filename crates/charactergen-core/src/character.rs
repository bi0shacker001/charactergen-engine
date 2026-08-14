use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

#[derive(Clone, Copy, Debug, Eq, Hash, JsonSchema, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CharacterId(pub Uuid);

impl CharacterId {
    pub fn new() -> Self {
        Self(Uuid::now_v7())
    }
}

impl Default for CharacterId {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy, Debug, Eq, JsonSchema, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CharacterLifecycle {
    Reference,
    Stub,
    Supporting,
    Principal,
    Dormant,
    Archived,
}

#[derive(Clone, Debug, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct ScoredAttribute {
    pub score: f32,
    #[serde(default)]
    pub confidence: f32,
    #[serde(default)]
    pub notes: String,
}

#[derive(Clone, Debug, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct SourceRef {
    pub source_type: String,
    pub source_id: String,
    #[serde(default)]
    pub source_uri: Option<String>,
    #[serde(default)]
    pub imported_at: Option<String>,
}

#[derive(Clone, Debug, JsonSchema, PartialEq, Serialize, Deserialize)]
pub struct Character {
    pub id: CharacterId,
    pub revision: u64,
    pub lifecycle: CharacterLifecycle,
    pub name: String,
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub summary: String,
    #[serde(default)]
    pub traits: BTreeMap<String, ScoredAttribute>,
    #[serde(default)]
    pub interests: BTreeMap<String, ScoredAttribute>,
    #[serde(default)]
    pub goals: Vec<String>,
    #[serde(default)]
    pub location_id: Option<String>,
    #[serde(default)]
    pub engine_assignment: Option<String>,
    #[serde(default)]
    pub sources: Vec<SourceRef>,
    #[serde(default)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

impl Character {
    pub fn stub(name: impl Into<String>) -> Self {
        Self {
            id: CharacterId::new(),
            revision: 0,
            lifecycle: CharacterLifecycle::Stub,
            name: name.into(),
            aliases: Vec::new(),
            summary: String::new(),
            traits: BTreeMap::new(),
            interests: BTreeMap::new(),
            goals: Vec::new(),
            location_id: None,
            engine_assignment: None,
            sources: Vec::new(),
            extensions: BTreeMap::new(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, JsonSchema, Serialize)]
pub struct CharacterPatch {
    pub expected_revision: u64,
    pub name: Option<String>,
    pub lifecycle: Option<CharacterLifecycle>,
    pub summary: Option<String>,
    pub traits: Option<BTreeMap<String, ScoredAttribute>>,
    pub interests: Option<BTreeMap<String, ScoredAttribute>>,
    pub goals: Option<Vec<String>>,
    pub location_id: Option<Option<String>>,
    pub engine_assignment: Option<Option<String>>,
}

impl CharacterPatch {
    pub fn apply(self, character: &mut Character) -> Result<(), CharacterConflict> {
        if self.expected_revision != character.revision {
            return Err(CharacterConflict {
                expected: self.expected_revision,
                actual: character.revision,
            });
        }
        if let Some(name) = self.name {
            character.name = name;
        }
        if let Some(lifecycle) = self.lifecycle {
            character.lifecycle = lifecycle;
        }
        if let Some(summary) = self.summary {
            character.summary = summary;
        }
        if let Some(traits) = self.traits {
            character.traits = traits;
        }
        if let Some(interests) = self.interests {
            character.interests = interests;
        }
        if let Some(goals) = self.goals {
            character.goals = goals;
        }
        if let Some(location_id) = self.location_id {
            character.location_id = location_id;
        }
        if let Some(engine_assignment) = self.engine_assignment {
            character.engine_assignment = engine_assignment;
        }
        character.revision += 1;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
#[error("character revision conflict: expected {expected}, actual {actual}")]
pub struct CharacterConflict {
    pub expected: u64,
    pub actual: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn patches_require_the_current_revision() {
        let mut character = Character::stub("Mara");
        CharacterPatch {
            expected_revision: 0,
            summary: Some("A paramedic".into()),
            ..Default::default()
        }
        .apply(&mut character)
        .unwrap();
        assert_eq!(character.revision, 1);
        assert!(CharacterPatch::default().apply(&mut character).is_err());
    }
}
