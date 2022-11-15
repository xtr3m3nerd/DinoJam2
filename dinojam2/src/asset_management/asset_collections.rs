use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

// #[derive(AssetCollection)]
// pub struct CutsceneAssets {
//     #[asset(key = "cutscenes.meta", collection(typed))]
//     // pub meta: Vec<Handle<CutsceneMetaAsset>>,
//     #[asset(key = "cutscenes.scenes", collection(typed))]
//     pub scene: Vec<Handle<DynamicScene>>,
// }

#[derive(AssetCollection)]
pub struct MapAssets {
    #[asset(key = "image.select")]
    pub select: Handle<Image>,
    #[asset(key = "image.terrain")]
    pub terrain: Handle<Image>,
    // #[asset(key = "image.units")]
    // pub units: Handle<TextureAtlas>,
}

#[derive(AssetCollection)]
pub struct UiAssets {
    #[asset(key = "font.regular")]
    pub font_regular: Handle<Font>,
    #[asset(key = "font.bold")]
    pub font_bold: Handle<Font>,
    #[asset(key = "font.light")]
    pub font_light: Handle<Font>,
}

// #[derive(AssetCollection)]
// pub struct UiScenes {
//     #[asset(key = "scene.main_menu")]
//     pub main_menu: Handle<DynamicScene>,
//     #[asset(key = "scene.dialogue_box")]
//     pub dialogue_box: Handle<DynamicScene>,
// }

// #[derive(AssetCollection)]
// pub struct AudioAssets {
//     #[asset(key = "sound.music.game")]
//     pub music_game: Handle<AudioSource>,
//     #[asset(key = "sound.music.menu")]
//     pub music_menu: Handle<AudioSource>,
//     #[asset(key = "sound.music.cutscene")]
//     pub music_cutscene: Handle<AudioSource>,
//     #[asset(key = "sound.squish_1")]
//     pub squish_1: Handle<AudioSource>,
//     #[asset(key = "sound.squish_2")]
//     pub squish_2: Handle<AudioSource>,
//     #[asset(key = "sound.squish_3")]
//     pub squish_3: Handle<AudioSource>,
//     #[asset(key = "sound.ui_1")]
//     pub ui_1: Handle<AudioSource>,
//     #[asset(key = "sound.ui_2")]
//     pub ui_2: Handle<AudioSource>,
//     #[asset(key = "sound.ui_3")]
//     pub ui_3: Handle<AudioSource>,
// }
