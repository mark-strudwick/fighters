use bevy::{
    prelude::*,
    window::{close_on_esc, WindowTheme},
};
use component::{Fighter, Kinematic, Player, Viewport};
use entity::spawn_fighter;

mod component;
mod entity;
pub mod util;

/// The initial screen width and target viewport
const SCREEN_WIDTH: f32 = 800.0;
/// The initial screen height and target viewport
const SCREEN_HEIGHT: f32 = 600.0;
/// The base speed of the fighters in pixels per second.
const FIGHTER_BASE_SPEED: f32 = 0.0; //150.0;
/// The movement speed of the fighters in pixels per second.
const FIGHTER_MOVEMENT_SPEED: f32 = 150.0;

fn main() {
    App::new()
        // .insert_resource(ClearColor(Color::LIME_GREEN))
        .insert_resource(ClearColor(Color::rgb(0.953, 0.878, 0.82)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Fighter".to_string(),
                resolution: (SCREEN_WIDTH, SCREEN_HEIGHT).into(),
                window_theme: Some(WindowTheme::Light),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_systems(Update, (close_on_esc, kinematics, yaw, forward_moving))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default()).insert(Viewport {
        width: SCREEN_WIDTH,
        height: SCREEN_HEIGHT,
    });

    spawn_fighter(Player::Red, &mut commands, &mut meshes, &mut materials);
    spawn_fighter(Player::Blue, &mut commands, &mut meshes, &mut materials);
}

/// Updates all entities with transforms and kinematics to move in the direction they are facing,
/// at the velocity indicated by their kinematics.
pub fn kinematics(time: Res<Time>, mut kinematics: Query<(&Kinematic, &mut Transform)>) {
    for (kinematic, mut transform) in kinematics.iter_mut() {
        let forward = transform.up();
        transform.translation += forward * time.delta_seconds() * kinematic.velocity;
    }
}

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
