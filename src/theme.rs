use crate::asset::AudioAssets;
use bevy::{asset::LoadState, prelude::*};

use crate::fsm::Fsm;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(Fsm::Loading).with_system(load))
            .add_system_set(SystemSet::on_exit(Fsm::MainMenu).with_system(play));
    }
}

pub struct ThemeState {
    audio_loaded: bool,
}

pub fn load(mut commands: Commands) {
    let theme_state = ThemeState {
        audio_loaded: false,
    };

    commands.insert_resource(theme_state);
}

pub fn play(
    audio: Res<Audio>,
    mut theme_state: ResMut<ThemeState>,
    audio_assets: ResMut<AudioAssets>,
    asset_server: ResMut<AssetServer>,
) {
    if !theme_state.audio_loaded
        && LoadState::Loaded == asset_server.get_load_state(&audio_assets.theme_handle)
    {
        audio.play(audio_assets.theme_handle.clone());
        theme_state.audio_loaded = true;
    }
}
