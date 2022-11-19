use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};
use crate::scenes::SceneHandle;
use crate::theme::ThemeState;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    Loading,
    Running,
}


pub fn lighting(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    let clear_color_hex_string = "0a0e17";
    commands.insert_resource(ClearColor(
        Color::hex(clear_color_hex_string)
            .unwrap_or_else(|_| panic!("couldn't make hex color from {}", clear_color_hex_string)),
    ));
    ambient_light.brightness = 0.6;
    ambient_light.color = Color::SILVER;
    let point_light_intensity = 20000.0;
    let point_light_color_hex_string = "AB69E7";
    let color = Color::hex(point_light_color_hex_string).unwrap_or_else(|_| {
        panic!(
            "couldn't make hex color from {}",
            point_light_color_hex_string
        )
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color,
            range: 50.,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(-40.0, 20.0, 0.0),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color,
            range: 50.,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(40.0, 20.0, 0.0),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color,
            range: 50.,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 20.0, -40.0),
        ..Default::default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            color,
            range: 50.,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 20.0, 40.0),
        ..Default::default()
    });
}

pub fn physics(mut physics_config: ResMut<RapierConfiguration>) {
    physics_config.gravity = Vec3::ZERO;
    physics_config.gravity.y = -100.0;
}

pub fn camera(mut commands: Commands) {
    commands.spawn(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        Vec3::new(-100.0, 60.0, 20.0),
        Vec3::new(0.0, 0.0, 0.0),
    ));
}


pub fn check_loaded(
    asset_server: Res<AssetServer>,
    audio_state: Res<ThemeState>,
    scene_handle: Res<SceneHandle>,
    mut state: ResMut<State<AppState>>,
) {
    if LoadState::Loaded != asset_server.get_load_state(&audio_state.loop_handle) {
        return;
    }

    if LoadState::Loaded != asset_server.get_load_state(&scene_handle.handle) {
        return;
    }

    state.set(AppState::Running).unwrap()
}