use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BuildingKind(pub usize);

#[derive(Debug, Clone, Component, Copy, PartialEq, Serialize, Deserialize)]
pub struct Building{
    pub position: (u32, u32),
    pub kind: BuildingKind,
    pub health: u32,
}

impl Building {
    pub fn new(
        pos: (u32, u32),
        kind: BuildingKind,
        stats: &Buildings,
    ) -> Self {
        Self {
            position: pos,
            kind,
            health: stats.0[kind.0].max_hp,
        }
    }
}

/// This will be available as a resource
///
/// Contains all the unit descriptors loaded from asset files
#[derive(Deref)]
pub struct Buildings(pub Vec<BuildingDescriptor>);

impl std::ops::Index<BuildingKind> for Buildings {
    type Output = BuildingDescriptor;
    fn index(&self, index: BuildingKind) -> &Self::Output {
        &self.0[index.0]
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct BuildingDescriptor {
    pub name: String,
    // Name that can be shown to players
    pub pub_name: String,
    pub max_hp: u32,
    pub sprite_idx: usize,
    pub faction: String,
}

