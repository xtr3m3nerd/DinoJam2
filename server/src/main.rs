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
