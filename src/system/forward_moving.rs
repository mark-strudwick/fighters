use bevy::prelude::*;

use crate::component::{Fighter, Kinematic, Player};

/// The forward moving system handles moving the player fighters forward, and stopping them.
pub fn forward_moving(
    keys: Res<Input<KeyCode>>,
    mut breakers: Query<(With<Fighter>, &Player, &mut Kinematic)>,
) {
    for (_fighter, player, mut kinematic) in breakers.iter_mut() {
        let break_keys: &[KeyCode] = match player {
            Player::Red => &[KeyCode::W],
            Player::Blue => &[KeyCode::I, KeyCode::Up],
        };
        if keys.any_just_pressed(break_keys.iter().copied()) {
            kinematic.velocity = crate::FIGHTER_MOVEMENT_SPEED;
        }
        if keys.any_just_released(break_keys.iter().copied()) {
            kinematic.velocity = crate::FIGHTER_BASE_SPEED;
        }
    }
}
