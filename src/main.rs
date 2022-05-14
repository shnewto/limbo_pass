use bevy::{prelude::*, window::PresentMode};
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::{controllers::orbit::OrbitCameraPlugin, LookTransformPlugin};

mod asset;
mod form;
mod fsm;
mod limbo;
mod menu;
mod scenes;
mod setup;
mod theme;

fn main() {
    let clear_color_hex_string = "0a0e17";
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(WindowDescriptor {
            title: "limbo pass".to_string(),
            present_mode: PresentMode::Fifo,
            ..default()
        })
        .insert_resource(ClearColor(
            Color::hex(clear_color_hex_string).unwrap_or_else(|_| {
                panic!("couldn't make hex color from {}", clear_color_hex_string)
            }),
        ))
        .insert_resource(form::Movements::default())
        .add_plugins(DefaultPlugins)
        .add_plugin(LookTransformPlugin)
        .add_plugin(OrbitCameraPlugin::default())
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(limbo::LimboPlugin)
        .run();
}
