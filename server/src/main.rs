use bevy::{app::ScheduleRunnerSettings, prelude::*, utils::Duration};

use log::{info, trace, warn};
use renet::{
    RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent,
    NETCODE_USER_DATA_BYTES,
};
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant, SystemTime};

// Only clients that can provide the same PROTOCOL_ID that the server is using will be able to
// connect. This can be used to make sure players use the most recent version of the client for
// instance.
pub const PROTOCOL_ID: u64 = 1208;

// TODO - convert server to minimal headless bevy so we can leverage ecs framework
// fn main() {
//     // this app runs once
//     App::new()
//         .insert_resource(ScheduleRunnerSettings::run_once())
//         .add_plugins(MinimalPlugins)
//         .add_system(hello_world_system)
//         .run();
//
//     // this app loops forever at 60 fps
//     App::new()
//         .insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
//             1.0 / 60.0,
//         )))
//         .add_plugins(MinimalPlugins)
//         .add_system(counter)
//         .run();
// }
//
// fn hello_world_system() {
//     println!("hello world");
// }
//
// fn counter(mut state: Local<CounterState>) {
//     if state.count % 60 == 0 {
//         println!("{}", state.count);
//     }
//     state.count += 1;
// }
//
// #[derive(Default)]
// struct CounterState {
//     count: u32,
// }

fn name_from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> String {
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&user_data[0..8]);
    let mut len = u64::from_le_bytes(buffer) as usize;
    len = len.min(NETCODE_USER_DATA_BYTES - 8);
    let data = user_data[8..len + 8].to_vec();
    String::from_utf8(data).unwrap()
}

fn main() {
    env_logger::init();
    let mut game_state = shared::GameState::default();

    let server_addr: SocketAddr = "127.0.0.1:5000".parse().unwrap();
    let mut server: RenetServer = RenetServer::new(
        // Pass the current time to renet, so it can use it to order messages
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap(),
        // Pass a server configeration specifying that we want to allow only 2 clients to connect
        // and that we don't want to authenticate them.
        ServerConfig::new(2, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure),
        // Pass the default connection configeration.
        // This will create a reliabl, unreliable and blocking channel.
        // We only actually need the reliable one, but we can just not use the other 2.
        RenetConnectionConfig::default(),
        UdpSocket::bind(server_addr).unwrap(),
    )
    .unwrap();

    trace!("Server listening on {}", server_addr);

    let mut last_updated = Instant::now();
    loop {
        // Update Server time
        let now = Instant::now();
        server.update(now - last_updated).unwrap();
        last_updated = now;

        while let Some(event) = server.get_event() {
            match event {
                ServerEvent::ClientConnected(id, user_data) => {
                    // Tell the recently joined player about the other player
                    for (player_id, player) in game_state.players.iter() {
                        let event = shared::GameEvent::PlayerJoined {
                            player_id: *player_id,
                            name: player.name.clone(),
                        };
                        server.send_message(id, 0, bincode::serialize(&event).unwrap());
                    }

                    // Add the new player to the game
                    let event = shared::GameEvent::PlayerJoined {
                        player_id: id,
                        name: name_from_user_data(&user_data),
                    };

                    game_state.consume(&event);

                    //Tell all playes that a new player has joined
                    server.broadcast_message(0, bincode::serialize(&event).unwrap());

                    info!("Client {} connected.", id);

                    // Game can start once two players have joined
                    if game_state.players.len() == 2 {
                        let event = shared::GameEvent::BeginGame { goes_first: id };
                        game_state.consume(&event);
                        server.broadcast_message(0, bincode::serialize(&event).unwrap());
                        trace!("The game has begun");
                    }
                }
                ServerEvent::ClientDisconnected(id) => {
                    let event = shared::GameEvent::PlayerDisconnected { player_id: id };
                    game_state.consume(&event);
                    server.broadcast_message(0, bincode::serialize(&event).unwrap());
                    info!("Client {} disconnected.", id);

                    // Then end the game, since game can't go on with a single player
                    let event = shared::GameEvent::EndGame {
                        reason: shared::EndGameReason::PlayerLeft { player_id: id },
                    };
                    game_state.consume(&event);

                    server.broadcast_message(0, bincode::serialize(&event).unwrap());

                    // NOTE: Since we dont authenticate users we cant do any reconnection attempts.
                    // We simply have no way to know if the next user is the same as the one that
                    // disconnected.
                }
            }
        }

        // Receive GameEvents from clients. Broadcast valid events.
        for client_id in server.clients_id().into_iter() {
            while let Some(message) = server.receive_message(client_id, 0) {
                if let Ok(event) = bincode::deserialize::<shared::GameEvent>(&message) {
                    if game_state.validate(&event) {
                        game_state.consume(&event);
                        trace!("Player {} sent:\n\t{:#?}", client_id, event);
                        server.broadcast_message(0, bincode::serialize(&event).unwrap());

                        // Determine if a player has won the game
                        if let Some(winner) = game_state.determine_winner() {
                            let event = shared::GameEvent::EndGame {
                                reason: shared::EndGameReason::PlayerWon { winner },
                            };
                            server.broadcast_message(0, bincode::serialize(&event).unwrap());
                        }
                    } else {
                        warn!("Player {} sent invalid event:\n\t{:#?}", client_id, event);
                    }
                }
            }
        }

        server.send_packets().unwrap();
        std::thread::sleep(Duration::from_millis(50));
    }
}

// use std::{collections::HashMap, net::UdpSocket, time::SystemTime};
//
// use bevy::{
//     diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
//     prelude::*,
// };
// use bevy_egui::{EguiContext, EguiPlugin};
// use bevy_rapier3d::prelude::*;
// use bevy_renet::{
//     renet::{RenetServer, ServerAuthentication, ServerConfig, ServerEvent},
//     RenetServerPlugin,
// };
// use demo_bevy::{
//     server_connection_config, setup_level, spawn_fireball, ClientChannel, NetworkedEntities, Player, PlayerCommand, PlayerInput,
//     Projectile, ServerChannel, ServerMessages, PROTOCOL_ID,
// };
// use renet_visualizer::RenetServerVisualizer;
//
// #[derive(Debug, Default)]
// pub struct ServerLobby {
//     pub players: HashMap<u64, Entity>,
// }
//
// const PLAYER_MOVE_SPEED: f32 = 5.0;
//
// fn new_renet_server() -> RenetServer {
//     let server_addr = "127.0.0.1:5000".parse().unwrap();
//     let socket = UdpSocket::bind(server_addr).unwrap();
//     let connection_config = server_connection_config();
//     let server_config = ServerConfig::new(64, PROTOCOL_ID, server_addr, ServerAuthentication::Unsecure);
//     let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
//     RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
// }
//
// fn main() {
//     let mut app = App::new();
//     app.add_plugins(DefaultPlugins);
//
//     app.add_plugin(RenetServerPlugin::default());
//     app.add_plugin(RapierPhysicsPlugin::<NoUserData>::default());
//     app.add_plugin(RapierDebugRenderPlugin::default());
//     app.add_plugin(FrameTimeDiagnosticsPlugin::default());
//     app.add_plugin(LogDiagnosticsPlugin::default());
//     app.add_plugin(EguiPlugin);
//
//     app.insert_resource(ServerLobby::default());
//     app.insert_resource(new_renet_server());
//     app.insert_resource(RenetServerVisualizer::<200>::default());
//
//     app.add_system(server_update_system);
//     app.add_system(server_network_sync);
//     app.add_system(move_players_system);
//     app.add_system(update_projectiles_system);
//     app.add_system(update_visulizer_system);
//     app.add_system(despawn_projectile_system);
//     app.add_system_to_stage(CoreStage::PostUpdate, projectile_on_removal_system);
//
//     app.add_startup_system(setup_level);
//     app.add_startup_system(setup_simple_camera);
//
//     app.run();
// }
//
// #[allow(clippy::too_many_arguments)]
// fn server_update_system(
//     mut server_events: EventReader<ServerEvent>,
//     mut commands: Commands,
//     mut meshes: ResMut<Assets<Mesh>>,
//     mut materials: ResMut<Assets<StandardMaterial>>,
//     mut lobby: ResMut<ServerLobby>,
//     mut server: ResMut<RenetServer>,
//     mut visualizer: ResMut<RenetServerVisualizer<200>>,
//     players: Query<(Entity, &Player, &Transform)>,
// ) {
//     for event in server_events.iter() {
//         match event {
//             ServerEvent::ClientConnected(id, _) => {
//                 println!("Player {} connected.", id);
//                 visualizer.add_client(*id);
//
//                 // Initialize other players for this new client
//                 for (entity, player, transform) in players.iter() {
//                     let translation: [f32; 3] = transform.translation.into();
//                     let message = bincode::serialize(&ServerMessages::PlayerCreate {
//                         id: player.id,
//                         entity,
//                         translation,
//                     })
//                     .unwrap();
//                     server.send_message(*id, ServerChannel::ServerMessages, message);
//                 }
//
//                 // Spawn new player
//                 let transform = Transform::from_xyz(0.0, 0.51, 0.0);
//                 let player_entity = commands
//                     .spawn_bundle(PbrBundle {
//                         mesh: meshes.add(Mesh::from(shape::Capsule::default())),
//                         material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
//                         transform,
//                         ..Default::default()
//                     })
//                     .insert(RigidBody::Dynamic)
//                     .insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y)
//                     .insert(Collider::capsule_y(0.5, 0.5))
//                     .insert(PlayerInput::default())
//                     .insert(Velocity::default())
//                     .insert(Player { id: *id })
//                     .id();
//
//                 lobby.players.insert(*id, player_entity);
//
//                 let translation: [f32; 3] = transform.translation.into();
//                 let message = bincode::serialize(&ServerMessages::PlayerCreate {
//                     id: *id,
//                     entity: player_entity,
//                     translation,
//                 })
//                 .unwrap();
//                 server.broadcast_message(ServerChannel::ServerMessages, message);
//             }
//             ServerEvent::ClientDisconnected(id) => {
//                 println!("Player {} disconnected.", id);
//                 visualizer.remove_client(*id);
//                 if let Some(player_entity) = lobby.players.remove(id) {
//                     commands.entity(player_entity).despawn();
//                 }
//
//                 let message = bincode::serialize(&ServerMessages::PlayerRemove { id: *id }).unwrap();
//                 server.broadcast_message(ServerChannel::ServerMessages, message);
//             }
//         }
//     }
//
//     for client_id in server.clients_id().into_iter() {
//         while let Some(message) = server.receive_message(client_id, ClientChannel::Command) {
//             let command: PlayerCommand = bincode::deserialize(&message).unwrap();
//             match command {
//                 PlayerCommand::BasicAttack { mut cast_at } => {
//                     println!("Received basic attack from client {}: {:?}", client_id, cast_at);
//
//                     if let Some(player_entity) = lobby.players.get(&client_id) {
//                         if let Ok((_, _, player_transform)) = players.get(*player_entity) {
//                             cast_at[1] = player_transform.translation[1];
//
//                             let direction = (cast_at - player_transform.translation).normalize_or_zero();
//                             let mut translation = player_transform.translation + (direction * 0.7);
//                             translation[1] = 1.0;
//
//                             let fireball_entity = spawn_fireball(&mut commands, &mut meshes, &mut materials, translation, direction);
//                             let message = ServerMessages::SpawnProjectile {
//                                 entity: fireball_entity,
//                                 translation: translation.into(),
//                             };
//                             let message = bincode::serialize(&message).unwrap();
//                             server.broadcast_message(ServerChannel::ServerMessages, message);
//                         }
//                     }
//                 }
//             }
//         }
//         while let Some(message) = server.receive_message(client_id, ClientChannel::Input) {
//             let input: PlayerInput = bincode::deserialize(&message).unwrap();
//             if let Some(player_entity) = lobby.players.get(&client_id) {
//                 commands.entity(*player_entity).insert(input);
//             }
//         }
//     }
// }
//
// fn update_projectiles_system(mut commands: Commands, mut projectiles: Query<(Entity, &mut Projectile)>, time: Res<Time>) {
//     for (entity, mut projectile) in projectiles.iter_mut() {
//         projectile.duration.tick(time.delta());
//         if projectile.duration.finished() {
//             commands.entity(entity).despawn();
//         }
//     }
// }
//
// fn update_visulizer_system(
//     mut egui_context: ResMut<EguiContext>,
//     mut visualizer: ResMut<RenetServerVisualizer<200>>,
//     server: Res<RenetServer>,
// ) {
//     visualizer.update(&server);
//     visualizer.show_window(egui_context.ctx_mut());
// }
//
// #[allow(clippy::type_complexity)]
// fn server_network_sync(mut server: ResMut<RenetServer>, query: Query<(Entity, &Transform), Or<(With<Player>, With<Projectile>)>>) {
//     let mut networked_entities = NetworkedEntities::default();
//     for (entity, transform) in query.iter() {
//         networked_entities.entities.push(entity);
//         networked_entities.translations.push(transform.translation.into());
//     }
//
//     let sync_message = bincode::serialize(&networked_entities).unwrap();
//     server.broadcast_message(ServerChannel::NetworkedEntities, sync_message);
// }
//
// fn move_players_system(mut query: Query<(&mut Velocity, &PlayerInput)>) {
//     for (mut velocity, input) in query.iter_mut() {
//         let x = (input.right as i8 - input.left as i8) as f32;
//         let y = (input.down as i8 - input.up as i8) as f32;
//         let direction = Vec2::new(x, y).normalize_or_zero();
//         velocity.linvel.x = direction.x * PLAYER_MOVE_SPEED;
//         velocity.linvel.z = direction.y * PLAYER_MOVE_SPEED;
//     }
// }
//
// pub fn setup_simple_camera(mut commands: Commands) {
//     // camera
//     commands.spawn_bundle(Camera3dBundle {
//         transform: Transform::from_xyz(-5.5, 5.0, 5.5).looking_at(Vec3::ZERO, Vec3::Y),
//         ..Default::default()
//     });
// }
//
// fn despawn_projectile_system(
//     mut commands: Commands,
//     mut collision_events: EventReader<CollisionEvent>,
//     projectile_query: Query<Option<&Projectile>>,
// ) {
//     for collision_event in collision_events.iter() {
//         if let CollisionEvent::Started(entity1, entity2, _) = collision_event {
//             if let Ok(Some(_)) = projectile_query.get(*entity1) {
//                 commands.entity(*entity1).despawn();
//             }
//             if let Ok(Some(_)) = projectile_query.get(*entity2) {
//                 commands.entity(*entity2).despawn();
//             }
//         }
//     }
// }
//
// fn projectile_on_removal_system(mut server: ResMut<RenetServer>, removed_projectiles: RemovedComponents<Projectile>) {
//     for entity in removed_projectiles.iter() {
//         let message = ServerMessages::DespawnProjectile { entity };
//         let message = bincode::serialize(&message).unwrap();
//
//         server.broadcast_message(ServerChannel::ServerMessages, message);
//     }
// }
