use crate::assets::SceneAssets;
use crate::fsm::Fsm;
use bevy::prelude::*;
use bevy::{
    asset::LoadState,
    gltf::{Gltf, GltfMesh},
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_rapier3d::prelude::*;

use crate::form::{Form, Movements};
use smooth_bevy_cameras::controllers::orbit::{OrbitCameraBundle, OrbitCameraController};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::on_enter(Fsm::Setup)
                .with_system(scene.label("scene"))
                .with_system(lighting.after("scene").label("lighting"))
                .with_system(physics.after("lighting").label("physics"))
                .with_system(complete.after("physics").label("complete")),
        )
        .add_system_set(SystemSet::on_exit(Fsm::Setup).with_system(camera));
    }
}

pub fn complete(mut state: ResMut<State<Fsm>>) {
    state.set(Fsm::Running).unwrap();
}

pub fn scene(
    asset_server: Res<AssetServer>,
    assets_gltf: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    scene_assets: ResMut<SceneAssets>,
    mut commands: Commands,
) {
    if let LoadState::Loaded = asset_server.get_load_state(&scene_assets.limbo_pass_handle) {
        let point_light_color_hex_string = "70FF00";
        if let Some(scenes_gltf) = assets_gltf.get(&scene_assets.limbo_pass_handle) {
            let _foot_light = PointLightBundle {
                point_light: PointLight {
                    color: Color::hex(point_light_color_hex_string).unwrap_or_else(|_| {
                        panic!(
                            "couldn't make hex color from {}",
                            point_light_color_hex_string
                        )
                    }),
                    range: 2.,
                    radius: 3.,
                    intensity: 100.,
                    ..Default::default()
                },
                ..Default::default()
            };

            commands
                .spawn_bundle(TransformBundle::from(Transform::from_xyz(-45.0, 1.5, 0.0)))
                .insert(RigidBody::Dynamic)
                .insert(LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z)
                .insert(Collider::ball(2.3))
                .insert(Velocity::zero())
                .insert(ExternalForce::default())
                .insert(Movements::default())
                .insert(Form {
                    // nothing special about these values, just played around until it felt like a ghost
                    thrust: Vec3::new(300.0, 100.0, 300.0),
                    drag: Vec3::new(250.0, 500.0, 250.0),
                })
                .with_children(|parent| {
                    parent.spawn_scene(scenes_gltf.named_scenes["FORM"].clone());
                });

            let terrain_mesh_handle = &scenes_gltf.named_meshes["TERRAIN"];

            let terrain_mesh: Option<&Mesh> = gltf_meshes
                .get(terrain_mesh_handle)
                .and_then(|gltf_mesh| gltf_mesh.primitives.get(0))
                .and_then(|terrain_primitive| meshes.get(&terrain_primitive.mesh));

            let attribute_positions =
                terrain_mesh.and_then(|m| m.attribute(Mesh::ATTRIBUTE_POSITION));

            if let (
                Some(VertexAttributeValues::Float32x3(vertex_values)),
                Some(Indices::U32(index_values)),
            ) = (attribute_positions, terrain_mesh.and_then(|m| m.indices()))
            {
                let vertices: Vec<Vect> = vertex_values
                    .iter()
                    .map(|v| Vec3::new(v[0], v[1], v[2]))
                    .collect();

                let indices: Vec<[u32; 3]> = index_values
                    .chunks(3)
                    .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                    .collect();

                commands
                    .spawn()
                    .insert(Collider::trimesh(vertices, indices))
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .with_children(|parent| {
                        parent.spawn_scene(scenes_gltf.named_scenes["TERRAIN"].clone());
                    });
            }
        }
    }
}

pub fn lighting(mut commands: Commands, mut ambient_light: ResMut<AmbientLight>) {
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
    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color,
            range: 50.,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(-40.0, 20.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color,
            range: 50.,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(40.0, 20.0, 0.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
        point_light: PointLight {
            color,
            range: 50.,
            intensity: point_light_intensity,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 20.0, -40.0),
        ..Default::default()
    });

    commands.spawn_bundle(PointLightBundle {
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
    commands.spawn_bundle(OrbitCameraBundle::new(
        OrbitCameraController::default(),
        PerspectiveCameraBundle::default(),
        Vec3::new(-100.0, 60.0, 20.0),
        Vec3::new(0.0, 0.0, 0.0),
    ));
}

// #[derive(Component)]
// pub struct MuteButton;

// pub fn mute_button(mut commands: Commands, font_assets: Res<FontAssets>) {
//     let clear_color_hex_string = "0a0e17";
//     commands
//         .spawn_bundle(ButtonBundle {
//             style: Style {
//                 size: Size::new(Val::Px(120.0), Val::Px(50.0)),
//                 margin: Rect::all(Val::Auto),
//                 justify_content: JustifyContent::Center,
//                 align_items: AlignItems::Center,
//                 position: Rect {
//                     left: Val::Px(450.0),
//                     bottom: Val::Px(250.0),
//                     ..default()
//                 },
//                 ..Default::default()
//             },
//             color: Color::hex(clear_color_hex_string)
//                 .unwrap_or_else(|_| {
//                     panic!("couldn't make hex color from {}", clear_color_hex_string)
//                 })
//                 .into(),
//             ..Default::default()
//         })
//         .insert(MuteButton)
//         .with_children(|parent| {
//             parent.spawn_bundle(TextBundle {
//                 text: Text {
//                     sections: vec![TextSection {
//                         value: "play theme".to_string(),
//                         style: TextStyle {
//                             font: font_assets.square_font_handle.clone(),
//                             font_size: 20.0,
//                             color: Color::rgb(0.9, 0.9, 0.9),
//                         },
//                     }],
//                     alignment: Default::default(),
//                 },
//                 ..Default::default()
//             });
//         });
// }
