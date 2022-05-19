use crate::asset::{AudioAssets, FontAssets};
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

pub struct ThemeState {
    pub audio_loaded: bool,
    pub volume: f32,
    pub muted: bool,
}

pub fn load(mut commands: Commands) {
    let theme_state = ThemeState {
        audio_loaded: false,
        volume: 1.3,
        muted: false,
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
        audio.set_volume(theme_state.volume);
        audio.play_looped(audio_assets.theme_handle.clone());
        theme_state.audio_loaded = true;
    }
}

type ButtonInteraction<'a> = (&'a Interaction, &'a mut UiColor);

pub fn mute_interactions(
    audio: Res<Audio>,
    mut theme_state: ResMut<ThemeState>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<MuteButton>)>,
) {
    for (interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                if theme_state.muted {
                    audio.set_volume(theme_state.volume);
                } else {
                    audio.set_volume(0.0);
                }
                theme_state.muted = !theme_state.muted;
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
                size: Size::new(Val::Px(60.0), Val::Px(25.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                position: Rect {
                    left: Val::Px(500.0),
                    bottom: Val::Px(300.0),
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
                        value: "mute".to_string(),
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
