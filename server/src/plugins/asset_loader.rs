use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

use crate::AppState;
use shared::asset_management::{terrain_descriptors::*, unit_descriptors::*};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::AssetsLoading)
                .continue_to_state(AppState::ServerListening)
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                    // put gamedata-related things here
                    "meta.assets",
                ])
                .with_collection::<UnitAssets>()
                .with_collection::<TerrainAssets>(),
        );
        app.add_plugin(TomlAssetPlugin::<UnitAsset>::new(&["units.toml"]));
        app.add_plugin(TomlAssetPlugin::<TerrainAsset>::new(&["terrain.toml"]));
    }
}
