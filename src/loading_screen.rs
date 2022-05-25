use crate::{assets::FontAssets, fsm::Fsm};
use bevy::{asset::LoadState, prelude::*};

pub struct LoadingScreenPlugin;

impl Plugin for LoadingScreenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FontAssets::default())
            .insert_resource(FontAssets::default())
            .add_system_set(SystemSet::on_enter(Fsm::LoadingScreen).with_system(load_font))
            .add_system_set(SystemSet::on_update(Fsm::LoadingScreen).with_system(check_font))
            .add_system_set(SystemSet::on_exit(Fsm::LoadingScreen).with_system(spawn))
            .add_system_set(SystemSet::on_enter(Fsm::Running).with_system(despawn_loading_screen));
    }
}

//
// font is a special case because we need it for the loading screen
//

pub fn load_font(asset_server: ResMut<AssetServer>, mut font_assets: ResMut<FontAssets>) {
    font_assets.square_font_handle = asset_server.load("font/square.ttf");
}

fn check_font(
    asset_server: Res<AssetServer>,
    font_assets: Res<FontAssets>,
    mut state: ResMut<State<Fsm>>,
) {
    if let LoadState::Loaded = asset_server.get_load_state(&font_assets.square_font_handle) {
        state.set(Fsm::LoadingAssets).unwrap();
    }
}

#[derive(Component)]
pub struct LoadingScreenCamera;
#[derive(Component)]
pub struct LoadingScreenText;

pub fn spawn(mut commands: Commands, font_assets: Res<FontAssets>) {
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(LoadingScreenCamera);
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Px(256.0), Val::Px(60.0)),
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
                        value: "limbo pass".to_string(),
                        style: TextStyle {
                            font: font_assets.square_font_handle.clone(),
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        })
        .insert(LoadingScreenText);

    commands.insert_resource(LoadingScreenState::Spawned);
}

pub fn despawn_loading_screen(
    mut commands: Commands,
    mut loading_screen_state: ResMut<LoadingScreenState>,
    loading_screen_text_query: Query<Entity, With<LoadingScreenText>>,
) {
    let text = loading_screen_text_query.single();
    commands.entity(text).despawn_recursive();

    *loading_screen_state = LoadingScreenState::Despawned;
}

#[derive(Component)]
pub enum LoadingScreenState {
    Spawned,
    Despawned,
}
