use bevy::prelude::*;

use crate::component::{Fighter, Player};

pub fn yaw(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    mut turners: Query<(With<Fighter>, &Player, &mut Transform)>,
) {
    for (_fighter, player, mut transform) in turners.iter_mut() {
        let (yaw_ccw_keys, yaw_cw_keys): (&[KeyCode], &[KeyCode]) = match player {
            Player::Red => (&[KeyCode::A], &[KeyCode::D]),
            Player::Blue => (&[KeyCode::Left, KeyCode::J], &[KeyCode::Right, KeyCode::L]),
        };
        let mut yaw_adj = 0.0;
        if keys.any_pressed(yaw_ccw_keys.iter().copied()) {
            yaw_adj += 1.0;
        }
        if keys.any_pressed(yaw_cw_keys.iter().copied()) {
            yaw_adj -= 1.0;
        }
        transform.rotate_z(time.delta_seconds() * yaw_adj * f32::to_radians(180.0));
    }
}
