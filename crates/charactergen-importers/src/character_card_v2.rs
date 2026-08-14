use async_trait::async_trait;
use charactergen_core::{
    ImportBatch, ImportError, ImportSource, StagedClaim, StagedEntity, WorldImporter,
};
use serde::Deserialize;
use serde_json::{Value, json};

pub struct CharacterCardV2Importer;

#[derive(Deserialize)]
struct CardEnvelope {
    spec: String,
    #[serde(default)]
    spec_version: String,
    data: CardData,
}

#[derive(Deserialize)]
struct CardData {
    name: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    personality: String,
    #[serde(default)]
    scenario: String,
    #[serde(default)]
    first_mes: String,
    #[serde(default)]
    creator: String,
    #[serde(default)]
    extensions: Value,
}

#[async_trait]
impl WorldImporter for CharacterCardV2Importer {
    fn id(&self) -> &'static str {
        "character-card-v2"
    }

    fn detect(&self, source: &ImportSource) -> f32 {
        let Ok(value) = serde_json::from_slice::<Value>(&source.content) else {
            return 0.0;
        };
        match value.get("spec").and_then(Value::as_str) {
            Some("chara_card_v2") => 1.0,
            _ => 0.0,
        }
    }

    async fn stage(&self, source: ImportSource) -> Result<ImportBatch, ImportError> {
        let card: CardEnvelope = serde_json::from_slice(&source.content)
            .map_err(|error| ImportError::Invalid(error.to_string()))?;
        if card.spec != "chara_card_v2" {
            return Err(ImportError::Unsupported);
        }
        let external_id = format!("card:{}", card.data.name);
        let mut claims = Vec::new();
        for (predicate, value) in [
            ("description", card.data.description),
            ("personality_prompt", card.data.personality),
            ("scenario_prompt", card.data.scenario),
            ("first_message", card.data.first_mes),
        ] {
            if !value.is_empty() {
                claims.push(StagedClaim {
                    subject_external_id: external_id.clone(),
                    predicate: predicate.into(),
                    value: Value::String(value),
                    confidence: 1.0,
                    canonical_candidate: false,
                    source_pointer: format!("/data/{predicate}"),
                });
            }
        }
        Ok(ImportBatch {
            importer_id: self.id().into(),
            source_format: format!("{}:{}", card.spec, card.spec_version),
            entities: vec![StagedEntity {
                external_id,
                entity_type: "character".into(),
                suggested_name: card.data.name,
                claims,
                extensions: json!({ "creator": card.data.creator, "card_extensions": card.data.extensions }),
            }],
            warnings: vec!["Character-card prompt fields are staged as claims and are not automatically canonical.".into()],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn stages_v2_card_without_promoting_prompt_text_to_fact() {
        let source = ImportSource {
            format_hint: None,
            filename: Some("mara.json".into()),
            media_type: Some("application/json".into()),
            content: br#"{"spec":"chara_card_v2","spec_version":"2.0","data":{"name":"Mara","description":"A paramedic"}}"#.to_vec(),
        };
        let batch = CharacterCardV2Importer.stage(source).await.unwrap();
        assert_eq!(batch.entities[0].suggested_name, "Mara");
        assert!(!batch.entities[0].claims[0].canonical_candidate);
    }
}
