use crate::scenes::SceneHandle;
use crate::theme::ThemeState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

#[derive(States, Debug, Clone, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    Loading,
    Running,
}

fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Color::srgb_u8(r, g, b)
}

pub fn lighting(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
    let clear_color_hex_string = "0a0e17";
    commands.insert_resource(ClearColor(
        hex_to_color(clear_color_hex_string)
    ));
    // Increase ambient light brightness for better global illumination
    ambient_light.brightness = 600.0; // Increased from 0.6 for more visibility
    ambient_light.color = Color::srgb(0.75, 0.75, 0.75); // SILVER equivalent
    
    let point_light_color_hex_string = "AB69E7";
    let color = hex_to_color(point_light_color_hex_string);
    
    // Spawn 4 point lights around the scene
    // Making them WAY bigger and brighter to test visibility
    for (x, z) in [(-40.0, 0.0), (40.0, 0.0), (0.0, -40.0), (0.0, 40.0)] {
        commands.spawn((
            PointLight {
                color,
                range: 500.0, 
                intensity: 10_000_000.0,
                shadows_enabled: true,
                ..default()
            },
            Transform::from_xyz(x, 15.0, z),
        ));
    }
    
    bevy::log::info!("Spawned 4 point lights with default settings");
}

// Gravity scale will be applied to the form entity in scenes.rs
// Using GravityScale component on the rigid body to make it fall faster
pub fn physics() {
    // Gravity scale is set on the form entity in scenes::spawn
}

pub fn camera(mut commands: Commands) {
    commands
        .spawn(Camera3d::default())
        .insert(OrbitCameraBundle::new(
            OrbitCameraController::default(),
            Vec3::new(-100.0, 60.0, 20.0),
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::Y,
        ));
}

pub fn check_loaded(
    asset_server: Res<AssetServer>,
    audio_state: Res<ThemeState>,
    scene_handle: Res<SceneHandle>,
    mut state: ResMut<NextState<AppState>>,
) {
    if !matches!(asset_server.get_load_state(&audio_state.loop_handle), Some(LoadState::Loaded)) {
        return;
    }

    if !matches!(asset_server.get_load_state(&scene_handle.handle), Some(LoadState::Loaded)) {
        return;
    }

    state.set(AppState::Running)
}

pub fn spawn_controls_text(mut commands: Commands, asset_server: Res<AssetServer>) {
    let controls_text = "wander
--------------------
space bar
← ↑ ↓ →
w a s d

look
--------------------
hold ctrl + move mouse (orbit)
scroll (zoom)
right click (pan)";

    // Load a font that supports Unicode arrows from assets
    let font_handle = asset_server.load("font/NotoSansMono-Bold.ttf");

    commands
        .spawn(Node {
            width: Val::Px(200.),
            height: Val::Px(10.),
            position_type: PositionType::Absolute,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            left: Val::Px(10.),
            top: Val::Px(10.),
            ..default()
        })
        .with_children(|parent| {
            parent.spawn((
                Text(controls_text.to_string()),
                TextFont {
                    font: font_handle,
                    font_size: 16.,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}
