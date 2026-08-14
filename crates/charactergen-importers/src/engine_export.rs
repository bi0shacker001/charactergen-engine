use async_trait::async_trait;
use charactergen_core::{ImportBatch, ImportError, ImportSource, WorldImporter};
use serde::{Deserialize, Serialize};
use serde_json::Value;

pub struct EngineExportImporter;

#[derive(Debug, Deserialize, Serialize)]
struct EngineExport {
    format: String,
    engine: String,
    entities: Vec<charactergen_core::StagedEntity>,
    #[serde(default)]
    metadata: Value,
}

#[async_trait]
impl WorldImporter for EngineExportImporter {
    fn id(&self) -> &'static str {
        "charactergen-engine-export-v1"
    }

    fn detect(&self, source: &ImportSource) -> f32 {
        let Ok(value) = serde_json::from_slice::<Value>(&source.content) else {
            return 0.0;
        };
        match value.get("format").and_then(Value::as_str) {
            Some("charactergen-engine-export-v1") => 1.0,
            _ => 0.0,
        }
    }

    async fn stage(&self, source: ImportSource) -> Result<ImportBatch, ImportError> {
        let export: EngineExport = serde_json::from_slice(&source.content)
            .map_err(|error| ImportError::Invalid(error.to_string()))?;
        if export.format != "charactergen-engine-export-v1" {
            return Err(ImportError::Unsupported);
        }
        Ok(ImportBatch {
            importer_id: self.id().into(),
            source_format: format!("{}:{}", export.format, export.engine),
            entities: export.entities,
            warnings: Vec::new(),
        })
    }
}
