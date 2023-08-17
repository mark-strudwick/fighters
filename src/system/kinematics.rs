use bevy::prelude::*;

use crate::component::Kinematic;

/// Updates all entities with transforms and kinematics to move in the direction they are facing,
/// at the velocity indicated by their kinematics.
pub fn kinematics(time: Res<Time>, mut kinematics: Query<(&Kinematic, &mut Transform)>) {
    for (kinematic, mut transform) in kinematics.iter_mut() {
        let forward = transform.up();
        transform.translation += forward * time.delta_seconds() * kinematic.velocity;
    }
}
