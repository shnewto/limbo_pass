use crate::{asset::{AudioAssets, FontAssets}, scenes::SceneState};
use bevy::{asset::LoadState, prelude::*};
use bevy_kira_audio::Audio;

use crate::fsm::Fsm;

pub struct ThemePlugin;

impl Plugin for ThemePlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(Fsm::Loading).with_system(load))
            .add_system_set(
                SystemSet::on_exit(Fsm::MainMenu)
                    .with_system(play)
                    .with_system(mute_button_init),
            )
            .add_system_set(SystemSet::on_update(Fsm::Running).with_system(mute_interactions));
    }
}

pub enum AudioLoadedState {
    NotLoaded,
    Loaded,
    Played(f32),
}

pub struct ThemeState {
    pub audio_state: AudioLoadedState,
    pub is_muted: bool,
    pub playing_label: String,
    pub muted_label: String,
}

pub fn load(mut commands: Commands) {
    let theme_state = ThemeState {
        audio_state: AudioLoadedState::NotLoaded,
        is_muted: false,
        playing_label: "mute theme".into(),
        muted_label: "play theme".into(),
    };

    commands.insert_resource(theme_state);
}

pub fn play(
    mut theme_state: ResMut<ThemeState>,
    scene_state: ResMut<SceneState>,
    audio_assets: ResMut<AudioAssets>,
    asset_server: ResMut<AssetServer>,
) {
    if let (AudioLoadedState::NotLoaded, true, true) = (
        &theme_state.audio_state,
        &scene_state.is_loaded,
        LoadState::Loaded == asset_server.get_load_state(&audio_assets.theme_handle),
    ) {
        theme_state.audio_state = AudioLoadedState::Loaded;
    }
}

type ButtonInteraction<'a> = (&'a Interaction, &'a mut UiColor, &'a Children);

pub fn mute_interactions(
    audio: Res<Audio>,
    mut theme_state: ResMut<ThemeState>,
    audio_assets: ResMut<AudioAssets>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<MuteButton>)>,
    mut text_query: Query<&mut Text>,
) {

    if let (AudioLoadedState::Loaded, false) = (&theme_state.audio_state, theme_state.is_muted) {
        let volume = 1.3;
        audio.set_volume(volume);
        audio.play_looped(audio_assets.theme_handle.clone());
        theme_state.audio_state = AudioLoadedState::Played(volume);
        return;
    }

    for (interaction, mut color, mut children) in interaction_query.iter_mut() {
        let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Clicked => {
                if let (AudioLoadedState::Played(_), true) = (&theme_state.audio_state, theme_state.is_muted) {
                    audio.resume();
                    text.sections[0].value = theme_state.playing_label.clone();
                }

                if let (AudioLoadedState::Played(_), false) = (&theme_state.audio_state, theme_state.is_muted) {
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

#[derive(Component)]
pub struct MuteButton;

pub fn mute_button_init(mut commands: Commands, font_assets: Res<FontAssets>) {
    let clear_color_hex_string = "0a0e17";
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: Rect {
                    left: Val::Px(450.0),
                    bottom: Val::Px(250.0),
                    ..default()
                },
                ..Default::default()
            },
            color: Color::hex(clear_color_hex_string)
                .unwrap_or_else(|_| {
                    panic!("couldn't make hex color from {}", clear_color_hex_string)
                })
                .into(),
            ..Default::default()
        })
        .insert(MuteButton)
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "mute theme".to_string(),
                        style: TextStyle {
                            font: font_assets.button_font_handle.clone(),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}
