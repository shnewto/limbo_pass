use crate::fsm::Fsm;
use bevy::prelude::*;
use bevy_asset_loader::{AssetCollection, AssetLoader};
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

/// This plugin loads all assets using [AssetLoader] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at https://bevy-cheatbook.github.io/features/assets.html
impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        AssetLoader::new(Fsm::Loading)
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<SceneAssets>()
            .continue_to_state(Fsm::MainMenu)
            .build(app);
    }
}


#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "font/square.ttf")]
    pub menu_font_handle: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/overworld.ogg")]
    pub theme_handle: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct SceneAssets {
    #[asset(path = "gltf/limbo_pass.gltf")]
    pub limbo_pass_handle: Handle<Scene>,
}