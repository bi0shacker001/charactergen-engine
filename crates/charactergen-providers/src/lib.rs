use charactergen_core::{EngineCapability, EngineDescriptor};
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ProviderConfig {
    OpenAi {
        api_key_env: String,
        model: String,
        #[serde(default = "openai_base_url")]
        base_url: String,
    },
    Anthropic {
        api_key_env: String,
        model: String,
        #[serde(default = "anthropic_base_url")]
        base_url: String,
    },
    OpenRouter {
        api_key_env: String,
        model: String,
        #[serde(default = "openrouter_base_url")]
        base_url: String,
    },
    Ollama {
        model: String,
        #[serde(default = "ollama_base_url")]
        base_url: String,
    },
    LlamaCpp {
        model: String,
        #[serde(default = "llama_cpp_base_url")]
        base_url: String,
    },
    OpenAiCompatible {
        base_url: String,
        model: String,
        api_key_env: Option<String>,
    },
    ExternalCharacterEngine {
        base_url: String,
        engine_kind: String,
        api_key_env: Option<String>,
        external_character_id: Option<String>,
    },
}

impl ProviderConfig {
    pub fn descriptor(&self, id: impl Into<String>) -> EngineDescriptor {
        let (display_name, source_kind, structured, streaming) = match self {
            Self::OpenAi { .. } => ("OpenAI", "llm", true, true),
            Self::Anthropic { .. } => ("Anthropic", "llm", true, true),
            Self::OpenRouter { .. } => ("OpenRouter", "llm_router", true, true),
            Self::Ollama { .. } => ("Ollama", "local_llm_server", true, true),
            Self::LlamaCpp { .. } => ("llama.cpp", "local_llm_server", true, true),
            Self::OpenAiCompatible { .. } => ("OpenAI-compatible", "llm", false, true),
            Self::ExternalCharacterEngine { engine_kind, .. } => {
                (engine_kind.as_str(), "character_engine", false, true)
            }
        };
        let mut capabilities = BTreeSet::from([
            EngineCapability::CompleteProfile,
            EngineCapability::SelectAction,
            EngineCapability::GenerateDialogue,
            EngineCapability::InterpretPerception,
            EngineCapability::FormMemory,
            EngineCapability::AppraiseRelationship,
            EngineCapability::Plan,
            EngineCapability::Narrate,
        ]);
        if structured {
            capabilities.insert(EngineCapability::StructuredOutput);
        }
        if streaming {
            capabilities.insert(EngineCapability::Streaming);
        }
        EngineDescriptor {
            id: id.into(),
            display_name: display_name.into(),
            source_kind: source_kind.into(),
            capabilities,
        }
    }
}

fn openai_base_url() -> String {
    "https://api.openai.com/v1".into()
}
fn anthropic_base_url() -> String {
    "https://api.anthropic.com/v1".into()
}
fn openrouter_base_url() -> String {
    "https://openrouter.ai/api/v1".into()
}
fn ollama_base_url() -> String {
    "http://127.0.0.1:11434/v1".into()
}
fn llama_cpp_base_url() -> String {
    "http://127.0.0.1:8080/v1".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ollama_defaults_to_its_local_openai_endpoint() {
        let config: ProviderConfig =
            serde_json::from_str(r#"{"type":"ollama","model":"qwen"}"#).unwrap();
        match config {
            ProviderConfig::Ollama { base_url, .. } => {
                assert_eq!(base_url, "http://127.0.0.1:11434/v1")
            }
            _ => panic!("wrong provider"),
        }
    }
}
