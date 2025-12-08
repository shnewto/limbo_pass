use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioInstance, AudioSource};

#[derive(Resource)]
pub struct ThemeState {
    pub loop_handle: Handle<AudioSource>,
    pub instance: Option<Handle<AudioInstance>>,
    pub is_playing: bool,
}

pub fn load(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let loop_handle = asset_server.load("audio/overworld.ogg");
    let theme_state = ThemeState {
        loop_handle,
        instance: None,
        is_playing: false,
    };

    commands.insert_resource(theme_state);
}

pub fn play(mut audio_state: ResMut<ThemeState>, audio: Res<Audio>) {
    if audio_state.instance.is_none() {
        let instance = audio.play(audio_state.loop_handle.clone()).looped().handle();
        audio_state.instance = Some(instance);
        audio_state.is_playing = true;
        bevy::log::info!("Music started");
    }
}
