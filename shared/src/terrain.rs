use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Clone, Component, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct TerrainKind(pub usize);

/// This will be available as a resource
/// Contains all the terrain descriptors loaded from asset files
#[derive(Deref)]
pub struct Terrain(pub Vec<TerrainDescriptor>);

impl std::ops::Index<TerrainKind> for Terrain {
    type Output = TerrainDescriptor;
    fn index(&self, index: TerrainKind) -> &Self::Output {
        &self.0[index.0]
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct TerrainDescriptor {
    pub name: String,
    pub sprite_idx: usize,
    pub wall: bool,
}

