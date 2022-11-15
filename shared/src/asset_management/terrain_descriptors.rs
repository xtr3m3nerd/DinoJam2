use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

#[derive(Clone, Component, Copy, Debug, Eq, PartialEq)]
pub struct TerrainKind(pub usize);

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

#[derive(AssetCollection)]
pub struct TerrainAssets {
    #[asset(key = "meta.terrain", collection(typed))]
    #[allow(dead_code)]
    handles: Vec<Handle<TerrainAsset>>,
    #[allow(dead_code)]
    all: TerrainMarker,
}

pub struct TerrainMarker;

#[derive(bevy::reflect::TypeUuid, serde::Deserialize)]
#[uuid = "25dce551-aa36-4c9e-b9a8-5f8dfe0df239"]
pub struct TerrainAsset {
    terrain: Vec<TerrainDescriptor>,
}

impl FromWorld for TerrainMarker {
    fn from_world(world: &mut World) -> Self {
        let mut all = Vec::new();
        {
            let assets = world.resource::<Assets<TerrainAsset>>();
            for (_, asset) in assets.iter() {
                for desc in asset.terrain.iter() {
                    all.push(desc.clone());
                }
            }
        }
        world.insert_resource(Terrain(all));
        TerrainMarker
    }
}
