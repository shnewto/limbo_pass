use crate::asset::FontAssets;
use crate::fsm::Fsm;
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(Fsm::MainMenu).with_system(main_menu_init))
            .add_system_set(SystemSet::on_update(Fsm::MainMenu).with_system(menu_interactions));
    }
}

type ButtonInteraction<'a> = (Entity, &'a Interaction, &'a mut UiColor);

pub fn menu_interactions(
    mut commands: Commands,
    mut state: ResMut<State<Fsm>>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<Button>)>,
) {
    for (button, interaction, mut color) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                commands.entity(button).despawn_recursive();
                state.set(Fsm::Running).unwrap();
            }
            Interaction::Hovered => {
                *color = Color::rgb(0.25, 0.25, 0.25).into();
            }
            Interaction::None => {
                *color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn main_menu_init(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands
        .spawn_bundle(ButtonBundle {
            style: Style {
                size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                margin: Rect::all(Val::Auto),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..Default::default()
            },
            color: Color::rgb(0.15, 0.15, 0.15).into(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "play".to_string(),
                        style: TextStyle {
                            font: font_assets.button_font_handle.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
}
