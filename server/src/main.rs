use std::{net::UdpSocket, time::SystemTime};

use log::{info, trace, warn};

use bevy::{
    app::ScheduleRunnerSettings,
    asset::AssetPlugin,
    //diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
    prelude::*,
    reflect::{FromReflect, Reflect},
    utils::Duration,
};
use bevy_renet::{
    renet::{
        RenetConnectionConfig, RenetServer, ServerAuthentication, ServerConfig, ServerEvent,
        NETCODE_USER_DATA_BYTES,
    },
    RenetServerPlugin,
};
use iyes_loopless::prelude::*;
use iyes_progress::prelude::*;
//use renet_visualizer::RenetServerVisualizer;

mod plugins;

#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, Default, Reflect, FromReflect, serde::Deserialize,
)]
pub enum AppState {
    #[default]
    AssetsLoading,
    ServerListening,
}

fn main() {
    env_logger::init();

    // this app loops forever at 60 fps
    let mut app = App::new();
    app.insert_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
    )));
    app.add_plugins(MinimalPlugins);
    app.add_plugin(AssetPlugin);

    app.add_loopless_state(AppState::AssetsLoading);
    app.register_type::<AppState>();
    app.add_plugin(ProgressPlugin::new(AppState::AssetsLoading));

    // Plugins
    app.add_plugin(RenetServerPlugin);
    app.add_plugin(plugins::asset_loader::AssetLoaderPlugin);

    app.insert_resource(shared::GameState::default());
    app.insert_resource(new_renet_server());
    //app.insert_resource(RenetServerVisualizer::<200>::default());

    app.add_startup_system(debug_server_state);
    app.add_system(server_update_system);

    app.run();
}

fn new_renet_server() -> RenetServer {
    let server_addr = "127.0.0.1:5000".parse().unwrap();
    let socket = UdpSocket::bind(server_addr).unwrap();
    let connection_config = RenetConnectionConfig::default();
    let server_config = ServerConfig::new(
        2,
        shared::PROTOCOL_ID,
        server_addr,
        ServerAuthentication::Unsecure,
    );
    let current_time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap();
    RenetServer::new(current_time, server_config, connection_config, socket).unwrap()
}

fn debug_server_state(server: Res<RenetServer>) {
    trace!("Server listening on {}", server.addr());
    println!("Server listening on {}", server.addr());
}

fn server_update_system(
    mut server_events: EventReader<ServerEvent>,
    mut server: ResMut<RenetServer>,
    mut game_state: ResMut<shared::GameState>,
    buildings: Res<shared::buildings::Buildings>,
    units: Res<shared::units::Units>,
    terrain: Res<shared::terrain::Terrain>,
) {
    for event in server_events.iter() {
        match event {
            ServerEvent::ClientConnected(id, user_data) => {
                // Tell the recently joined player about the other player
                for (player_id, player) in game_state.players.iter() {
                    let event = shared::GameEvent::PlayerJoined {
                        player_id: *player_id,
                        name: player.name.clone(),
                    };
                    server.send_message(*id, 0, bincode::serialize(&event).unwrap());
                }

                // Add the new player to the game
                let event = shared::GameEvent::PlayerJoined {
                    player_id: *id,
                    name: name_from_user_data(&user_data),
                };

                game_state.consume(&event, &buildings, &units, &terrain);

                //Tell all playes that a new player has joined
                server.broadcast_message(0, bincode::serialize(&event).unwrap());

                info!("Client {} connected.", id);

                // Game can start once two players have joined
                if game_state.players.len() == 2 {
                    let event = shared::GameEvent::BeginGame { goes_first: *id };
                    game_state.consume(&event, &buildings, &units, &terrain);
                    server.broadcast_message(0, bincode::serialize(&event).unwrap());
                    trace!("The game has begun");
                }
            }
            ServerEvent::ClientDisconnected(id) => {
                let event = shared::GameEvent::PlayerDisconnected { player_id: *id };
                game_state.consume(&event, &buildings, &units, &terrain);
                server.broadcast_message(0, bincode::serialize(&event).unwrap());
                info!("Client {} disconnected.", id);

                // Then end the game, since game can't go on with a single player
                let event = shared::GameEvent::EndGame {
                    reason: shared::EndGameReason::PlayerLeft { player_id: *id },
                };
                game_state.consume(&event, &buildings, &units, &terrain);

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
                if game_state.validate(&event, &buildings, &units, &terrain) {
                    game_state.consume(&event, &buildings, &units, &terrain);
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
}

fn name_from_user_data(user_data: &[u8; NETCODE_USER_DATA_BYTES]) -> String {
    let mut buffer = [0u8; 8];
    buffer.copy_from_slice(&user_data[0..8]);
    let mut len = u64::from_le_bytes(buffer) as usize;
    len = len.min(NETCODE_USER_DATA_BYTES - 8);
    let data = user_data[8..len + 8].to_vec();
    String::from_utf8(data).unwrap()
}
