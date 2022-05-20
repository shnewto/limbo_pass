use crate::fsm::Fsm;
use bevy::{asset::LoadState, prelude::*};
use bevy_kira_audio::AudioSource;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AudioAssets::default())
            .insert_resource(SceneAssets::default())
            .add_system_set(SystemSet::on_enter(Fsm::LoadingAssets).with_system(load))
            .add_system_set(SystemSet::on_update(Fsm::LoadingAssets).with_system(check));
    }
}

#[derive(Component, Debug, Default)]
pub struct FontAssets {
    pub square_font_handle: Handle<Font>,
}

#[derive(Component, Debug, Default)]
pub struct AudioAssets {
    pub theme_handle: Handle<AudioSource>,
}

#[derive(Component, Debug, Default)]
pub struct SceneAssets {
    pub limbo_pass_handle: Handle<Scene>,
}

fn load(
    asset_server: ResMut<AssetServer>,
    mut audio_assets: ResMut<AudioAssets>,
    mut scene_assets: ResMut<SceneAssets>,
) {
    audio_assets.theme_handle = asset_server.load("audio/overworld.ogg");
    scene_assets.limbo_pass_handle = asset_server.load("gltf/limbo_pass.gltf");
}

fn check(
    mut state: ResMut<State<Fsm>>,
    asset_server: Res<AssetServer>,
    font_assets: Res<FontAssets>,
    audio_assets: Res<AudioAssets>,
    scene_assets: Res<SceneAssets>,
) {
    if let (LoadState::Loaded, LoadState::Loaded, LoadState::Loaded) = (
        asset_server.get_load_state(&font_assets.square_font_handle),
        asset_server.get_load_state(&audio_assets.theme_handle),
        asset_server.get_load_state(&scene_assets.limbo_pass_handle),
    ) {
        state.set(Fsm::Setup).unwrap();
    }
}
