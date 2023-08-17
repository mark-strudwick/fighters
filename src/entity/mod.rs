use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

/// Spawns a new fighter for a specified player.
pub fn spawn_fighter(
    player: crate::Player,
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
) {
    let (starting_translation, starting_rotation) = player.starts_at();
    commands
        .spawn(MaterialMesh2dBundle {
            mesh: meshes
                .add(crate::util::create_simple_mesh(
                    vec![[0.0, 19.5, 0.0], [11.7, -8.25, 0.0], [-11.7, -8.25, 0.0]],
                    vec![0, 2, 1],
                ))
                .into(),
            material: materials.add(ColorMaterial::from(player.color())),
            transform: Transform::default()
                .with_translation(starting_translation)
                .with_rotation(Quat::from_rotation_z(starting_rotation)),
            ..Default::default()
        })
        .insert(player)
        .insert(crate::Kinematic {
            velocity: crate::FIGHTER_BASE_SPEED,
        })
        .insert(crate::Fighter);
}
