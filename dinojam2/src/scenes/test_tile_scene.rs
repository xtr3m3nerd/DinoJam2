use bevy::{
    prelude::*,
    utils::HashMap,
};
use bevy_ecs_tilemap::prelude::*;
//use bevy_ecs_tilemap::helpers::hex_grid::neighbors::{HexDirection, HexNeighbors};
use iyes_loopless::prelude::*;
use crate::states::AppState;
use crate::asset_management::{
    asset_collections::MapAssets,
    terrain_descriptors::{Terrain, TerrainKind},
};

pub struct TestTileScenePlugin;

impl Plugin for TestTileScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(AppState::InGame, init_map);
        //app.add_enter_system(AppState::InGame, spawn_assets);

        // app.add_system_set(
        //     ConditionSet::new()
        //     .run_in_state(AppState::InGame)
        //     .with_system(spawn_tile_labels)
        //     .with_system(update_cursor_pos)
        //     .with_system(hover_highlight_tile_label)
        //     .with_system(highlight_neighbor_label.after(hover_highlight_tile_label))
        //     .into()
        // );
    }
}

// TODO - Move everything below to its own contained module for map building.
#[derive(Hash, Eq, Clone, Component, Copy, PartialEq)]
pub enum Layer {
    Select,
    Movement,
    Terrain,
}

#[derive(Deref)]
pub struct LayerToMap(pub HashMap<Layer, Entity>);

#[derive(Component)]
pub struct SelectTile;

// Should follow the order of `image/tile/select.png`. Converts to `TileTexture` with `as u32`.
#[derive(Default)]
pub enum Select {
    Inactive,
    #[default]
    Active,
}

impl From<Select> for TileTexture {
    fn from(select: Select) -> Self {
        Self(select as u32)
    }
}

#[derive(Component)]
pub struct MovementTile;

#[derive(Component)]
pub struct TerrainTile;

// In pixels
pub const TILE_SIZE_X: u32 = 48;
pub const TILE_SIZE_Y: u32 = 54;
// Characters will be at `1.`
const SELECT_LAYER_Z: f32 = 2.;
const MOVEMENT_LAYER_Z: f32 = 1.;
const TERRAIN_LAYER_Z: f32 = 0.;
// For some reason, this value has to be smaller than expected
const SELECT_LAYER_ALPHA: f32 = 0.4;
const SIZE_X: u32 = 10;
const SIZE_Y: u32 = 10;

// Based on https://github.com/StarArawn/bevy_ecs_tilemap/blob/main/examples/basic.rs
pub fn init_map(
    mut commands: Commands,
    assets: Res<MapAssets>,
    terrain: Res<Terrain>,
) {
    // Some common data between the layers
    let map_size: TilemapSize =
        UVec2::new(SIZE_X, SIZE_Y).into();
    let tile_size = Vec2::new(TILE_SIZE_X as f32, TILE_SIZE_Y as f32);
    // No `From<Vec2>` T_T
    let grid_size = TilemapGridSize {
        x: tile_size.x,
        y: tile_size.y,
    };
    let tile_size = TilemapTileSize {
        x: tile_size.x,
        y: tile_size.y,
    };

    let mut layer_to_map = LayerToMap(HashMap::new());
    for layer in [Layer::Select, Layer::Movement, Layer::Terrain] {
        let map = commands.spawn().id();
        let mut tile_storage = TileStorage::empty(map_size);

        // Can't use `fill_tilemap` bc we use a color (for select transparency)
        for x in 0..map_size.x {
            for y in 0..map_size.y {
                let tile_pos = UVec2::new(x, y).into();

                let terrain_kind = TerrainKind(
                    match x < 1 || x >= (map_size.x - 1) || y < 1 || y >= (map_size.y - 1) {
                        false => 0,
                        true => 1,
                    },
                );

                let mut tile = commands.spawn_bundle(TileBundle {
                    position: tile_pos,
                    tilemap_id: TilemapId(map),
                    // TEMP We'll want to load it from some data eventually
                    texture: match layer {
                        // Movement is like the select layer but green
                        // They should be separate tho, so you can have both tiles at the same spot
                        Layer::Select | Layer::Movement => Select::Inactive.into(),
                        Layer::Terrain => TileTexture(terrain[terrain_kind].sprite_idx as u32),
                    },
                    color: TileColor(match layer {
                        Layer::Select => Vec3::ONE.extend(SELECT_LAYER_ALPHA).into(),
                        Layer::Movement => Color::rgba(0.5, 1., 0.5, SELECT_LAYER_ALPHA),
                        _ => Color::WHITE,
                    }),
                    ..default()
                });

                match layer {
                    Layer::Select => {
                        tile.insert(SelectTile);
                    }
                    Layer::Movement => {
                        tile.insert(MovementTile);
                    }
                    Layer::Terrain => {
                        tile.insert(TerrainTile);
                    }
                }
                tile.insert(terrain_kind);

                // Ime it's really messy for empty tiles to not have entities
                tile_storage.set(&tile_pos, tile.id());
            }
        }

        commands.entity(map).insert_bundle(TilemapBundle {
            grid_size,
            size: map_size,
            storage: tile_storage,
            texture: TilemapTexture::Single(match layer {
                Layer::Select | Layer::Movement => assets.select.clone(),
                Layer::Terrain => assets.terrain.clone(),
            }),
            tile_size,
            transform: Transform::from_translation(Vec2::ZERO.extend(match layer {
                Layer::Select => SELECT_LAYER_Z,
                Layer::Movement => MOVEMENT_LAYER_Z,
                Layer::Terrain => TERRAIN_LAYER_Z,
            })),
            map_type: TilemapType::Hexagon(HexCoordSystem::RowOdd),
            ..default()
        });
        layer_to_map.0.insert(layer, map);
    }
    commands.insert_resource(layer_to_map);
}

// #[derive(Component)]
// struct TileLabel;
//
// // Generates tile position labels of the form: `(tile_pos.x, tile_pos.y)`
// fn spawn_tile_labels(
//     mut commands: Commands,
//     tilemap_q: Query<(&Transform, &TilemapType, &TilemapGridSize, &TileStorage)>,
//     tile_q: Query<&mut TilePos>,
//     font_handle: Res<Handle<Font>>,
// ) {
//     let text_style = TextStyle {
//         font: font_handle.clone(),
//         font_size: 20.0,
//         color: Color::BLACK,
//     };
//     let text_alignment = TextAlignment::CENTER;
//     for (map_transform, map_type, grid_size, tilemap_storage) in tilemap_q.iter() {
//         for tile_entity in tilemap_storage.iter().flatten() {
//             let tile_pos = tile_q.get(*tile_entity).unwrap();
//             let tile_center = tile_pos.center_in_world(grid_size, map_type).extend(1.0);
//             let transform = *map_transform * Transform::from_translation(tile_center);
//             commands
//                 .entity(*tile_entity)
//                 .insert_bundle(Text2dBundle {
//                     text: Text::from_section(
//                               format!("{}, {}", tile_pos.x, tile_pos.y),
//                               text_style.clone(),
//                           )
//                         .with_alignment(text_alignment),
//                         transform,
//                         ..default()
//                 })
//             .insert(TileLabel);
//             }
//     }
// }
//
//
// #[derive(Component)]
// struct Hovered;
//
// // Converts the cursor position into a world position, taking into account any transforms applied
// // the camera.
// pub fn cursor_pos_in_world(
//     windows: &Windows,
//     cursor_pos: Vec2,
//     cam_t: &Transform,
//     cam: &Camera,
// ) -> Vec3 {
//     let window = windows.primary();
//
//     let window_size = Vec2::new(window.width(), window.height());
//
//     // Convert screen position [0..resolution] to ndc [-1..1]
//     // (ndc = normalized device coordinates)
//     let ndc_to_world = cam_t.compute_matrix() * cam.projection_matrix().inverse();
//     let ndc = (cursor_pos / window_size) * 2.0 - Vec2::ONE;
//     ndc_to_world.project_point3(ndc.extend(0.0))
// }
//
// #[derive(Default)]
// pub struct CursorPos(Vec3);
//
// // We need to keep the cursor position updated based on any `CursorMoved` events.
// pub fn update_cursor_pos(
//     windows: Res<Windows>,
//     camera_q: Query<(&Transform, &Camera)>,
//     mut cursor_moved_events: EventReader<CursorMoved>,
//     mut cursor_pos: ResMut<CursorPos>,
// ) {
//     for cursor_moved in cursor_moved_events.iter() {
//         // To get the mouse's world position, we have to transform its window position by
//         // any transforms on the camera. This is done by projecting the cursor position into
//         // camera space (world space).
//         for (cam_t, cam) in camera_q.iter() {
//             *cursor_pos = CursorPos(cursor_pos_in_world(
//                     &windows,
//                     cursor_moved.position,
//                     cam_t,
//                     cam,
//             ));
//         }
//     }
// }
//
// // This is where we check which tile the cursor is hovered over.
// fn hover_highlight_tile_label(
//     mut commands: Commands,
//     cursor_pos: Res<CursorPos>,
//     tilemap_q: Query<(
//         &TilemapSize,
//         &TilemapGridSize,
//         &TilemapType,
//         &TileStorage,
//         &Transform,
//     )>,
//     highlighted_tiles_q: Query<Entity, With<Hovered>>,
//     mut tile_label_q: Query<&mut Text, With<TileLabel>>,
// ) {
//     // Un-highlight any previously highlighted tile labels.
//     for highlighted_tile_entity in highlighted_tiles_q.iter() {
//         if let Ok(mut tile_text) = tile_label_q.get_mut(highlighted_tile_entity) {
//             for mut section in tile_text.sections.iter_mut() {
//                 section.style.color = Color::BLACK;
//             }
//             commands.entity(highlighted_tile_entity).remove::<Hovered>();
//         }
//     }
//
//     for (map_size, grid_size, map_type, tile_storage, map_transform) in tilemap_q.iter() {
//         // Grab the cursor position from the `Res<CursorPos>`
//         let cursor_pos: Vec3 = cursor_pos.0;
//         // We need to make sure that the cursor's world position is correct relative to the map
//         // due to any map transformation.
//         let cursor_in_map_pos: Vec2 = {
//             // Extend the cursor_pos vec3 by 1.0
//             let cursor_pos = Vec4::from((cursor_pos, 1.0));
//             let cursor_in_map_pos = map_transform.compute_matrix().inverse() * cursor_pos;
//             Vec2::new(cursor_in_map_pos.x, cursor_in_map_pos.y)
//         };
//         // Once we have a world position we can transform it into a possible tile position.
//         if let Some(tile_pos) =
//             TilePos::from_world_pos(&cursor_in_map_pos, map_size, grid_size, map_type)
//         {
//             // Highlight the relevant tile's label
//             if let Some(tile_entity) = tile_storage.get(&tile_pos) {
//                 if let Ok(mut tile_text) = tile_label_q.get_mut(tile_entity) {
//                     for mut section in tile_text.sections.iter_mut() {
//                         section.style.color = Color::RED;
//                     }
//                     commands.entity(tile_entity).insert(Hovered);
//                 }
//             }
//         }
//     }
// }
//
// #[derive(Component)]
// struct NeighborHighlight;
//
// // Swaps the map type, when user presses SPACE
// #[allow(clippy::too_many_arguments)]
// fn highlight_neighbor_label(
//     mut commands: Commands,
//     tilemap_query: Query<(&TilemapType, &TilemapSize, &TileStorage)>,
//     keyboard_input: Res<Input<KeyCode>>,
//     highlighted_tiles_q: Query<Entity, With<NeighborHighlight>>,
//     hovered_tiles_q: Query<&TilePos, With<Hovered>>,
//     mut tile_label_q: Query<&mut Text, With<TileLabel>>,
// ) {
//     // Un-highlight any previously highlighted tile labels.
//     for highlighted_tile_entity in highlighted_tiles_q.iter() {
//         if let Ok(mut tile_text) = tile_label_q.get_mut(highlighted_tile_entity) {
//             for mut section in tile_text.sections.iter_mut() {
//                 section.style.color = Color::BLACK;
//             }
//             commands
//                 .entity(highlighted_tile_entity)
//                 .remove::<NeighborHighlight>();
//         }
//     }
//
//     for (map_type, map_size, tile_storage) in tilemap_query.iter() {
//         let hex_coord_sys = if let TilemapType::Hexagon(hex_coord_sys) = map_type {
//             hex_coord_sys
//         } else {
//             continue;
//         };
//
//         for hovered_tile_pos in hovered_tiles_q.iter() {
//             let neighboring_positions =
//                 HexNeighbors::get_neighboring_positions(hovered_tile_pos, map_size, hex_coord_sys);
//
//             for neighbor_pos in neighboring_positions.iter() {
//                 // We want to ensure that the tile position lies within the tile map, so we do a
//                 // `checked_get`.
//                 if let Some(tile_entity) = tile_storage.checked_get(neighbor_pos) {
//                     if let Ok(mut tile_text) = tile_label_q.get_mut(tile_entity) {
//                         for mut section in tile_text.sections.iter_mut() {
//                             section.style.color = Color::BLUE;
//                         }
//                         commands.entity(tile_entity).insert(NeighborHighlight);
//                     }
//                 }
//             }
//
//             let selected_hex_direction = if keyboard_input.pressed(KeyCode::Key0) {
//                 Some(HexDirection::Zero)
//             } else if keyboard_input.pressed(KeyCode::Key1) {
//                 Some(HexDirection::One)
//             } else if keyboard_input.pressed(KeyCode::Key2) {
//                 Some(HexDirection::Two)
//             } else if keyboard_input.pressed(KeyCode::Key3) {
//                 Some(HexDirection::Three)
//             } else if keyboard_input.pressed(KeyCode::Key4) {
//                 Some(HexDirection::Four)
//             } else if keyboard_input.pressed(KeyCode::Key5) {
//                 Some(HexDirection::Five)
//             } else {
//                 None
//             };
//
//             if let Some(hex_direction) = selected_hex_direction {
//                 let tile_pos = match map_type {
//                     TilemapType::Hexagon(hex_coord_sys) => {
//                         // Get the neighbor in a particular direction.
//                         // This function does not check to see if the calculated neighbor lies
//                         // within the tile map.
//                         hex_direction.offset(hovered_tile_pos, *hex_coord_sys)
//                     }
//                     _ => unreachable!(),
//                 };
//
//                 // We want to ensure that the tile position lies within the tile map, so we do a
//                 // `checked_get`.
//                 if let Some(tile_entity) = tile_storage.checked_get(&tile_pos) {
//                     if let Ok(mut tile_text) = tile_label_q.get_mut(tile_entity) {
//                         for mut section in tile_text.sections.iter_mut() {
//                             section.style.color = Color::GREEN;
//                         }
//                         commands.entity(tile_entity).insert(NeighborHighlight);
//                     }
//                 }
//             }
//         }
//     }
// }
//
// // Spawns different tiles that are used for this example.
// fn spawn_assets(mut commands: Commands, asset_server: Res<AssetServer>) {
//     let font: Handle<Font> = asset_server.load("fonts/FiraSans-Bold.ttf");
//
//     commands.insert_resource(font);
// }
