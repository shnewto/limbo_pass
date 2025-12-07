use bevy::{light::PointLightShadowMap, prelude::*, window::PresentMode};
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
        .insert_resource(form::Movements::default())
        .insert_resource(PointLightShadowMap { size: 2048 })
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
            primary_window: Some(Window {
                title: "limbo pass".to_string(),
                // Fifo doesn't work on WASM, use AutoVsync instead
                present_mode: if cfg!(target_arch = "wasm32") {
                    PresentMode::AutoVsync
                } else {
                    PresentMode::Fifo
                },
                // Make window fill the viewport on WASM
                fit_canvas_to_parent: cfg!(target_arch = "wasm32"),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin::default())
        .add_plugins(AudioPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .init_state::<AppState>()
        .add_systems(OnEnter(AppState::Loading), (
            setup::camera,
            setup::lighting,
            setup::physics,
            scenes::load,
            theme::load,
        ))
        .add_systems(Update, (
            setup::check_loaded.run_if(in_state(AppState::Loading)),
        ))
        .add_systems(OnExit(AppState::Loading), (
            scenes::spawn,
            theme::play,
            setup::spawn_controls_text,
        ))
        .add_systems(Update, (
            form::get_movement.run_if(in_state(AppState::Running)),
            form::apply_movement.after(form::get_movement).run_if(in_state(AppState::Running)),
            form::wrap_movement.after(form::apply_movement).run_if(in_state(AppState::Running)),
        ))
        .run();
}
