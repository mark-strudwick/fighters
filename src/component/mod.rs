use bevy::prelude::*;

mod player;

pub use player::Player;

#[derive(Debug, Component)]
pub struct Viewport {
    pub width: f32,
    pub height: f32,
}

/// Kinematic is attached to entities which move.
#[derive(Component)]
pub struct Kinematic {
    /// The speed an entity should move, in the direction indicated by its transform.
    pub velocity: f32,
}

/// Fighter is attached to each of the two player fighter entities.
#[derive(Component)]
pub struct Fighter;
