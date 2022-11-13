use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::collections::HashMap;

const MAP_WIDTH: usize = 8;
const MAP_HEIGHT: usize = 8;
const MAP_SIZE: usize = MAP_WIDTH * MAP_HEIGHT;

// This just makes it easier to dissern between a player id and any u64
type PlayerId = u64;

/// Struct for board positional related data.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct BoardTile {
    pub terrain: TerrainTile,
    pub unit: Option<Unit>,
}

impl Default for BoardTile {
    fn default() -> Self {
        Self {
            terrain: TerrainTile::Empty,
            unit: None,
        }
    }
}

/// Possible states that a terrain position in the board can be in
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum TerrainTile {
    Empty,
    Land,
    Water,
    Lava,
}

/// Different factions that a player can play
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Faction {
    Volcano,
    Dinosaur,
}

/// Struct for storing player related data.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Player {
    pub name: String,
    pub faction: Faction,
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

/// Different unit types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum UnitType {
    Lava,
    LavaGolem,
    VolcanoRocks,
    DinoFighter,
    DinoRanged,
    DinoScout,
}

/// Different unit types
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Unit {
    pub unit_type: UnitType,
    pub max_hp: u32,
    pub cur_hp: u32,
    pub move_range: u32,
    pub range_remaining: u32,
    pub attack_range: u32,
    pub damage: u32,
}

/// An event that progresses the GameState forward
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GameEvent {
    BeginGame { goes_first: PlayerId },
    EndGame { reason: EndGameReason },
    PlayerJoined { player_id: PlayerId, name: String },
    PlayerDisconnected { player_id: PlayerId },
    BuildUnit { player_id: PlayerId, at: usize, unit: Unit },
    MoveUnit { player_id: PlayerId, at: usize, unit: Unit },
    EndTurn { player_id: PlayerId },
}

/// Possible states that a position in the board can be in
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Tile {
    Empty,
    Tic,
    Tac,
}

/// A GameState object that is able to keep track of a game
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameState {
    pub stage: Stage,
    #[serde(with = "BigArray")]
    pub board: [ BoardTile; MAP_SIZE ],
    pub active_player_id: PlayerId,
    pub players: HashMap<PlayerId, Player>,
    pub histroy: Vec<GameEvent>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            stage: Stage::PreGame,
            board: [ BoardTile::default(); MAP_SIZE ],
            active_player_id: 0,
            players: HashMap::new(),
            histroy: Vec::new(),
        }
    }
}

impl GameState {
    /// Determins where an event is valid considering the current GameState
    pub fn validate(&self, event: &GameEvent) -> bool {
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
            },
            BuildUnit { player_id, at, unit: _ } => {
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


                // Check that the player is not trying to place a piece on top of existing peice
                if self.board[*at].terrain != TerrainTile::Empty {
                    return false;
                }

                // TODO validate that the placement was a valid move for the unit
                // TODO validate that player could afford to build unit
            },
            MoveUnit { player_id, at, unit: _ } => {
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

                // Check that the player is not trying to place a piece on top of existing peice
                if self.board[*at].terrain != TerrainTile::Empty {
                    return false;
                }

                // TODO validate that the move was a valid move for the unit
                // TODO validate that unit hasnt already taken action for this turn
            },
            EndTurn { player_id } => {
                // Check that player exists
                if !self.players.contains_key(player_id) {
                    return false;
                }

                //Check if player is currently the one making their move
                if self.active_player_id != *player_id {
                    return false;
                }
            },
        }

        // We couldnt find anything wrong so must be good
        true
    }

    /// Consumes and event, modifying the GameState and adding the event to its history.
    /// NOTE: Consume assumes the event to have already been balidated and will accept any event
    /// passed to it
    pub fn consume(&mut self, valid_event: &GameEvent) {
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
                    },
                );
            }
            PlayerDisconnected { player_id } => {
                self.players.remove(player_id);
            }
            BuildUnit { player_id: _, at: _, unit: _ } => {
                // TODO impliment actual build logic
                //let piece = self.get_player_faction(player_id);
            },
            MoveUnit { player_id: _, at: _, unit: _ } => {
                // TODO impliment actual move logic
                // let piece = self.get_player_faction(player_id);
            },
            EndTurn { player_id } => {
                // Switch which player is the active player
                self.active_player_id = self
                    .players
                    .keys()
                    .find(|id| *id != player_id)
                    .unwrap()
                    .clone();
                },
        }

        self.histroy.push(valid_event.clone());
    }

    /// Determines if someone has won the game
    pub fn determine_winner(&self) -> Option<PlayerId> {

        if self.volcano_has_been_plugged() {
            if let Some((dinosaur_player, _)) = self.players
                .iter()
                    .find(|(_, player)| player.faction == Faction::Dinosaur)
            {
                return Some(*dinosaur_player);
            }
        }

        if self.all_dino_dead() && self.all_dino_villages_destroyed()  {
            if let Some((volcano_player, _)) = self.players
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
