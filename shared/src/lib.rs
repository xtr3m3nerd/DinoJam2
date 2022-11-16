use std::fmt;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::collections::HashMap;

pub mod asset_management;
pub mod hex;
pub mod buildings;
pub mod units;
pub mod terrain;

use crate::buildings::*;
use crate::units::*;
use crate::terrain::*;

// Only clients that can provide the same PROTOCOL_ID that the server is using will be able to
// connect. This can be used to make sure players use the most recent version of the client for
// instance.
pub const PROTOCOL_ID: u64 = 1208;

const MAP_WIDTH: usize = 8;
const MAP_HEIGHT: usize = 8;
const MAP_SIZE: usize = MAP_WIDTH * MAP_HEIGHT;

// This just makes it easier to dissern between a player id and any u64
type PlayerId = u64;

/// Struct for board positional related data.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoardTile {
    pub terrain: TerrainKind,
    pub unit: Option<Unit>,
    pub building: Option<Building>,
}

impl Default for BoardTile {
    fn default() -> Self {
        Self {
            terrain: TerrainKind(0),
            unit: None,
            building: None,
        }
    }
}

/// Different factions that a player can play
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Faction {
    Volcano,
    Dinosaur,
}

impl fmt::Display for Faction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Faction::Volcano => write!(f, "Volcano"),
            Faction::Dinosaur => write!(f, "Dinosaur"),
        }
    }
}

/// Struct for storing player related data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub faction: Faction,
    pub gold: u32,
}

/// The different states a game can be in. (not to be confused with the entire "GameState")
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Stage {
    PreGame,
    InGame,
    Ended,
}

/// The reasons why a game could end
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EndGameReason {
    PlayerLeft { player_id: PlayerId },
    PlayerWon { winner: PlayerId },
}

/// An event that progresses the GameState forward
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    BeginGame {
        goes_first: PlayerId,
    },
    EndGame {
        reason: EndGameReason,
    },
    PlayerJoined {
        player_id: PlayerId,
        name: String,
    },
    PlayerDisconnected {
        player_id: PlayerId,
    },
    BuildUnit {
        player_id: PlayerId,
        at: usize,
        unit_kind: UnitKind,
    },
    MoveUnit {
        player_id: PlayerId,
        from: usize,
        to: usize,
    },
    EndTurn {
        player_id: PlayerId,
    },
}

/// A GameState object that is able to keep track of a game
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub stage: Stage,
    #[serde(with = "BigArray")]
    pub board: [BoardTile; MAP_SIZE],
    pub active_player_id: PlayerId,
    pub players: HashMap<PlayerId, Player>,
    pub histroy: Vec<GameEvent>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stage: Stage::PreGame,
            board: [BoardTile::default(); MAP_SIZE],
            active_player_id: 0,
            players: HashMap::new(),
            histroy: Vec::new(),
        }
    }
}

pub struct Descriptors {
    pub units: Units,
    pub terrain: Terrain,
    pub buildings: Buildings,
}

impl GameState {
    /// Determins where an event is valid considering the current GameState
    pub fn validate(
        &self,
        event: &GameEvent,
        buildings: &Buildings,
        units: &Units,
        terrain: &Terrain,
    ) -> bool {
        use GameEvent::*;
        match event {
            BeginGame { goes_first } => {
                // Check that the player supposed to go first exists
                if !self.players.contains_key(goes_first) {
                    return false;
                }

                // Check that the game hasnt started yet.
                if self.stage != Stage::PreGame {
                    return false;
                }
            }
            EndGame { reason } => match reason {
                EndGameReason::PlayerWon { winner: _ } => {
                    //Check that the game has started before someone wins it
                    if self.stage != Stage::InGame {
                        return false;
                    }
                }
                _ => {}
            },
            PlayerJoined { player_id, name: _ } => {
                // Check that there isnt another player with the same id
                if self.players.contains_key(player_id) {
                    return false;
                }
            }
            PlayerDisconnected { player_id } => {
                // Check that player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }
            }
            BuildUnit {
                player_id,
                at,
                unit_kind,
            } => {
                // Check that player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }

                //Check if player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }

                // Check that the tile index is inside the board
                if *at > MAP_SIZE {
                    return false;
                }

                let player = self.players.get(player_id).unwrap();
                let board_tile = self.board[*at];

                // Check that there is a building at the location that is the player's faction
                if let Some(building) = board_tile.building {
                    let building_descriptor = buildings[building.kind].clone();
                    if building_descriptor.faction != player.faction.to_string() {
                        return false;
                    }
                } else {
                    // There is no building to build unit from
                    return false;
                }

                // Check that the player is not trying to place a piece on top of existing peice
                if board_tile.unit != None {
                    return false;
                }

                let unit_descriptor = units[*unit_kind].clone();

                // Check that player could afford to build unit
                if unit_descriptor.cost > player.gold {
                    return false;
                }

                // Check that the faction is the same as the player
                if unit_descriptor.faction != player.faction.to_string() {
                    return false;
                }

            }
            MoveUnit {
                player_id,
                from,
                to,
            } => {
                // Check that player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }

                //Check if player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }

                // Check that the tile index is inside the board
                if *from > MAP_SIZE || *to > MAP_SIZE {
                    return false;
                }

                let player = self.players.get(player_id).unwrap();
                let from_board_tile = self.board[*from];
                let to_board_tile = self.board[*to];

                // Check that it is valid to move to tile
                let terrain_descriptor = terrain[to_board_tile.terrain].clone();
                if terrain_descriptor.wall {
                    return false;
                }

                if let Some(_unit_to_move) = from_board_tile.unit {
                    // TODO Check to see if movement is within range
                    //let from_tilepos = ;
                    //let to_tilepos = ;
                    //let path = get_hex_path(from_tilepos. to_tilepos);

                    //if path.size() > unit_to_move.range_remaining {
                    //}

                    if let Some(unit_to_attack) = to_board_tile.unit {
                        // Check that the player is not trying to place a piece on top of an allied
                        let unit_to_attack_descriptor = units[unit_to_attack.kind].clone();
                        if unit_to_attack_descriptor.faction == player.faction.to_string() {
                            return false;
                        }
                    }
                } else {
                    // No unit to move
                    return false;
                }
            }
            EndTurn { player_id } => {
                // Check that player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }

                //Check if player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }
            }
        }

        // We couldnt find anything wrong so must be good
        true
    }

    /// Consumes and event, modifying the GameState and adding the event to its history.
    /// NOTE: Consume assumes the event to have already been balidated and will accept any event
    /// passed to it
    pub fn consume(
        &mut self,
        valid_event: &GameEvent,
        _buildings: &Buildings,
        units: &Units,
        _terrain: &Terrain,
    ) {
        use GameEvent::*;
        match valid_event {
            BeginGame { goes_first } => {
                self.active_player_id = *goes_first;
                self.stage = Stage::InGame;
            }
            EndGame { reason: _ } => self.stage = Stage::Ended,
            PlayerJoined { player_id, name } => {
                self.players.insert(
                    *player_id,
                    Player {
                        name: name.to_string(),
                        // First player to join get volcano, second get dinos
                        faction: if self.players.len() > 0 {
                            Faction::Dinosaur
                        } else {
                            Faction::Volcano
                        },
                        gold: 0,
                    },
                );
            }
            PlayerDisconnected { player_id } => {
                self.players.remove(player_id);
            }
            BuildUnit {
                player_id,
                at,
                unit_kind,
            } => {
                let x = (at % MAP_WIDTH) as u32;
                let y = (at / MAP_HEIGHT) as u32;
                let mut board_tile = self.board[*at];
                board_tile.unit = Some(Unit::new(
                        (x , y),
                        *unit_kind,
                        units,
                ));

                let unit_descriptor = &units[*unit_kind];
                let mut player = self.players.get_mut(player_id).unwrap();
                player.gold -= unit_descriptor.cost;
            }
            MoveUnit {
                player_id: _,
                from,
                to,
            } => {
                let x = (to % MAP_WIDTH) as u32;
                let y = (to / MAP_HEIGHT) as u32;
                let mut from_unit = self.board[*from].unit.unwrap();
                if let Some(mut to_unit) = self.board[*from].unit {
                    let unit_descriptor = &units[from_unit.kind];
                    to_unit.health -= unit_descriptor.damage;
                    if to_unit.health <= 0 {
                        from_unit.position = (x, y);
                        self.board[*from].unit = None;
                        self.board[*to].unit = Some(from_unit);
                    } else {
                        // TODO Handle case where enemy unit doesnt die
                        // probably move to closest point along path to unit
                    }
                } else {
                    from_unit.position = (x, y);
                    self.board[*from].unit = None;
                    self.board[*to].unit = Some(from_unit);
                }
            }
            EndTurn { player_id } => {
                // Switch which player is the active player
                self.active_player_id = self
                    .players
                    .keys()
                    .find(|id| *id != player_id)
                    .unwrap()
                    .clone();
                }
        }

        self.histroy.push(valid_event.clone());
    }

    /// Determines if someone has won the game
    pub fn determine_winner(&self) -> Option<PlayerId> {
        if self.volcano_has_been_plugged() {
            if let Some((dinosaur_player, _)) = self
                .players
                    .iter()
                    .find(|(_, player)| player.faction == Faction::Dinosaur)
            {
                return Some(*dinosaur_player);
            }
        }

        if self.all_dino_dead() && self.all_dino_villages_destroyed() {
            if let Some((volcano_player, _)) = self
                .players
                    .iter()
                    .find(|(_, player)| player.faction == Faction::Volcano)
            {
                return Some(*volcano_player);
            }
        }

        None
    }

    /// Determines if the volcano has been plugged with boulder
    pub fn volcano_has_been_plugged(&self) -> bool {
        // TODO - Impliment function

        // Volcano has not been plugged
        false
    }

    /// Determines if all dinos on map are dead
    pub fn all_dino_dead(&self) -> bool {
        // TODO - Impliment function

        // Some dinos are still alive
        false
    }

    /// Determines if the volcano has been plugged with boulder
    pub fn all_dino_villages_destroyed(&self) -> bool {
        // TODO - Impliment function

        // Some dinos villages are still standing
        false
    }
    /// Get player faction from player_id
    pub fn get_player_faction(&self, player_id: &PlayerId) -> Faction {
        self.players.get(player_id).unwrap().faction
    }
}

// use std::time::Duration;
//
// use bevy::prelude::*;
// use bevy_rapier3d::prelude::*;
// use bevy_renet::renet::{ChannelConfig, ReliableChannelConfig, RenetConnectionConfig, UnreliableChannelConfig, NETCODE_KEY_BYTES};
// use serde::{Deserialize, Serialize};
//
// pub const PRIVATE_KEY: &[u8; NETCODE_KEY_BYTES] = b"an example very very secret key."; // 32-bytes
// pub const PROTOCOL_ID: u64 = 7;
//
// #[derive(Debug, Component)]
// pub struct Player {
//     pub id: u64,
// }
//
// #[derive(Debug, Default, Clone, Copy, Serialize, Deserialize, Component)]
// pub struct PlayerInput {
//     pub up: bool,
//     pub down: bool,
//     pub left: bool,
//     pub right: bool,
// }
//
// #[derive(Debug, Serialize, Deserialize, Component)]
// pub enum PlayerCommand {
//     BasicAttack { cast_at: Vec3 },
// }
//
// pub enum ClientChannel {
//     Input,
//     Command,
// }
//
// pub enum ServerChannel {
//     ServerMessages,
//     NetworkedEntities,
// }
//
// #[derive(Debug, Serialize, Deserialize, Component)]
// pub enum ServerMessages {
//     PlayerCreate { entity: Entity, id: u64, translation: [f32; 3] },
//     PlayerRemove { id: u64 },
//     SpawnProjectile { entity: Entity, translation: [f32; 3] },
//     DespawnProjectile { entity: Entity },
// }
//
// #[derive(Debug, Serialize, Deserialize, Default)]
// pub struct NetworkedEntities {
//     pub entities: Vec<Entity>,
//     pub translations: Vec<[f32; 3]>,
// }
//
// impl From<ClientChannel> for u8 {
//     fn from(channel_id: ClientChannel) -> Self {
//         match channel_id {
//             ClientChannel::Command => 0,
//             ClientChannel::Input => 1,
//         }
//     }
// }
//
// impl ClientChannel {
//     pub fn channels_config() -> Vec<ChannelConfig> {
//         vec![
//             ReliableChannelConfig {
//                 channel_id: Self::Input.into(),
//                 message_resend_time: Duration::ZERO,
//                 ..Default::default()
//             }
//             .into(),
//             ReliableChannelConfig {
//                 channel_id: Self::Command.into(),
//                 message_resend_time: Duration::ZERO,
//                 ..Default::default()
//             }
//             .into(),
//         ]
//     }
// }
//
// impl From<ServerChannel> for u8 {
//     fn from(channel_id: ServerChannel) -> Self {
//         match channel_id {
//             ServerChannel::NetworkedEntities => 0,
//             ServerChannel::ServerMessages => 1,
//         }
//     }
// }
//
// impl ServerChannel {
//     pub fn channels_config() -> Vec<ChannelConfig> {
//         vec![
//             UnreliableChannelConfig {
//                 channel_id: Self::NetworkedEntities.into(),
//                 sequenced: true, // We don't care about old positions
//                 ..Default::default()
//             }
//             .into(),
//             ReliableChannelConfig {
//                 channel_id: Self::ServerMessages.into(),
//                 message_resend_time: Duration::from_millis(200),
//                 ..Default::default()
//             }
//             .into(),
//         ]
//     }
// }
//
// pub fn client_connection_config() -> RenetConnectionConfig {
//     RenetConnectionConfig {
//         send_channels_config: ClientChannel::channels_config(),
//         receive_channels_config: ServerChannel::channels_config(),
//         ..Default::default()
//     }
// }
//
// pub fn server_connection_config() -> RenetConnectionConfig {
//     RenetConnectionConfig {
//         send_channels_config: ServerChannel::channels_config(),
//         receive_channels_config: ClientChannel::channels_config(),
//         ..Default::default()
//     }
// }
//
// /// set up a simple 3D scene
// pub fn setup_level(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
//     // plane
//     commands
//         .spawn_bundle(PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Box::new(10., 1., 10.))),
//             material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
//             transform: Transform::from_xyz(0.0, -1.0, 0.0),
//             ..Default::default()
//         })
//         .insert(Collider::cuboid(5., 0.5, 5.));
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
// }
//
// #[derive(Debug, Component)]
// pub struct Projectile {
//     pub duration: Timer,
// }
//
// pub fn spawn_fireball(
//     commands: &mut Commands,
//     meshes: &mut ResMut<Assets<Mesh>>,
//     materials: &mut ResMut<Assets<StandardMaterial>>,
//     translation: Vec3,
//     mut direction: Vec3,
// ) -> Entity {
//     if !direction.is_normalized() {
//         direction = Vec3::X;
//     }
//     commands
//         .spawn_bundle(PbrBundle {
//             mesh: meshes.add(Mesh::from(shape::Icosphere {
//                 radius: 0.1,
//                 subdivisions: 5,
//             })),
//             material: materials.add(Color::rgb(1.0, 0.0, 0.0).into()),
//             transform: Transform::from_translation(translation),
//             ..Default::default()
//         })
//         .insert(RigidBody::Dynamic)
//         .insert(LockedAxes::ROTATION_LOCKED | LockedAxes::TRANSLATION_LOCKED_Y)
//         .insert(Collider::ball(0.1))
//         .insert(Velocity::linear(direction * 10.))
//         .insert(ActiveEvents::COLLISION_EVENTS)
//         .insert(Projectile {
//             duration: Timer::from_seconds(1.5, false),
//         })
//         .id()
// }
//
// /// A 3D ray, with an origin and direction. The direction is guaranteed to be normalized.
// #[derive(Debug, PartialEq, Copy, Clone, Default)]
// pub struct Ray3d {
//     pub(crate) origin: Vec3,
//     pub(crate) direction: Vec3,
// }
//
// impl Ray3d {
//     pub fn new(origin: Vec3, direction: Vec3) -> Self {
//         Ray3d { origin, direction }
//     }
//
//     pub fn from_screenspace(windows: &Res<Windows>, camera: &Camera, camera_transform: &GlobalTransform) -> Option<Self> {
//         let window = windows.get_primary().unwrap();
//         let cursor_position = match window.cursor_position() {
//             Some(c) => c,
//             None => return None,
//         };
//
//         let view = camera_transform.compute_matrix();
//         let screen_size = camera.logical_target_size()?;
//         let projection = camera.projection_matrix();
//         let far_ndc = projection.project_point3(Vec3::NEG_Z).z;
//         let near_ndc = projection.project_point3(Vec3::Z).z;
//         let cursor_ndc = (cursor_position / screen_size) * 2.0 - Vec2::ONE;
//         let ndc_to_world: Mat4 = view * projection.inverse();
//         let near = ndc_to_world.project_point3(cursor_ndc.extend(near_ndc));
//         let far = ndc_to_world.project_point3(cursor_ndc.extend(far_ndc));
//         let ray_direction = far - near;
//
//         Some(Ray3d::new(near, ray_direction))
//     }
//
//     pub fn intersect_y_plane(&self, y_offset: f32) -> Option<Vec3> {
//         let plane_normal = Vec3::Y;
//         let plane_origin = Vec3::new(0.0, y_offset, 0.0);
//         let denominator = self.direction.dot(plane_normal);
//         if denominator.abs() > f32::EPSILON {
//             let point_to_point = plane_origin * y_offset - self.origin;
//             let intersect_dist = plane_normal.dot(point_to_point) / denominator;
//             let intersect_position = self.direction * intersect_dist + self.origin;
//             Some(intersect_position)
//         } else {
//             None
//         }
//     }
// }
