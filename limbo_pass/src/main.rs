use bevy::{prelude::*, window::PresentMode};
use bevy_kira_audio::AudioPlugin;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

mod form;
mod fsm;
mod scenes;
mod setup;
mod theme;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(WindowDescriptor {
            title: "limbo pass".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(form::Movements::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(AudioPlugin)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        // .add_plugin(RapierDebugRenderPlugin::default())
        .add_state(fsm::Fsm::Loading)
        .add_system_set(
            SystemSet::on_enter(fsm::Fsm::Loading)
                .with_system(setup::camera)
                .with_system(setup::lighting)
                .with_system(setup::physics)
                .with_system(scenes::load)
                .with_system(theme::load),
        )
        .add_system_set(SystemSet::on_update(fsm::Fsm::Loading).with_system(
            |mut state: ResMut<State<fsm::Fsm>>| state.set(fsm::Fsm::MainMenu).unwrap(),
        ))
        .add_system_set(SystemSet::on_enter(fsm::Fsm::MainMenu).with_system(fsm::main_menu_init))
        .add_system_set(
            SystemSet::on_update(fsm::Fsm::MainMenu).with_system(fsm::menu_interactions),
        )
        .add_system_set(
            SystemSet::on_enter(fsm::Fsm::Running)
                .with_system(theme::play)
                .with_system(scenes::spawn),
        )
        .add_system_set(
            SystemSet::on_update(fsm::Fsm::Running)
                .with_system(form::get_movement.label("get_movement"))
                .with_system(
                    form::apply_movement
                        .after("get_movement")
                        .label("apply_movement"),
                )
                .with_system(form::wrap_movement.after("apply_movement")),
        )
        .run();
}
