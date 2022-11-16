use serde::{Deserialize, Serialize};
use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct UnitKind(pub usize);

#[derive(Debug, Clone, Component, Copy, PartialEq, Serialize, Deserialize)]
pub struct Unit{
    pub position: (u32, u32),
    pub kind: UnitKind,
    pub health: u32,
    pub range_remaining: u32,
}

impl Unit {
    pub fn new(
        pos: (u32, u32),
        kind: UnitKind,
        stats: &Units,
    ) -> Self {
        Self {
            position: pos,
            kind,
            health: stats.0[kind.0].max_hp,
            range_remaining: stats.0[kind.0].move_range,
        }
    }
}

/// This will be available as a resource
///
/// Contains all the unit descriptors loaded from asset files
#[derive(Deref)]
pub struct Units(pub Vec<UnitDescriptor>);

impl std::ops::Index<UnitKind> for Units {
    type Output = UnitDescriptor;
    fn index(&self, index: UnitKind) -> &Self::Output {
        &self.0[index.0]
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct UnitDescriptor {
    pub name: String,
    // Name that can be shown to players
    pub pub_name: String,
    pub max_hp: u32,
    pub move_range: u32,
    pub attack_range: u32,
    pub damage: u32,
    pub cost: u32,
    pub sprite_idx: usize,
    pub faction: String,
}

