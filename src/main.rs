use bevy::{
    prelude::*,
    window::{close_on_esc, WindowTheme},
};
use component::{Fighter, Kinematic, Player, Viewport};
use entity::spawn_fighter;
use system::{forward_moving, kinematics, yaw};

mod component;
mod entity;
mod system;
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
