use bevy::{prelude::*, window::PresentMode};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};
use crate::setup::AppState;

mod form;
mod scenes;
mod setup;
mod theme;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(form::Movements::default())
        .add_state::<AppState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "limbo pass".to_string(),
                present_mode: PresentMode::Fifo,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_systems((
            setup::camera.in_schedule(OnEnter(AppState::Loading)),
            setup::lighting.in_schedule(OnEnter(AppState::Loading)),
            setup::physics.in_schedule(OnEnter(AppState::Loading)),
            scenes::load.in_schedule(OnEnter(AppState::Loading)),
            theme::load.in_schedule(OnEnter(AppState::Loading)),
            setup::check_loaded.run_if(in_state(AppState::Loading)),
            scenes::spawn.in_schedule(OnExit(AppState::Loading)),
            theme::play.in_schedule(OnExit(AppState::Loading)),
            form::get_movement.run_if(in_state(AppState::Running)),
            form::apply_movement.after(form::get_movement).run_if(in_state(AppState::Running)),
            form::wrap_movement.after(form::apply_movement).run_if(in_state(AppState::Running)),

        ))
        .run();
}
