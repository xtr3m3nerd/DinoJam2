use bevy::prelude::*;
use bevy::asset::Asset;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::toml::TomlAssetPlugin;

use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

use crate::states::AppState;
use crate::asset_management::{
    asset_collections::*,
    terrain_descriptors::*,
    unit_descriptors::*,
    HandleFromPath,
};

pub struct AssetLoaderPlugin;

impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(AppState::AssetsLoading)
                .continue_to_state(AppState::MainMenu)
                .with_dynamic_collections::<StandardDynamicAssetCollection>(vec![
                    // put UI-related things here
                    // (fonts, images, sounds, scenes)
                    "ui.assets",
                    // put gamedata-related things here
                    "meta.assets",
                    // put gameplay visual related things here
                    // (spritesheets, etc)
                    "game.assets",
                    // put cutscene-related things here
                    // (images, metadata, scenes, etc)
                    //"cutscene.assets",
                    //"audio.assets",
                ])
                .with_collection::<UiAssets>()
                //.with_collection::<UiScenes>()
                .with_collection::<UnitAssets>()
                .with_collection::<TerrainAssets>()
                //.with_collection::<CutsceneAssets>()
                .with_collection::<MapAssets>(),
                //.with_collection::<AudioAssets>(),
        );
        app.add_plugin(TomlAssetPlugin::<UnitAsset>::new(&["units.toml"]));
        app.add_plugin(TomlAssetPlugin::<TerrainAsset>::new(&["terrain.toml"]));
        // app.add_plugin(TomlAssetPlugin::<CutsceneMetaAsset>::new(&[
        //     "cutscene.toml",
        // ]));
        app.add_system_to_stage(CoreStage::Last, debug_progress.run_in_state(AppState::AssetsLoading));
        app.add_enter_system(AppState::InGame, debug_units);
        app.add_startup_system(enable_hot_reloading);

        // workaround cuz scenes dont support asset handles properly
        app.register_type::<HandleFromPath<Image>>();
        app.add_system_to_stage("preupdate", setup_handle_from_path::<Image>);
        app.register_type::<HandleFromPath<Font>>();
        app.add_system_to_stage("preupdate", setup_handle_from_path::<Font>);
        app.register_type::<HandleFromPath<DynamicScene>>();
        app.add_system_to_stage("preupdate", setup_handle_from_path::<DynamicScene>);
        // … add others if needed
    }
}

fn enable_hot_reloading(ass: Res<AssetServer>) {
    ass.watch_for_changes().ok();
}

#[allow(dead_code)]
fn debug_progress(counter: Res<ProgressCounter>) {
    let progress = counter.progress();
    debug!("Progress: {}/{}", progress.done, progress.total);
    let progress = counter.progress_complete();
    debug!("Full Progress: {}/{}", progress.done, progress.total);
}

#[allow(dead_code)]
fn debug_units(bp: Res<Units>) {
    dbg!(&bp.0);
}

fn setup_handle_from_path<T: Asset>(
    mut commands: Commands,
    q: Query<(Entity, &HandleFromPath<T>), Changed<HandleFromPath<T>>>,
    ass: Res<AssetServer>,
) {
    for (e, hfp) in q.iter() {
        commands.entity(e).insert(ass.load::<T, _>(&hfp.path));
    }
}
