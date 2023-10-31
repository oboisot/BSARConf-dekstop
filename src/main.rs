//! Demonstrates how to use transparency in 3D.
//! Shows the effects of different blend modes.
//! The `fade_transparency` system smoothly changes the transparency over time.

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 1.0})
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, fade_transparency)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // opaque plane, uses `alpha_mode: Opaque` by default
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(30000.0).into()),
        material: materials.add(Color::hex("8b8989").unwrap().into()),
        ..default()
    });
    // transparent cube, uses `alpha_mode: Blend`
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube { size: 1000.0 })),
        // Notice how there is no need to set the `alpha_mode` explicitly here.
        // When converting a color to a material using `into()`, the alpha mode is
        // automatically set to `Blend` if the alpha channel is anything lower than 1.0.
        material: materials.add(Color::rgba(0.5, 0.5, 1.0, 0.0).into()),
        transform: Transform::from_xyz(0.0, 500.0, 0.0),
        ..default()
    });
    // opaque sphere
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 500.0,
                subdivisions: 3,
            })
            .unwrap(),
        ),
        material: materials.add(Color::rgb(0.7, 0.2, 0.1).into()),
        transform: Transform::from_xyz(0.0, 500.0, -1500.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
        ..default()
    });
    // camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(30e3, 30e3, 30e3).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera::default()
    ));
}

/// Fades the alpha channel of all materials between 0 and 1 over time.
/// Each blend mode responds differently to this:
/// - [`Opaque`](AlphaMode::Opaque): Ignores alpha channel altogether, these materials stay completely opaque.
/// - [`Mask(f32)`](AlphaMode::Mask): Object appears when the alpha value goes above the mask's threshold, disappears
///                when the alpha value goes back below the threshold.
/// - [`Blend`](AlphaMode::Blend): Object fades in and out smoothly.
pub fn fade_transparency(time: Res<Time>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let alpha = (time.elapsed_seconds().sin() / 2.0) + 0.5;
    for (_, material) in materials.iter_mut() {
        material.base_color.set_a(alpha);
    }
}
