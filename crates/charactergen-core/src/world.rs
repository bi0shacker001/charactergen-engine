use crate::CharacterId;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use uuid::Uuid;

macro_rules! entity_id {
    ($name:ident) => {
        #[derive(
            Clone,
            Copy,
            Debug,
            Eq,
            Hash,
            JsonSchema,
            Ord,
            PartialEq,
            PartialOrd,
            Serialize,
            Deserialize,
        )]
        #[serde(transparent)]
        pub struct $name(pub Uuid);

        impl $name {
            pub fn new() -> Self {
                Self(Uuid::now_v7())
            }
        }

        impl Default for $name {
            fn default() -> Self {
                Self::new()
            }
        }
    };
}

entity_id!(InstanceId);
entity_id!(WorldId);
entity_id!(LocationId);
entity_id!(RouteId);

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct Instance {
    pub id: InstanceId,
    pub revision: u64,
    pub name: String,
    #[serde(default)]
    pub world_ids: Vec<WorldId>,
    pub simulation_time_ms: i64,
    pub time_scale: f32,
    pub paused: bool,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct World {
    pub id: WorldId,
    pub instance_id: InstanceId,
    pub revision: u64,
    pub name: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub extensions: BTreeMap<String, serde_json::Value>,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct Location {
    pub id: LocationId,
    pub world_id: WorldId,
    pub revision: u64,
    pub name: String,
    pub kind: String,
    pub parent_id: Option<LocationId>,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub acoustic_isolation: f32,
    #[serde(default)]
    pub blocks_visibility: bool,
}

#[derive(Clone, Debug, JsonSchema, Serialize, Deserialize)]
pub struct Route {
    pub id: RouteId,
    pub world_id: WorldId,
    pub from: LocationId,
    pub to: LocationId,
    pub travel_time_ms: u64,
    #[serde(default)]
    pub bidirectional: bool,
    #[serde(default)]
    pub sound_attenuation: f32,
}

#[derive(Clone, Debug, Default)]
pub struct LocationGraph {
    locations: BTreeMap<LocationId, Location>,
    routes: Vec<Route>,
}

impl LocationGraph {
    pub fn new(locations: impl IntoIterator<Item = Location>, routes: Vec<Route>) -> Self {
        Self {
            locations: locations
                .into_iter()
                .map(|location| (location.id, location))
                .collect(),
            routes,
        }
    }

    pub fn shortest_travel_time(&self, from: LocationId, to: LocationId) -> Option<u64> {
        if !self.locations.contains_key(&from) || !self.locations.contains_key(&to) {
            return None;
        }
        let mut distances = BTreeMap::from([(from, 0_u64)]);
        let mut frontier = vec![(from, 0_u64)];
        while let Some((index, _)) = frontier.iter().enumerate().min_by_key(|(_, item)| item.1) {
            let (current, distance) = frontier.swap_remove(index);
            if current == to {
                return Some(distance);
            }
            if distance > distances.get(&current).copied().unwrap_or(u64::MAX) {
                continue;
            }
            for (neighbor, cost) in self.neighbors(current) {
                let candidate = distance.saturating_add(cost);
                if candidate < distances.get(&neighbor).copied().unwrap_or(u64::MAX) {
                    distances.insert(neighbor, candidate);
                    frontier.push((neighbor, candidate));
                }
            }
        }
        None
    }

    pub fn sound_level_at(&self, source: LocationId, listener: LocationId, volume: f32) -> f32 {
        if source == listener {
            return volume.clamp(0.0, 1.0);
        }
        let mut levels = BTreeMap::from([(source, volume.clamp(0.0, 1.0))]);
        let mut frontier = vec![source];
        while let Some(current) = frontier.pop() {
            let current_level = levels[&current];
            for route in self.routes_from(current) {
                let neighbor = if route.from == current {
                    route.to
                } else {
                    route.from
                };
                let boundary = self.locations.get(&neighbor).map_or(1.0, |location| {
                    1.0 - location.acoustic_isolation.clamp(0.0, 1.0)
                });
                let propagated =
                    current_level * (1.0 - route.sound_attenuation.clamp(0.0, 1.0)) * boundary;
                if propagated > levels.get(&neighbor).copied().unwrap_or(0.0) + f32::EPSILON {
                    levels.insert(neighbor, propagated);
                    frontier.push(neighbor);
                }
            }
        }
        levels
            .get(&listener)
            .copied()
            .unwrap_or(0.0)
            .clamp(0.0, 1.0)
    }

    fn neighbors(&self, location: LocationId) -> impl Iterator<Item = (LocationId, u64)> + '_ {
        self.routes.iter().filter_map(move |route| {
            if route.from == location {
                Some((route.to, route.travel_time_ms))
            } else if route.bidirectional && route.to == location {
                Some((route.from, route.travel_time_ms))
            } else {
                None
            }
        })
    }

    fn routes_from(&self, location: LocationId) -> impl Iterator<Item = &Route> {
        self.routes.iter().filter(move |route| {
            route.from == location || (route.bidirectional && route.to == location)
        })
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn location(id: LocationId, world_id: WorldId, isolation: f32) -> Location {
        Location {
            id,
            world_id,
            revision: 0,
            name: "Place".into(),
            kind: "room".into(),
            parent_id: None,
            description: String::new(),
            acoustic_isolation: isolation,
            blocks_visibility: false,
        }
    }

    #[test]
    fn routes_determine_travel_time_and_sound_loss() {
        let world_id = WorldId::new();
        let kitchen = LocationId::new();
        let hall = LocationId::new();
        let bedroom = LocationId::new();
        let graph = LocationGraph::new(
            [
                location(kitchen, world_id, 0.0),
                location(hall, world_id, 0.1),
                location(bedroom, world_id, 0.5),
            ],
            vec![
                Route {
                    id: RouteId::new(),
                    world_id,
                    from: kitchen,
                    to: hall,
                    travel_time_ms: 5_000,
                    bidirectional: true,
                    sound_attenuation: 0.2,
                },
                Route {
                    id: RouteId::new(),
                    world_id,
                    from: hall,
                    to: bedroom,
                    travel_time_ms: 3_000,
                    bidirectional: true,
                    sound_attenuation: 0.3,
                },
            ],
        );
        assert_eq!(graph.shortest_travel_time(kitchen, bedroom), Some(8_000));
        let heard = graph.sound_level_at(kitchen, bedroom, 1.0);
        assert!(heard > 0.2 && heard < 0.3);
    }
}
