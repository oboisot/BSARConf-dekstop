//! Demonstrates how to use transparency in 3D.
//! Shows the effects of different blend modes.
//! The `fade_transparency` system smoothly changes the transparency over time.

use bevy::prelude::*;
use bevy::render::mesh::PrimitiveTopology;

pub mod pan_orbit_controls;
use pan_orbit_controls::{PanOrbitCamera, pan_orbit_camera};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 1.0})
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (axes, pan_orbit_camera))
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
        transform: Transform::from_rotation(
            Quat::from_mat3(&Mat3{ x_axis: Vec3::Y, y_axis: Vec3::Z, z_axis: Vec3::X }) // Set plane in the XY-plane (rather than XZ-plane)
        ),
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
        material: materials.add(Color::rgba(0.7, 0.2, 0.1, 0.5).into()),
        transform: Transform::from_xyz(0.0, 0.0, 1000.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
        ..default()
    });
    // Spawn a line strip that goes from point to point
    commands.spawn(MaterialMeshBundle {
        mesh: meshes.add(Mesh::from(LineStrip {
            points: vec![
                Vec3::ZERO,
                Vec3::new(0.0, 0.0, 10000.0),
                Vec3::new(15000.0, 0.0, 0.0),
            ],
        })),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        material: materials.add(Color::BLACK.into()),
        ..default()
    });
    // Camera
    let eye = Vec3::new(10e3, -10e3, 10e3);
    let focus = Vec3::ZERO;
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(eye)
                .looking_at(focus, Vec3::Z),
            ..Default::default()
        },
        PanOrbitCamera {
            focus,
            radius: (focus - eye).length(),
            ..Default::default()
        },
    ));
}

fn axes(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::X * 500.0, Color::RED);    // World X-axis
    gizmos.line(Vec3::ZERO, Vec3::Y * 500.0, Color::GREEN);  // World Y-axis
    gizmos.line(Vec3::ZERO, Vec3::Z * 500.0, Color::BLUE);   // World Z-axis
}


/// A list of lines with a start and end position
#[derive(Debug, Clone)]
pub struct LineList {
    pub lines: Vec<(Vec3, Vec3)>,
}

impl From<LineList> for Mesh {
    fn from(line: LineList) -> Self {
        // This tells wgpu that the positions are list of lines
        // where every pair is a start and end point
        let mut mesh = Mesh::new(PrimitiveTopology::LineList);

        let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh
    }
}

/// A list of points that will have a line drawn between each consecutive points
#[derive(Debug, Clone)]
pub struct LineStrip {
    pub points: Vec<Vec3>,
}

impl From<LineStrip> for Mesh {
    fn from(line: LineStrip) -> Self {
        // This tells wgpu that the positions are a list of points
        // where a line will be drawn between each consecutive point
        let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
        mesh
    }
}