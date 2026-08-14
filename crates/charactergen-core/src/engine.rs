use async_trait::async_trait;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Eq, JsonSchema, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EngineCapability {
    CompleteProfile,
    SelectAction,
    GenerateDialogue,
    InterpretPerception,
    FormMemory,
    AppraiseRelationship,
    Plan,
    Narrate,
    StructuredOutput,
    Streaming,
    ProviderOwnedState,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct EngineDescriptor {
    pub id: String,
    pub display_name: String,
    pub source_kind: String,
    pub capabilities: BTreeSet<EngineCapability>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct EngineRequest {
    pub request_id: String,
    pub capability: EngineCapability,
    pub subject_id: Option<String>,
    pub instructions: String,
    pub context: Value,
    pub output_schema: Option<Value>,
    pub idempotency_key: String,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct EngineResponse {
    pub provider_id: String,
    pub model_or_engine: Option<String>,
    pub content: Value,
    pub raw_text: Option<String>,
    pub warnings: Vec<String>,
    pub usage: Option<Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum EngineError {
    #[error("capability is not supported: {0:?}")]
    Unsupported(EngineCapability),
    #[error("provider configuration is invalid: {0}")]
    Configuration(String),
    #[error("provider request failed: {0}")]
    Request(String),
    #[error("provider returned an invalid response: {0}")]
    InvalidResponse(String),
}

#[async_trait]
pub trait CharacterEngine: Send + Sync {
    fn descriptor(&self) -> &EngineDescriptor;
    async fn execute(&self, request: EngineRequest) -> Result<EngineResponse, EngineError>;
}
