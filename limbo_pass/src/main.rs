use crate::setup::AppState;
use bevy::{prelude::*, window::PresentMode};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

mod form;
mod scenes;
mod setup;
mod theme;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(form::Movements::default())
        .init_state::<AppState>()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "limbo pass".to_string(),
                    present_mode: PresentMode::Fifo,
                    ..default()
                }),
                ..default()
            }),
            LookTransformPlugin,
            OrbitCameraPlugin::default(),
            AudioPlugin,
            RapierPhysicsPlugin::<NoUserData>::default(),
            RapierDebugRenderPlugin::default(),
        ))
        .add_systems(
            OnEnter(AppState::Loading),
            (
                setup::camera,
                setup::lighting,
                setup::physics,
                scenes::load,
                theme::load,
            ),
        )
        .add_systems(OnExit(AppState::Loading), (scenes::spawn, theme::play))
        .add_systems(
            Update,
            (
                setup::check_loaded.run_if(in_state(AppState::Loading)),
                form::get_movement.run_if(in_state(AppState::Running)),
                form::apply_movement
                    .after(form::get_movement)
                    .run_if(in_state(AppState::Loading)),
                form::wrap_movement
                    .after(form::apply_movement)
                    .run_if(in_state(AppState::Running)),
            ),
        )
        .run();
}
