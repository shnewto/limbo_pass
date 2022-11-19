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
        .add_state(setup::AppState::Loading)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                title: "limbo pass".to_string(),
                present_mode: PresentMode::Fifo,
                ..default()
            },
            ..default()
        }))
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_system_set(SystemSet::on_enter(setup::AppState::Loading).with_system(setup::camera))
        .add_system_set(SystemSet::on_enter(setup::AppState::Loading).with_system(setup::lighting))
        .add_system_set(SystemSet::on_enter(setup::AppState::Loading).with_system(setup::physics))
        .add_system_set(SystemSet::on_enter(setup::AppState::Loading).with_system(scenes::load))
        .add_system_set(SystemSet::on_enter(setup::AppState::Loading).with_system(theme::load))
        .add_system_set(SystemSet::on_update(setup::AppState::Loading).with_system(setup::check_loaded))
        .add_system_set(SystemSet::on_exit(setup::AppState::Loading).with_system(scenes::spawn))
        .add_system_set(SystemSet::on_exit(setup::AppState::Loading).with_system(theme::play))
        .add_system(form::get_movement.label("get_movement"))
        .add_system(
            form::apply_movement
                .after("get_movement")
                .label("apply_movement"),
        )
        .add_system(form::wrap_movement.after("apply_movement"))
        .run();
}
