use bevy::{
    prelude::*,
    //render::texture::ImageSettings,
    //sprite::Material2dPlugin,
};
use bevy_kira_audio::AudioPlugin;

use crate::config::*;
use crate::debug::DebugPlugin;
use crate::scenes;

pub fn run(app: &mut App) {
    app.insert_resource(WindowDescriptor {
        title: TITLE.to_string(),
        //width: WINDOW_HEIGHT * RESOLUTION,
        //height: WINDOW_HEIGHT,
        ////position: Some(Vec2::new(MONITOR_WIDTH / 4.0, MONITOR_HEIGHT / 4.0)),
        //resizable: false,
        //resize_constraints: bevy::window::WindowResizeConstraints {
        //    min_width: WINDOW_HEIGHT * RESOLUTION,
        //    max_width: WINDOW_HEIGHT * RESOLUTION,
        //    min_height: WINDOW_HEIGHT,
        //    max_height: WINDOW_HEIGHT,
        //},
        //mode: WindowMode::Windowed,
        ..Default::default()
    });
    app
        //.insert_resource(ImageSettings::default_nearest())
        //.init_resource::<resources::setting::Setting>()
        //.init_resource::<resources::dictionary::Dictionary>()
        //.add_state(SceneState::LoadingScene)
        //.add_startup_system(plugins::music::background_audio_channel_setup)
        //.add_system(plugins::music::play_background_music)
        .add_plugins(DefaultPlugins)
        //.add_plugin(Material2dPlugin::<PostProcessingMaterial>::default())
        //.add_plugin(plugins::camera::CameraPlugin)
        //.add_plugin(plugins::input::InputHandlePlugin)
        //.add_plugin(plugins::player::PlayerPlugin)
        //.add_plugin(scenes::loading_scene::LoadingScenePlugin)
        //.add_plugin(scenes::main_menu_scene::MainMenuScenePlugin)
        //.add_plugin(scenes::level_select_scene::LevelSelectScenePlugin)
        //.add_plugin(scenes::playing_scene::PlayingScenePlugin)
        //.add_plugin(scenes::victory_scene::VictoryScenePlugin)
        //.add_plugin(scenes::game_over_scene::GameOverScenePlugin)
        //.add_plugin(scenes::quit_to_menu_scene::QuitToMenuScenePlugin)
        .add_plugin(scenes::test_scene::TestScenePlugin)
        .add_plugin(AudioPlugin);

    if USE_DEBUG {
        app.add_plugin(DebugPlugin);
    }
    app.run();
}
