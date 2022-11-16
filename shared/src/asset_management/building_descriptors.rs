use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::buildings::*;

#[derive(AssetCollection)]
pub struct BuildingAssets {
    #[asset(key = "meta.terrain", collection(typed))]
    #[allow(dead_code)]
    handles: Vec<Handle<BuildingAsset>>,
    #[allow(dead_code)]
    all: BuildingMarker,
}

pub struct BuildingMarker;

#[derive(bevy::reflect::TypeUuid, serde::Deserialize)]
#[uuid = "e97afdad-38f7-4f30-acb5-043a11f2d598"]
pub struct BuildingAsset {
    building: Vec<BuildingDescriptor>,
}

impl FromWorld for BuildingMarker {
    fn from_world(world: &mut World) -> Self {
        let mut all = Vec::new();
        {
            let assets = world.resource::<Assets<BuildingAsset>>();
            for (_, asset) in assets.iter() {
                for desc in asset.building.iter() {
                    all.push(desc.clone());
                }
            }
        }
        world.insert_resource(Buildings(all));
        BuildingMarker
    }
}
