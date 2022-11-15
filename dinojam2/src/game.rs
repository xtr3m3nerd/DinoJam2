use bevy::{
    prelude::*,
    //render::texture::ImageSettings,
    //sprite::Material2dPlugin,
};
use bevy_ecs_tilemap::TilemapPlugin;
use bevy_kira_audio::AudioPlugin;
//use leafwing_input_manager::prelude::*;
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;

use crate::{
    config::*,
    states::AppState,
    debug::DebugPlugin,
    scenes,
    plugins,
    util,
};

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
    // app.insert_resource(ImageSettings::default_nearest());
    // app.init_resource::<resources::setting::Setting>();
    // app.init_resource::<resources::dictionary::Dictionary>();
    // app.add_state(SceneState::LoadingScene);
    // app.add_startup_system(plugins::music::background_audio_channel_setup);
    // app.add_system(plugins::music::play_background_music);

    // Bevy
    app.add_plugins(DefaultPlugins);

    //app.add_loopless_state(AppState::MainMenu);

    // Globals
    app.add_loopless_state(AppState::AssetsLoading);
    app.register_type::<AppState>();
    app.add_stage_before(CoreStage::Update, "preupdate", SystemStage::parallel());

    // External plugins
    app.add_plugin(ProgressPlugin::new(AppState::AssetsLoading));
    app.add_plugin(TilemapPlugin);
    //app.add_plugin(InputManagerPlugin::<gameplay::Action>::default());
    app.add_plugin(AudioPlugin);

    // Internal plugins
    app.add_plugin(plugins::asset_loader::AssetLoaderPlugin);
    //app.add_state(AppState::MainMenu);

    // app.add_plugin(Material2dPlugin::<PostProcessingMaterial>::default());
    app.add_system(util::debug_current_state);
    app.add_plugin(plugins::camera::CameraPlugin);
    // app.add_plugin(plugins::input::InputHandlePlugin);
    // app.add_plugin(plugins::player::PlayerPlugin);
    // app.add_plugin(scenes::loading_scene::LoadingScenePlugin);
    app.add_plugin(scenes::main_menu_scene::MainMenuScenePlugin);
    // app.add_plugin(scenes::game_scene::GameScenePlugin);
    app.add_plugin(scenes::test_tile_scene::TestTileScenePlugin);
    // app.add_plugin(scenes::level_select_scene::LevelSelectScenePlugin);
    // app.add_plugin(scenes::playing_scene::PlayingScenePlugin);
    // app.add_plugin(scenes::victory_scene::VictoryScenePlugin);
    // app.add_plugin(scenes::game_over_scene::GameOverScenePlugin);
    // app.add_plugin(scenes::quit_to_menu_scene::QuitToMenuScenePlugin);
    //app.add_plugin(scenes::test_scene::TestScenePlugin);

    if USE_DEBUG {
        app.add_plugin(DebugPlugin);
    }
    app.run();
}

// TODO - impliment bevy client
// use bevy::prelude::*;
// use bevy_renet::{
//     renet::{
//         ClientAuthentication, DefaultChannel, RenetClient, RenetConnectionConfig, RenetError, RenetServer, ServerAuthentication,
//         ServerConfig, ServerEvent,
//     },
//     run_if_client_connected, RenetClientPlugin, RenetServerPlugin,
// };
//
// use std::time::SystemTime;
// use std::{collections::HashMap, net::UdpSocket};
//
// use serde::{Deserialize, Serialize};
//
// const PROTOCOL_ID: u64 = 7;
//
// const PLAYER_MOVE_SPEED: f32 = 1.0;
//
// #[derive(Debug, Default, Serialize, Deserialize, Component)]
// struct PlayerInput {
//     up: bool,
//     down: bool,
//     left: bool,
//     right: bool,
// }
//
// #[derive(Debug, Component)]
// struct Player {
//     id: u64,
// }
//
// #[derive(Debug, Default)]
// struct Lobby {
//     players: HashMap<u64, Entity>,
// }
//
// #[derive(Debug, Serialize, Deserialize, Component)]
// enum ServerMessages {
//     PlayerConnected { id: u64 },
//     PlayerDisconnected { id: u64 },
// }
//
// fn new_renet_client() -> RenetClient {
//     let server_addr = "127.0.0.1:5000".parse().unwrap();
//     let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
//     let connection_config = RenetConnectionConfig::default();
//     let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
//     let client_id = current_time.as_millis() as u64;
//     let authentication = ClientAuthentication::Unsecure {
//         client_id,
//         protocol_id: PROTOCOL_ID,
//         server_addr,
//         user_data: None,
//     };
//     RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
// }
//
// fn new_renet_server() -> RenetServer {
//     let server_addr = "127.0.0.1:5000".parse().unwrap();
//     let socket = UdpSocket::bind(server_addr).unwrap();
//     let connection_config = RenetConnectionConfig::default();
//     let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
//     let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
//     RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
// }
//
// fn main() {
//     println!("Usage: run with \"server\" or \"client\" argument");
//     let args: Vec<String> = std::env::args().collect();
//
//     let exec_type = &args[1];
//     let is_host = match exec_type.as_str() {
//         "client" => false,
//         "server" => true,
//         _ => panic!("Invalid argument, must be \"client\" or \"server\"."),
//     };
//
//     let mut app = App::new();
//     app.add_plugins(DefaultPlugins);
//     app.insert_resource(Lobby::default());
//
//     if is_host {
//         app.add_plugin(RenetServerPlugin::default());
//         app.insert_resource(new_renet_server());
//         app.add_system(server_update_system);
//         app.add_system(server_sync_players);
//         app.add_system(move_players_system);
//     } else {
//         app.add_plugin(RenetClientPlugin::default());
//         app.insert_resource(new_renet_client());
//         app.insert_resource(PlayerInput::default());
//         app.add_system(player_input);
//         app.add_system(client_send_input.with_run_criteria(run_if_client_connected));
//         app.add_system(client_sync_players.with_run_criteria(run_if_client_connected));
//     }
//
//     app.add_startup_system(setup);
//     app.add_system(panic_on_error_system);
//
//     app.run();
// }
//
// fn server_update_system(
//     mut server_events: EventReader<ServerEvent>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut lobby: ResMut<Lobby>,
//     mut server: ResMut<RenetServer>,
// ) {
//     for event in server_events.iter() {
//         match event {
//             ServerEvent::ClientConnected(id, _) => {
//                 println!("Player {} connected.", id);
//                 // Spawn player cube
//                 let player_entity = commands
//                     .spawn_bundle(PbrBundle {
//                         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//                         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//                         transform: Transform::from_xyz(0.0, 0.5, 0.0),
//                         ..Default::default()
//                     })
//                     .insert(PlayerInput::default())
//                     .insert(Player { id: *id })
//                     .id();
//
//                 // We could send an InitState with all the players id and positions for the client
//                 // but this is easier to do.
//                 for &player_id in lobby.players.keys() {
//                     let message = bincode::serialize(&ServerMessages::PlayerConnected { id: player_id }).unwrap();
//                     server.send_message(*id, DefaultChannel::Reliable, message);
//                 }
//
//                 lobby.players.insert(*id, player_entity);
//
//                 let message = bincode::serialize(&ServerMessages::PlayerConnected { id: *id }).unwrap();
//                 server.broadcast_message(DefaultChannel::Reliable, message);
//             }
//             ServerEvent::ClientDisconnected(id) => {
//                 println!("Player {} disconnected.", id);
//                 if let Some(player_entity) = lobby.players.remove(id) {
//                     commands.entity(player_entity).despawn();
//                 }
//
//                 let message = bincode::serialize(&ServerMessages::PlayerDisconnected { id: *id }).unwrap();
//                 server.broadcast_message(DefaultChannel::Reliable, message);
//             }
//         }
//     }
//
//     for client_id in server.clients_id().into_iter() {
//         while let Some(message) = server.receive_message(client_id, DefaultChannel::Reliable) {
//             let player_input: PlayerInput = bincode::deserialize(&message).unwrap();
//             if let Some(player_entity) = lobby.players.get(&client_id) {
//                 commands.entity(*player_entity).insert(player_input);
//             }
//         }
//     }
// }
//
// fn server_sync_players(mut server: ResMut<RenetServer>, query: Query<(&Transform, &Player)>) {
//     let mut players: HashMap<u64, [f32; 3]> = HashMap::new();
//     for (transform, player) in query.iter() {
//         players.insert(player.id, transform.translation.into());
//     }
//
//     let sync_message = bincode::serialize(&players).unwrap();
//     server.broadcast_message(DefaultChannel::Unreliable, sync_message);
// }
//
// fn client_sync_players(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut client: ResMut<RenetClient>,
//     mut lobby: ResMut<Lobby>,
// ) {
//     while let Some(message) = client.receive_message(DefaultChannel::Reliable) {
//         let server_message = bincode::deserialize(&message).unwrap();
//         match server_message {
//             ServerMessages::PlayerConnected { id } => {
//                 println!("Player {} connected.", id);
//                 let player_entity = commands
//                     .spawn_bundle(PbrBundle {
//                         mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
//                         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//                         transform: Transform::from_xyz(0.0, 0.5, 0.0),
//                         ..Default::default()
//                     })
//                     .id();
//
//                 lobby.players.insert(id, player_entity);
//             }
//             ServerMessages::PlayerDisconnected { id } => {
//                 println!("Player {} disconnected.", id);
//                 if let Some(player_entity) = lobby.players.remove(&id) {
//                     commands.entity(player_entity).despawn();
//                 }
//             }
//         }
//     }
//
//     while let Some(message) = client.receive_message(DefaultChannel::Unreliable) {
//         let players: HashMap<u64, [f32; 3]> = bincode::deserialize(&message).unwrap();
//         for (player_id, translation) in players.iter() {
//             if let Some(player_entity) = lobby.players.get(player_id) {
//                 let transform = Transform {
//                     translation: (*translation).into(),
//                     ..Default::default()
//                 };
//                 commands.entity(*player_entity).insert(transform);
//             }
//         }
//     }
// }
//
// /// set up a simple 3D scene
// fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
//     // plane
//     commands.spawn_bundle(PbrBundle {
//         mesh: meshes.add(Mesh::from(shape::Plane { size: 5.0 })),
//         material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//         ..Default::default()
//     });
//     // light
//     commands.spawn_bundle(PointLightBundle {
//         point_light: PointLight {
//             intensity: 1500.0,
//             shadows_enabled: true,
//             ..Default::default()
//         },
//         transform: Transform::from_xyz(4.0, 8.0, 4.0),
//         ..Default::default()
//     });
//     // camera
//     commands.spawn_bundle(Camera3dBundle {
//         transform: Transform::from_xyz(-2.0, 2.5, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
//         ..Default::default()
//     });
// }
//
// fn player_input(keyboard_input: Res<Input<KeyCode>>, mut player_input: ResMut<PlayerInput>) {
//     player_input.left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
//     player_input.right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
//     player_input.up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
//     player_input.down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
// }
//
// fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
//     let input_message = bincode::serialize(&*player_input).unwrap();
//
//     client.send_message(DefaultChannel::Reliable, input_message);
// }
//
// fn move_players_system(mut query: Query<(&mut Transform, &PlayerInput)>, time: Res<Time>) {
//     for (mut transform, input) in query.iter_mut() {
//         let x = (input.right as i8 - input.left as i8) as f32;
//         let y = (input.down as i8 - input.up as i8) as f32;
//         transform.translation.x += x * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
//         transform.translation.z += y * PLAYER_MOVE_SPEED * time.delta().as_secs_f32();
//     }
// }
//
// // If any error is found we just panic
// fn panic_on_error_system(mut renet_error: EventReader<RenetError>) {
//     for e in renet_error.iter() {
//         panic!("{}", e);
//     }
// }

// /// Next grouping of client code to evaluate
// use std::{collections::HashMap, net::UdpSocket, time::SystemTime};
//
// use bevy::{
//     diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
//     prelude::*,
// };
// use bevy_egui::{EguiContext, EguiPlugin};
// use bevy_renet::{
//     renet::{ClientAuthentication, RenetClient, RenetError},
//     run_if_client_connected, RenetClientPlugin,
// };
// use demo_bevy::{
//     client_connection_config, setup_level, ClientChannel, NetworkedEntities, PlayerCommand, PlayerInput, Ray3d, ServerChannel,
//     ServerMessages, PROTOCOL_ID,
// };
// use renet_visualizer::{RenetClientVisualizer, RenetVisualizerStyle};
// use smooth_bevy_cameras::{LookTransform, LookTransformBundle, LookTransformPlugin, Smoother};
//
// #[derive(Component)]
// struct ControlledPlayer;
//
// #[derive(Default)]
// struct NetworkMapping(HashMap<Entity, Entity>);
//
// #[derive(Debug)]
// struct PlayerInfo {
//     client_entity: Entity,
//     server_entity: Entity,
// }
//
// #[derive(Debug, Default)]
// struct ClientLobby {
//     players: HashMap<u64, PlayerInfo>,
// }
//
// fn new_renet_client() -> RenetClient {
//     let server_addr = "127.0.0.1:5000".parse().unwrap();
//     let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
//     let connection_config = client_connection_config();
//     let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
//     let client_id = current_time.as_millis() as u64;
//     let authentication = ClientAuthentication::Unsecure {
//         client_id,
//         protocol_id: PROTOCOL_ID,
//         server_addr,
//         user_data: None,
//     };
//
//     RenetClient::new(current_time, socket, connection_config, authentication).unwrap()
// }
//
// fn main() {
//     let mut app = App::new();
//     app.add_plugins(DefaultPlugins);
//     app.add_plugin(RenetClientPlugin::default());
//     app.add_plugin(LookTransformPlugin);
//     app.add_plugin(FrameTimeDiagnosticsPlugin::default());
//     app.add_plugin(LogDiagnosticsPlugin::default());
//     app.add_plugin(EguiPlugin);
//
//     app.add_event::<PlayerCommand>();
//
//     app.insert_resource(ClientLobby::default());
//     app.insert_resource(PlayerInput::default());
//     app.insert_resource(new_renet_client());
//     app.insert_resource(NetworkMapping::default());
//
//     app.add_system(player_input);
//     app.add_system(camera_follow);
//     app.add_system(update_target_system);
//     app.add_system(client_send_input.with_run_criteria(run_if_client_connected));
//     app.add_system(client_send_player_commands.with_run_criteria(run_if_client_connected));
//     app.add_system(client_sync_players.with_run_criteria(run_if_client_connected));
//
//     app.insert_resource(RenetClientVisualizer::<200>::new(RenetVisualizerStyle::default()));
//     app.add_system(update_visulizer_system);
//
//     app.add_startup_system(setup_level);
//     app.add_startup_system(setup_camera);
//     app.add_startup_system(setup_target);
//     app.add_system(panic_on_error_system);
//
//     app.run();
// }
//
// // If any error is found we just panic
// fn panic_on_error_system(mut renet_error: EventReader<RenetError>) {
//     for e in renet_error.iter() {
//         panic!("{}", e);
//     }
// }
//
// fn update_visulizer_system(
//     mut egui_context: ResMut<EguiContext>,
//     mut visualizer: ResMut<RenetClientVisualizer<200>>,
//     client: Res<RenetClient>,
//     mut show_visualizer: Local<bool>,
//     keyboard_input: Res<Input<KeyCode>>,
// ) {
//     visualizer.add_network_info(client.network_info());
//     if keyboard_input.just_pressed(KeyCode::F1) {
//         *show_visualizer = !*show_visualizer;
//     }
//     if *show_visualizer {
//         visualizer.show_window(egui_context.ctx_mut());
//     }
// }
//
// fn player_input(
//     keyboard_input: Res<Input<KeyCode>>,
//     mut player_input: ResMut<PlayerInput>,
//     mouse_button_input: Res<Input<MouseButton>>,
//     target_query: Query<&Transform, With<Target>>,
//     mut player_commands: EventWriter<PlayerCommand>,
// ) {
//     player_input.left = keyboard_input.pressed(KeyCode::A) || keyboard_input.pressed(KeyCode::Left);
//     player_input.right = keyboard_input.pressed(KeyCode::D) || keyboard_input.pressed(KeyCode::Right);
//     player_input.up = keyboard_input.pressed(KeyCode::W) || keyboard_input.pressed(KeyCode::Up);
//     player_input.down = keyboard_input.pressed(KeyCode::S) || keyboard_input.pressed(KeyCode::Down);
//
//     if mouse_button_input.just_pressed(MouseButton::Left) {
//         let target_transform = target_query.single();
//         player_commands.send(PlayerCommand::BasicAttack {
//             cast_at: target_transform.translation,
//         });
//     }
// }
//
// fn client_send_input(player_input: Res<PlayerInput>, mut client: ResMut<RenetClient>) {
//     let input_message = bincode::serialize(&*player_input).unwrap();
//
//     client.send_message(ClientChannel::Input, input_message);
// }
//
// fn client_send_player_commands(mut player_commands: EventReader<PlayerCommand>, mut client: ResMut<RenetClient>) {
//     for command in player_commands.iter() {
//         let command_message = bincode::serialize(command).unwrap();
//         client.send_message(ClientChannel::Command, command_message);
//     }
// }
//
// fn client_sync_players(
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut client: ResMut<RenetClient>,
//     mut lobby: ResMut<ClientLobby>,
//     mut network_mapping: ResMut<NetworkMapping>,
// ) {
//     let client_id = client.client_id();
//     while let Some(message) = client.receive_message(ServerChannel::ServerMessages) {
//         let server_message = bincode::deserialize(&message).unwrap();
//         match server_message {
//             ServerMessages::PlayerCreate { id, translation, entity } => {
//                 println!("Player {} connected.", id);
//                 let mut client_entity = commands.spawn_bundle(PbrBundle {
//                     mesh: meshes.add(Mesh::from(shape::Capsule::default())),
//                     material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//                     transform: Transform::from_xyz(translation[0], translation[1], translation[2]),
//                     ..Default::default()
//                 });
//
//                 if client_id == id {
//                     client_entity.insert(ControlledPlayer);
//                 }
//
//                 let player_info = PlayerInfo {
//                     server_entity: entity,
//                     client_entity: client_entity.id(),
//                 };
//                 lobby.players.insert(id, player_info);
//                 network_mapping.0.insert(entity, client_entity.id());
//             }
//             ServerMessages::PlayerRemove { id } => {
//                 println!("Player {} disconnected.", id);
//                 if let Some(PlayerInfo {
//                     server_entity,
//                     client_entity,
//                 }) = lobby.players.remove(&id)
//                 {
//                     commands.entity(client_entity).despawn();
//                     network_mapping.0.remove(&server_entity);
//                 }
//             }
//             ServerMessages::SpawnProjectile { entity, translation } => {
//                 let projectile_entity = commands.spawn_bundle(PbrBundle {
//                     mesh: meshes.add(Mesh::from(shape::Icosphere {
//                         radius: 0.1,
//                         subdivisions: 5,
//                     })),
//                     material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
//                     transform: Transform::from_translation(translation.into()),
//                     ..Default::default()
//                 });
//                 network_mapping.0.insert(entity, projectile_entity.id());
//             }
//             ServerMessages::DespawnProjectile { entity } => {
//                 if let Some(entity) = network_mapping.0.remove(&entity) {
//                     commands.entity(entity).despawn();
//                 }
//             }
//         }
//     }
//
//     while let Some(message) = client.receive_message(ServerChannel::NetworkedEntities) {
//         let networked_entities: NetworkedEntities = bincode::deserialize(&message).unwrap();
//
//         for i in 0..networked_entities.entities.len() {
//             if let Some(entity) = network_mapping.0.get(&networked_entities.entities[i]) {
//                 let translation = networked_entities.translations[i].into();
//                 let transform = Transform {
//                     translation,
//                     ..Default::default()
//                 };
//                 commands.entity(*entity).insert(transform);
//             }
//         }
//     }
// }
//
// #[derive(Component)]
// struct Target;
//
// fn update_target_system(
//     windows: Res<Windows>,
//     mut target_query: Query<&mut Transform, With<Target>>,
//     camera_query: Query<(&Camera, &GlobalTransform)>,
// ) {
//     let (camera, camera_transform) = camera_query.single();
//     let mut target_transform = target_query.single_mut();
//     if let Some(ray) = Ray3d::from_screenspace(&windows, camera, camera_transform) {
//         if let Some(pos) = ray.intersect_y_plane(1.0) {
//             target_transform.translation = pos;
//         }
//     }
// }
//
// fn setup_camera(mut commands: Commands) {
//     commands
//         .spawn_bundle(LookTransformBundle {
//             transform: LookTransform {
//                 eye: Vec3::new(0.0, 8., 2.5),
//                 target: Vec3::new(0.0, 0.5, 0.0),
//             },
//             smoother: Smoother::new(0.9),
//         })
//         .insert_bundle(Camera3dBundle {
//             transform: Transform::from_xyz(0., 8.0, 2.5).looking_at(Vec3::new(0.0, 0.5, 0.0), Vec3::Y),
//             ..default()
//         });
// }
//
// fn setup_target(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
//     commands
//         .spawn_bundle(PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Icosphere {
//                 radius: 0.1,
//                 subdivisions: 5,
//             })),
//             material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
//             transform: Transform::from_xyz(0.0, 0., 0.0),
//             ..Default::default()
//         })
//         .insert(Target);
// }
//
// fn camera_follow(
//     mut camera_query: Query<&mut LookTransform, (With<Camera>, Without<ControlledPlayer>)>,
//     player_query: Query<&Transform, With<ControlledPlayer>>,
// ) {
//     let mut cam_transform = camera_query.single_mut();
//     if let Ok(player_transform) = player_query.get_single() {
//         cam_transform.eye.x = player_transform.translation.x;
//         cam_transform.eye.z = player_transform.translation.z + 2.5;
//         cam_transform.target = player_transform.translation;
//     }
// }
