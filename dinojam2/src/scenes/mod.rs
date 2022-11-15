pub mod main_menu_scene;
pub mod game_scene;
pub mod test_scene;
pub mod test_tile_scene;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum SceneState {
    TestScene,
}
