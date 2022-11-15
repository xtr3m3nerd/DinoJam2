use bevy::prelude::*;
use bevy_renet::RenetClientPlugin;
use renet::{
    ClientAuthentication, RenetClient, RenetConnectionConfig, RenetError, NETCODE_USER_DATA_BYTES,
};
use std::{net::UdpSocket, time::SystemTime};
use store::{GameState, GameEvent, EndGameReason};

// This id need to be the same as the server is using
const PROTOCOL_ID: u64 = 1208;
const TILE_SIZE: f32 = 160.0;
const BOARD_SIZE: f32 = TILE_SIZE * 3.0;
const BOARD_POS_Y: f32 = -30.0;

fn main() {
    // Get username from stdin args
    let args = std::env::args().collect::<Vec<String>>();
    let username = &args[1];

    App::new()
        .insert_resource(WindowDescriptor {
            // Adding the usernamt to the window title makes debugging a whole lot easier
            title: format!("TicTacToe <{}>", username),
            width: 480.0,
            height: 540.0,
            ..default()
        })
    .insert_resource(ClearColor(Color::hex("282828").unwrap()))
        .add_plugins(DefaultPlugins)
        .add_plugin(RenetClientPlugin)
        .insert_resource(new_renet_client(&username).unwrap())
        .add_system(handle_renet_error)
        .insert_resource(GameState::default())
        .add_startup_system(setup)
        .add_system(update_waiting_text)
        .add_system(input)
        .add_system_to_stage(
            CoreStage::PostUpdate,
            // Renet exposes a nice run criteria
            // that can be used to make sure that this system only runs when the client is
            // connected
            receive_events_from_server.with_run_criteria(bevy_renet::run_if_client_connected),
        )
        .add_event::<GameEvent>()
        .add_system(update_board)
        .add_system(change_ui_by_stage)
        .add_system(update_in_game_ui)
        .run();
}

/////////////// RENET NETWORKING //////////////////
// Creates a RenetClient thats already connected to a server.
// Returns an Err if connection fails
fn new_renet_client(username: &String) -> anyhow::Result<RenetClient> {
    let server_addr = "127.0.0.1:5000".parse()?;
    let socket = UdpSocket::bind("127.0.0.1:0")?;
    let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
    let client_id = current_time.as_millis() as u64;

    // Place username in user data
    let mut user_data = [0u8; NETCODE_USER_DATA_BYTES];
    if username.len() > NETCODE_USER_DATA_BYTES - 8 {
        panic!("Username is too big");
    }

    user_data[0..8].copy_from_slice(&(username.len() as u64).to_le_bytes());
    user_data[8..username.len() + 8].copy_from_slice(username.as_bytes());

    let client = RenetClient::new(
        current_time,
        socket,
        client_id,
        RenetConnectionConfig::default(),
        ClientAuthentication::Unsecure {
            client_id,
            protocol_id: PROTOCOL_ID,
            server_addr,
            user_data: Some(user_data),
        },
    )?;

    Ok(client)
}

// If there is any network error we just panic
// Ie. Client has lost connect to server, if internet is gon or server shut off.
fn handle_renet_error(mut renet_error: EventReader<RenetError>) {
    for err in renet_error.iter() {
        panic!("{}", err);
    }
}

fn receive_events_from_server(
    mut client: ResMut<RenetClient>,
    mut game_state: ResMut<GameState>,
    mut game_events: EventWriter<GameEvent>,
){
    while let Some(message) = client.receive_message(0) {
        // Whenever the server sends a message we know that it musbe be a game event
        let event: GameEvent = bincode::deserialize(&message).unwrap();
        trace!("{:#?}", event);

        // We trust the server, no need to validate
        game_state.consume(&event);

        // Send the events into the beby event system so systems can react to it
        game_events.send(event);
    }
}

/////////////// COMPONENTS //////////////
#[derive(Component)]
struct UIRoot;

#[derive(Component)]
struct WaitingText;

type TileIndex = usize;
#[derive(Component)]
struct HoverDot(pub TileIndex);

#[derive(Component)]
struct PlayerHandle(pub u64);


/////////////// SETUP /////////////

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // TicTacToe is a 2D Game
    // To show 2D sprites we need a 2D Camera
    commands.spawn_bundle(Camera2dBundle::default());

    // Spawn board background
    commands.spawn_bundle(SpriteBundle {
        transform: Transform::from_xyz(0.0, BOARD_POS_Y, 0.0),
        sprite: Sprite {
            custom_size: Some(Vec2::new(480.0,480.0)),
            ..default()
        },
        texture: asset_server.load("background.png").into(),
        ..default()
    });


    // Spawn pregame ui
    commands
        // A container that centers its children on the screen
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: UiRect {
                    left: Val::Px(0.0),
                    top: Val::Px(0.0),
                    ..default()
                },
                size: Size::new(Val::Percent(100.0), Val::Px(60.0)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            color: Color::NONE.into(),
            ..default()
        })
    .insert(UIRoot)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle::from_section(
                        "Waiting for an opponent...",
                        TextStyle {
                            font: asset_server.load("Inconsolata.ttf"),
                            font_size: 24.0,
                            color: Color::hex("ebdbb2").unwrap(),
                        },
                ))
                .insert(WaitingText);
            });

    // Spawn a dot in each tile for hover effect
    for x in 0..3 {
        for y in 0..3 {
            commands
                .spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(
                                   TILE_SIZE * (x as f32 - 1.0),
                                   BOARD_POS_Y + TILE_SIZE * (y as f32 - 1.0),
                                   0.0,
                               ),
                               sprite: Sprite {
                                   color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                                   custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                   ..default()
                               },
                               texture: asset_server.load("dot.png").into(),
                               ..default()
                })
            .insert(HoverDot(x + y*3));
            }
    }
}

////////// UPDATE SYSTEMS //////////////////
fn update_waiting_text(mut text_query: Query<&mut Text, With<WaitingText>>, time: Res<Time>) {
    if let Ok(mut text) = text_query.get_single_mut() {
        let num_dots = (time.time_since_startup().as_secs() % 3) + 1;
        text.sections[0].value = format!(
            "Waiting for an opponent{}{}",
            ".".repeat(num_dots as usize),
            " ".repeat(3 - num_dots as usize)
        )
    }
}

fn update_board(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut game_events: EventReader<GameEvent>,
    asset_server: Res<AssetServer>,
) {
    for event in game_events.iter() {
        match event {
            GameEvent::PlaceTile { player_id, at } => {
                let x = at % 3;
                let y = at / 3;
                let texture = asset_server.load(match game_state.get_player_tile(player_id) {
                    store::Tile::Tac => "tac.png",
                    store::Tile::Tic => "tic.png",
                    store::Tile::Empty => "dot.png", // This should never happen
                });

                commands.spawn_bundle(SpriteBundle {
                    transform: Transform::from_xyz(
                                   TILE_SIZE * (x as f32 - 1.0),
                                   BOARD_POS_Y + TILE_SIZE * (y as f32 - 1.0),
                                   0.0,
                               ),
                               sprite: Sprite {
                                   custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
                                   ..default()
                               },
                               texture: texture.into(),
                               ..default()
                });
            },
            _ => {}
        }
    }
}

fn change_ui_by_stage(
    mut commands: Commands,
    game_state: Res<GameState>,
    mut game_events: EventReader<GameEvent>,
    mut ui_root: Query<(Entity, &mut Style), With<UIRoot>>,
    asset_server: Res<AssetServer>,
) {
    let (ui_root_entity, mut ui_root_style) = ui_root.get_single_mut().unwrap();
    let mut ui_root = commands.entity(ui_root_entity);

    for event in game_events.iter() {
        match event {
            GameEvent::BeginGame { goes_first: _ } => {
                // Remove waiting text when game beings
                ui_root.despawn_descendants();

                // Spawn in game ui
                ui_root_style.justify_content = JustifyContent::SpaceBetween;
                ui_root.with_children(|parent| {
                    for (player_id, player) in game_state.players.iter() {
                        let is_active_player = game_state.active_player_id == *player_id;
                        let is_tac_player = player.piece == store::Tile::Tac;

                        parent.spawn_bundle(TextBundle::from_section(
                                player.name.clone(),
                                TextStyle {
                                    font: asset_server.load("Inconsolata.ttf"),
                                    font_size: 24.0,
                                    color: if !is_active_player {
                                        Color::hex("ebdbb2").unwrap()
                                    } else {
                                        if is_tac_player {
                                            Color::hex("d65d0e").unwrap()
                                        } else {
                                            Color::hex("458488").unwrap()
                                        }
                                    },
                                },
                        ))
                            .insert(PlayerHandle(*player_id));
                        }
                });
            },
            GameEvent::EndGame { reason } => {
                // Despawn in game ui
                ui_root.despawn_descendants();
                ui_root_style.justify_content = JustifyContent::Center;
                match reason {
                    EndGameReason::PlayerLeft{ player_id: _ } => {
                        ui_root.with_children(|parent| {
                            parent.spawn_bundle(TextBundle::from_section(
                                    "Your opponent has left",
                                    TextStyle {
                                        font: asset_server.load("Inconsolata.ttf"),
                                        font_size: 24.0,
                                        color: Color::hex("ebdbb2").unwrap(),
                                    },
                            ));
                        });
                    },
                    EndGameReason::PlayerWon{ winner } => {
                        ui_root.with_children(|parent| {
                            let winner_player = game_state.players.get(winner).unwrap();
                            let is_tac_player = winner_player.piece == store::Tile::Tac;

                            parent.spawn_bundle(TextBundle::from_section(
                                    format!("{} has won!", winner_player.name.clone()),
                                    TextStyle {
                                        font: asset_server.load("Inconsolata.ttf"),
                                        font_size: 24.0,
                                        color: if is_tac_player {
                                            Color::hex("d65d0e").unwrap()
                                        } else {
                                            Color::hex("458488").unwrap()
                                        },
                                    },
                            ));
                        });
                    },
                }
            },
            _ => {}
        }
    }
}

fn update_in_game_ui(
    game_state: Res<GameState>,
    mut game_events: EventReader<GameEvent>,
    mut player_handles: Query<(&PlayerHandle, &mut Text)>,
) {
    for event in game_events.iter() {
        match event {
            GameEvent::PlaceTile { player_id: _, at: _ } => {
                for (handle, mut text) in player_handles.iter_mut() {
                    let is_active_player = game_state.active_player_id == handle.0;
                    let is_tac_player = game_state.players.get(&handle.0).unwrap().piece == store::Tile::Tac;

                    text.sections[0].style.color = if !is_active_player {
                        Color::hex("ebdbb2").unwrap()
                    } else {
                        if is_tac_player {
                            Color::hex("d65d0e").unwrap()
                        } else {
                            Color::hex("458488").unwrap()
                        }
                    }
                }
            },
            _ => {}
        }
    }
}

////////// INPUT SYSTEMS //////////////////
fn input (
    windows: Res<Windows>,
    input: Res<Input<MouseButton>>,
    game_state: Res<GameState>,
    mut hover_dots: Query<(&HoverDot, &mut Sprite)>,
    mut client: ResMut<RenetClient>,
) {
    // We only want to handle inputs once we are ingame
    if game_state.stage != store::Stage::InGame {
        return;
    }

    let window = windows.get_primary().unwrap();
    if let Some(mouse_position) = window.cursor_position() {
        // Determine the index of the tile that the mouse is currently over
        // Calculate Window Size
        let x_offset = (window.width() - BOARD_SIZE)/2.0;
        let y_offset = (window.height() - BOARD_SIZE)/2.0 + BOARD_POS_Y;

        let x_tile: usize = ((mouse_position.x - x_offset) / TILE_SIZE).floor() as usize;
        let y_tile: usize = ((mouse_position.y - y_offset) / TILE_SIZE).floor() as usize;
        let mut tile = x_tile + y_tile * 3;


        // If mouse is outside board do nothing
        if mouse_position.x < x_offset
            || mouse_position.x > x_offset + BOARD_SIZE
                || mouse_position.y < y_offset
                || mouse_position.y > y_offset + BOARD_SIZE
        {
            tile = 10;
        }

        // Toggle hover dots on and off
        for (dot, mut dot_sprite) in hover_dots.iter_mut() {
            if dot.0 == tile {
                dot_sprite.color.set_a(1.0);
            } else {
                dot_sprite.color.set_a(0.0);
            }
        }

        // If left mouse button is pressed, send a place tile event to the server
        if input.just_pressed(MouseButton::Left) && tile != 10 {
            let event = GameEvent::PlaceTile {
                player_id: client.client_id(),
                at: tile,
            };
            client.send_message(0, bincode::serialize(&event).unwrap());
        }
    }
}

