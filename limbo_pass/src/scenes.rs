use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
    render::mesh::{Indices, VertexAttributeValues},
};
use bevy_rapier3d::prelude::*;

use crate::form::{Form, Movements};

#[derive(Resource)]
pub struct SceneHandle {
    pub handle: Handle<Gltf>,
}

pub fn load(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(SceneHandle {
        handle: asset_server.load("gltf/limbo_pass.gltf"),
    });
}

pub fn spawn(
    gltf_assets: Res<Assets<Gltf>>,
    gltf_meshes: Res<Assets<GltfMesh>>,
    meshes: Res<Assets<Mesh>>,
    scene_handle: Res<SceneHandle>,
    mut commands: Commands,
) {
    let point_light_color_hex_string = "70FF00";
    if let Some(scenes_gltf) = gltf_assets.get(&scene_handle.handle) {
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
            .spawn(SceneBundle {
                scene: scenes_gltf.named_scenes["FORM"].clone(),
                ..default()
            })
            .insert(TransformBundle::from(Transform::from_xyz(-45.0, 1.5, 0.0)))
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
            });

        let terrain_mesh_handle = &scenes_gltf.named_meshes["TERRAIN"];

        let terrain_mesh: Option<&Mesh> = gltf_meshes
            .get(terrain_mesh_handle)
            .and_then(|gltf_mesh| gltf_mesh.primitives.get(0))
            .and_then(|terrain_primitive| meshes.get(&terrain_primitive.mesh));

        let attribute_positions = terrain_mesh.and_then(|m| m.attribute(Mesh::ATTRIBUTE_POSITION));

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
                .spawn(SceneBundle {
                    scene: scenes_gltf.named_scenes["TERRAIN"].clone(),
                    ..default()
                })
                .insert(Collider::trimesh(vertices, indices))
                .insert(ActiveEvents::COLLISION_EVENTS);
        }
    }
}
