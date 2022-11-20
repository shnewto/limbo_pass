use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioControl, AudioSource};

#[derive(Resource)]
pub struct ThemeState {
    pub loop_handle: Handle<AudioSource>,
}

pub fn load(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    let loop_handle = asset_server.load("audio/overworld.ogg");
    let theme_state = ThemeState { loop_handle };

    commands.insert_resource(theme_state);
}

pub fn play(audio: Res<Audio>, audio_state: Res<ThemeState>) {
    audio.play(audio_state.loop_handle.clone()).looped();
}
