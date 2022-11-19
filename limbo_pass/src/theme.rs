use bevy::{asset::LoadState, prelude::*};
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

#[derive(Resource)]
pub struct ThemeState {
    audio_loaded: bool,
    loop_handle: Handle<AudioSource>,
}

pub fn load(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let loop_handle = asset_server.load("audio/overworld.ogg");
    let theme_state = ThemeState {
        audio_loaded: false,
        loop_handle,
    };

    commands.insert_resource(theme_state);
}

pub fn play(
    audio: Res<Audio>,
    mut audio_state: ResMut<ThemeState>,
    asset_server: ResMut<AssetServer>,
) {
    if !audio_state.audio_loaded
        && LoadState::Loaded == asset_server.get_load_state(&audio_state.loop_handle)
    {
        audio.play(audio_state.loop_handle.clone()).looped();
        audio_state.audio_loaded = true;
    }
}
