use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

// Movement and Form implementation draws _heavily_ from the player/ship/controller in the
// blender_bevy_top_down_space_shooter, very cool project!
// https://github.com/sdfgeoff/blender_bevy_top_down_space_shooter

#[derive(Default, Component, Debug)]
pub struct Form {
    pub thrust: Vec3,
    pub drag: Vec3,
}

#[derive(Debug)]
pub enum Movement {
    PushForward(f32),
    PushBackward(f32),
    PushLeft(f32),
    PushRight(f32),
    TurnLeft(f32),
    TurnRight(f32),
    Lift(f32),
}

impl Movement {
    pub fn as_lin_vec(&self) -> Vec3 {
        match self {
            Self::PushForward(p) => Vec3::new(*p, 0.0, 0.0),
            Self::PushBackward(p) => Vec3::new(-*p, 0.0, 0.0),
            Self::PushLeft(p) => Vec3::new(0.0, 0.0, -*p),
            Self::PushRight(p) => Vec3::new(0.0, 0.0, *p),
            Self::Lift(p) => Vec3::new(0.0, *p, 0.0),
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
    pub fn as_ang_vec(&self) -> Vec3 {
        match self {
            Self::TurnLeft(p) => Vec3::new(0.0, *p, 0.0),
            Self::TurnRight(p) => Vec3::new(0.0, -*p, 0.0),
            _ => Vec3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Default, Component, Debug)]
pub struct Movements(Vec<Movement>);

pub fn get_movement(mut query: Query<&mut Movements>, keys: Res<Input<KeyCode>>) {
    for mut movements in query.iter_mut() {
        movements.0.clear();
        let push_factor = 30.0;
        let turn_factor = 20.0;
        if keys.pressed(KeyCode::W) || keys.pressed(KeyCode::Up) {
            movements.0.push(Movement::PushForward(push_factor))
        }
        if keys.pressed(KeyCode::S) || keys.pressed(KeyCode::Down) {
            movements.0.push(Movement::PushBackward(push_factor))
        }
        if keys.pressed(KeyCode::A) {
            movements.0.push(Movement::PushLeft(push_factor))
        }
        if keys.pressed(KeyCode::D) {
            movements.0.push(Movement::PushRight(push_factor))
        }

        if keys.pressed(KeyCode::Left) {
            movements.0.push(Movement::TurnLeft(turn_factor))
        }
        if keys.pressed(KeyCode::Right) {
            movements.0.push(Movement::TurnRight(turn_factor))
        }

        if keys.pressed(KeyCode::Space) {
            movements.0.push(Movement::Lift(90.0))
        }
    }
}

pub fn apply_movement(
    mut form_query: Query<(
        &Movements,
        &Form,
        &GlobalTransform,
        &mut ExternalForce,
        &Velocity,
    )>,
) {
    if let Ok((movements, form, global_transform, mut rb_forces, rb_velocities)) =
        form_query.get_single_mut()
    {
        let mut forces = Vec3::new(0.0, 0.0, 0.0);
        let mut torques = Vec3::new(0.0, 0.0, 0.0);

        for movement in movements.0.iter() {
            forces += movement.as_lin_vec() * form.thrust;
            torques += movement.as_ang_vec() * form.thrust;
        }

        let local_to_global = global_transform.compute_matrix();
        forces = local_to_global.transform_vector3(forces);
        torques = local_to_global.transform_vector3(torques);

        let linvel: Vec3 = rb_velocities.linvel;
        forces -= linvel * form.drag;
        let angvel: Vec3 = rb_velocities.angvel;
        torques -= angvel * form.drag;

        rb_forces.force = forces;
        rb_forces.torque = torques;
    }
}

pub fn wrap_movement(mut form_query: Query<(&Form, &mut Transform)>) {
    if let Ok((_form, mut transform)) = form_query.get_single_mut() {
        let max_terrain_coord = 50.0;
        let min_terrain_coord = -50.0;
        let current_x = transform.translation.x;
        let current_z = transform.translation.z;

        // starting coords
        // -40.0, 20.0, 0.0

        if current_x > max_terrain_coord
            || current_z > max_terrain_coord
            || current_x < min_terrain_coord
            || current_z < min_terrain_coord
        {
            transform.translation = Vec3::new(0.0, 20.0, 0.0);
        }
    }
}
