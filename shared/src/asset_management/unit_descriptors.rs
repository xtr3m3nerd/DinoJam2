use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::units::*;

/// internal thingy to load all the asset files
/// and accumulate them into a Units resource
#[derive(AssetCollection)]
pub struct UnitAssets {
    #[asset(key = "meta.units", collection(typed))]
    #[allow(dead_code)]
    handles: Vec<Handle<UnitAsset>>,
    #[allow(dead_code)]
    all: UnitMarker,
}

pub struct UnitMarker;

#[derive(bevy::reflect::TypeUuid, serde::Deserialize)]
#[uuid = "78a56857-57f8-4f05-b639-c2c3b7a00085"]
pub struct UnitAsset {
    unit: Vec<UnitDescriptor>,
}

impl FromWorld for UnitMarker {
    fn from_world(world: &mut World) -> Self {
        let mut all = Vec::new();
        {
            let assets = world.resource::<Assets<UnitAsset>>();
            for (_, asset) in assets.iter() {
                for desc in asset.unit.iter() {
                    all.push(desc.clone());
                }
            }
        }
        world.insert_resource(Units(all));
        UnitMarker
    }
}
