use std::marker::PhantomData;

use bevy::prelude::*;
use bevy::asset::Asset;
use bevy::reflect::{ Reflect, FromReflect };

pub mod asset_collections;
pub mod terrain_descriptors;
pub mod unit_descriptors;

/// This can be added to things to be loaded from scenes
/// (bevy cannot handle handles in scenes yet ;) )
/// At runtime, will be replaced with handle
#[derive(Component, Clone, Reflect, FromReflect)]
#[reflect(Component)]
pub struct HandleFromPath<T: Asset> {
    pub path: String,
    #[reflect(ignore)]
    pub _pd: PhantomData<T>,
}

impl<T: Asset> Default for HandleFromPath<T> {
    fn default() -> Self {
        Self {
            path: "".into(),
            _pd: PhantomData,
        }
    }
}

impl<T: Asset> From<&str> for HandleFromPath<T> {
    fn from(str: &str) -> Self {
        Self {
            path: str.into(),
            _pd: PhantomData,
        }
    }
}
