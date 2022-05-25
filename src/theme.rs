use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{assets::AudioAssets, fsm::Fsm, setup::MuteButton};

pub struct ThemePlugin;

#[derive(Default, Debug, Component)]
pub struct ThemeState {
    pub is_muted: bool,
    pub playing_label: String,
    pub muted_label: String,
    pub started: bool,
}

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ThemeState {
            is_muted: false,
            playing_label: "mute theme".into(),
            muted_label: "play theme".into(),
            started: false,
        })
        .add_system_set(SystemSet::on_exit(Fsm::Setup).with_system(start))
        .add_system_set(SystemSet::on_update(Fsm::Running).with_system(pause));
    }
}

pub fn start(audio: Res<Audio>, audio_assets: ResMut<AudioAssets>) {
    audio.set_volume(1.7);
    audio.play_looped(audio_assets.theme_handle.clone());
}

type ButtonInteraction<'a> = (&'a Interaction, &'a mut UiColor, &'a Children);

fn pause(
    audio: Res<Audio>,
    mut theme_state: ResMut<ThemeState>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<MuteButton>)>,
    mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color, children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                if theme_state.is_muted {
                    audio.resume();
                    text.sections[0].value = theme_state.playing_label.clone();
                }

                if !theme_state.is_muted {
                    audio.pause();
                    text.sections[0].value = theme_state.muted_label.clone();
                }
                theme_state.is_muted = !theme_state.is_muted;
            }

            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                let clear_color_hex_string = "0a0e17";
                *color = Color::hex(clear_color_hex_string)
                    .unwrap_or_else(|_| {
                        panic!("couldn't make hex color from {}", clear_color_hex_string)
                    })
                    .into();
            }
        }
    }
}
