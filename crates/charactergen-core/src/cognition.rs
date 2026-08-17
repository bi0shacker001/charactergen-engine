use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, JsonSchema, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UtilityCurve {
    Linear,
    Quadratic,
    Inverse,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct Consideration {
    pub name: String,
    pub input: f32,
    pub weight: f32,
    pub curve: UtilityCurve,
}

impl Consideration {
    pub fn score(&self) -> f32 {
        let input = self.input.clamp(0.0, 1.0);
        let value = match self.curve {
            UtilityCurve::Linear => input,
            UtilityCurve::Quadratic => input * input,
            UtilityCurve::Inverse => 1.0 - input,
        };
        value * self.weight.max(0.0)
    }
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct ActionCandidate {
    pub id: String,
    pub available: bool,
    pub base_utility: f32,
    #[serde(default)]
    pub considerations: Vec<Consideration>,
}

impl ActionCandidate {
    pub fn utility(&self) -> f32 {
        if !self.available {
            return f32::NEG_INFINITY;
        }
        self.considerations
            .iter()
            .fold(self.base_utility, |score, item| score + item.score())
    }
}

pub fn choose_action(candidates: &[ActionCandidate]) -> Option<&ActionCandidate> {
    candidates
        .iter()
        .filter(|candidate| candidate.available)
        .max_by(|left, right| {
            left.utility()
                .total_cmp(&right.utility())
                .then_with(|| right.id.cmp(&left.id))
        })
}

#[derive(Clone, Copy, Debug, Default, JsonSchema, Serialize, Deserialize)]
pub struct RelevanceSignals {
    pub player_social_proximity: f32,
    pub geographic_proximity: f32,
    pub recent_interaction: f32,
    pub active_commitment: f32,
    pub unresolved_event: f32,
    pub story_pressure: f32,
    pub host_pinned: bool,
}

impl RelevanceSignals {
    pub fn score(self) -> f32 {
        if self.host_pinned {
            return 1.0;
        }
        (self.player_social_proximity.clamp(0.0, 1.0) * 0.3
            + self.geographic_proximity.clamp(0.0, 1.0) * 0.2
            + self.recent_interaction.clamp(0.0, 1.0) * 0.15
            + self.active_commitment.clamp(0.0, 1.0) * 0.15
            + self.unresolved_event.clamp(0.0, 1.0) * 0.1
            + self.story_pressure.clamp(0.0, 1.0) * 0.1)
            .clamp(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unavailable_actions_cannot_win() {
        let candidates = vec![
            ActionCandidate {
                id: "impossible".into(),
                available: false,
                base_utility: 100.0,
                considerations: vec![],
            },
            ActionCandidate {
                id: "wait".into(),
                available: true,
                base_utility: 0.2,
                considerations: vec![],
            },
        ];
        assert_eq!(choose_action(&candidates).unwrap().id, "wait");
    }

    #[test]
    fn host_pins_override_calculated_relevance() {
        assert_eq!(
            RelevanceSignals {
                host_pinned: true,
                ..Default::default()
            }
            .score(),
            1.0
        );
    }
}
