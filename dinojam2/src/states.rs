use bevy::reflect::{FromReflect, Reflect};

#[derive(
    Clone, Copy, Debug, Eq, Hash, PartialEq, Default, Reflect, FromReflect, serde::Deserialize,
)]
pub enum AppState {
    #[default]
    AssetsLoading,
    MainMenu,
    InGame,
    // PlayCutscene,
    // // dev tools / editors
    // EditorCutscene,
    // EditorLevelMap,
    // EditorEzScene,
}
