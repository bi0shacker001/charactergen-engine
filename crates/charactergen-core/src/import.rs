use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct ImportSource {
    pub format_hint: Option<String>,
    pub filename: Option<String>,
    pub media_type: Option<String>,
    pub content: Vec<u8>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct StagedClaim {
    pub subject_external_id: String,
    pub predicate: String,
    pub value: Value,
    pub confidence: f32,
    pub canonical_candidate: bool,
    pub source_pointer: String,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct StagedEntity {
    pub external_id: String,
    pub entity_type: String,
    pub suggested_name: String,
    pub claims: Vec<StagedClaim>,
    pub extensions: Value,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct ImportBatch {
    pub importer_id: String,
    pub source_format: String,
    pub entities: Vec<StagedEntity>,
    pub warnings: Vec<String>,
}

#[derive(Debug, thiserror::Error)]
pub enum ImportError {
    #[error("unsupported source")]
    Unsupported,
    #[error("invalid source: {0}")]
    Invalid(String),
}

#[async_trait]
pub trait WorldImporter: Send + Sync {
    fn id(&self) -> &'static str;
    fn detect(&self, source: &ImportSource) -> f32;
    async fn stage(&self, source: ImportSource) -> Result<ImportBatch, ImportError>;
}
