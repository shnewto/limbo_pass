use bevy::{
    gltf::{Gltf, GltfMesh},
    prelude::*,
};
use bevy::mesh::{Indices, VertexAttributeValues};

fn hex_to_color(hex: &str) -> Color {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    Color::srgb_u8(r, g, b)
}
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
        // Spawn foot light (this was previously unused)
        commands.spawn((
            PointLight {
                color: hex_to_color(point_light_color_hex_string),
                range: 2.,
                intensity: 100.,
                ..Default::default()
            },
            Transform::default(),
        ));

        let scene_handle = scenes_gltf.named_scenes["FORM"].clone();
        let form_entity = commands
            .spawn((
                SceneRoot(scene_handle),
                Transform::from_xyz(-45.0, 1.5, 0.0),
                RigidBody::Dynamic,
                Collider::ball(2.3),
                LockedAxes::ROTATION_LOCKED_X | LockedAxes::ROTATION_LOCKED_Z,
                Velocity::zero(),
                ExternalForce::default(),
                Movements::default(),
                GravityScale(10.0), // Make gravity 2x stronger for faster falling
                Form {
                    // nothing special about these values, just played around until it felt like a ghost
                    thrust: Vec3::new(300.0, 100.0, 300.0),
                    drag: Vec3::new(250.0, 500.0, 250.0),
                },
            ))
            .id();
        
        bevy::log::info!("Form entity spawned with collider: {:?}", form_entity);

        // Spawn terrain scene - it will spawn even if we can't extract mesh data for collider
        let terrain_scene_handle = scenes_gltf.named_scenes["TERRAIN"].clone();
        let mut terrain_entity = commands.spawn(SceneRoot(terrain_scene_handle));
        
        // Terrain needs to be a static rigid body for collisions to work
        terrain_entity.insert(RigidBody::Fixed);

        // Try to add collider if we can extract mesh data
        let terrain_mesh_handle = scenes_gltf.named_meshes.get("TERRAIN");
        
        if let Some(terrain_mesh_handle) = terrain_mesh_handle {
            bevy::log::info!("Found TERRAIN mesh handle: {:?}", terrain_mesh_handle);
            
            let terrain_mesh: Option<&Mesh> = gltf_meshes
                .get(terrain_mesh_handle)
                .and_then(|gltf_mesh| {
                    bevy::log::info!("GltfMesh has {} primitives", gltf_mesh.primitives.len());
                    gltf_mesh.primitives.first()
                })
                .and_then(|terrain_primitive| {
                    bevy::log::info!("Trying to get mesh from primitive: {:?}", terrain_primitive.mesh);
                    meshes.get(&terrain_primitive.mesh)
                });

            if let Some(terrain_mesh) = terrain_mesh {
                bevy::log::info!("Successfully got terrain mesh");
                let attribute_positions = terrain_mesh.attribute(Mesh::ATTRIBUTE_POSITION);
                let indices = terrain_mesh.indices();

                // Match on both attribute and index types
                match (attribute_positions, indices) {
                    (
                        Some(VertexAttributeValues::Float32x3(vertex_values)),
                        Some(Indices::U32(index_values)),
                    ) => {
                        let vertices: Vec<Vect> = vertex_values
                            .iter()
                            .map(|v| Vec3::new(v[0], v[1], v[2]))
                            .collect();

                        let indices: Vec<[u32; 3]> = index_values
                            .chunks(3)
                            .map(|chunk| [chunk[0], chunk[1], chunk[2]])
                            .collect();

                        let vertex_count = vertices.len();
                        let triangle_count = indices.len();
                        if let Ok(collider) = Collider::trimesh(vertices, indices) {
                            terrain_entity.insert(collider);
                            terrain_entity.insert(ActiveEvents::COLLISION_EVENTS);
                            bevy::log::info!("Terrain collider added successfully with {} vertices, {} triangles", vertex_count, triangle_count);
                        } else {
                            bevy::log::warn!("Failed to create terrain trimesh collider");
                        }
                    }
                    (
                        Some(VertexAttributeValues::Float32x3(vertex_values)),
                        Some(Indices::U16(index_values)),
                    ) => {
                        // Handle U16 indices
                        let vertices: Vec<Vect> = vertex_values
                            .iter()
                            .map(|v| Vec3::new(v[0], v[1], v[2]))
                            .collect();

                        let indices: Vec<[u32; 3]> = index_values
                            .chunks(3)
                            .map(|chunk| [chunk[0] as u32, chunk[1] as u32, chunk[2] as u32])
                            .collect();

                        let vertex_count = vertices.len();
                        let triangle_count = indices.len();
                        if let Ok(collider) = Collider::trimesh(vertices, indices) {
                            terrain_entity.insert(collider);
                            terrain_entity.insert(ActiveEvents::COLLISION_EVENTS);
                            bevy::log::info!("Terrain collider added successfully with {} vertices, {} triangles (U16 indices)", vertex_count, triangle_count);
                        } else {
                            bevy::log::warn!("Failed to create terrain trimesh collider");
                        }
                    }
                    (pos_attr, idx) => {
                        bevy::log::warn!("Failed to extract terrain mesh data for collider - unsupported format");
                        if pos_attr.is_none() {
                            bevy::log::warn!("  - No position attribute found");
                        } else {
                            bevy::log::warn!("  - Position attribute is not Float32x3");
                        }
                        if idx.is_none() {
                            bevy::log::warn!("  - No indices found");
                        } else {
                            bevy::log::warn!("  - Indices are not U32 or U16");
                        }
                    }
                }
            } else {
                bevy::log::warn!("Failed to get terrain mesh from GltfMesh or Mesh assets");
            }
        } else {
            bevy::log::warn!("TERRAIN mesh not found in named_meshes");
        }
    }
}
