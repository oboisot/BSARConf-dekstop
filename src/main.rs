//! Demonstrates how to use transparency in 3D.
//! Shows the effects of different blend modes.
//! The `fade_transparency` system smoothly changes the transparency over time.

use bevy::prelude::*;

mod assets;
use assets::controls::pan_orbit_controls::{PanOrbitCamera, pan_orbit_camera};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 1.0})
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    position: WindowPosition::Automatic,
                    resolution: [800.0, 600.0].into(),
                    title: "BSAR Configurator".to_string(),
                    ..Default::default()
                    }),
                ..default()
            }
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, (axes, grid, pan_orbit_camera))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // opaque plane, uses `alpha_mode: Opaque` by default
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Quad::new(Vec2::splat(30000.0)).into()),
        material: materials.add(Color::hex("8b8989").unwrap().into()),
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
    // opaque cylinder
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Cylinder {
                radius: 250.0,
                height: 300.0,
                resolution: 10u32,
                segments: 1u32
            })
            .unwrap(),
        ),
        material: materials.add(Color::rgba(1.0, 1.0, 1.0, 0.75).into()),
        transform: Transform::from_rotation(Quat::from_rotation_x(std::f32::consts::FRAC_PI_2)),
        ..default()
    });
    // // Spawn a line strip that goes from point to point
    // commands.spawn(MaterialMeshBundle {
    //     mesh: meshes.add(Mesh::from(LineStrip {
    //         points: vec![
    //             Vec3::ZERO,
    //             Vec3::new(0.0, 0.0, 10000.0),
    //             Vec3::new(15000.0, 0.0, 0.0),
    //         ],
    //     })),
    //     transform: Transform::from_xyz(0.0, 0.0, 0.0),
    //     material: materials.add(Color::BLACK.into()),
    //     ..default()
    // });
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
    const ORIGIN: Vec3 = Vec3{ x: 0.0, y: 0.0, z: 0.1 };
    const X: Vec3 = Vec3{ x: 500.0, y:   0.0, z: 0.1 };
    const Y: Vec3 = Vec3{ x:   0.0, y: 500.0, z: 0.1 };
    const Z: Vec3 = Vec3{ x:   0.0, y:   0.0, z: 500.0 };
    gizmos.line(ORIGIN, X, Color::RED);    // World X-axis
    gizmos.line(ORIGIN, Y, Color::GREEN);  // World Y-axis
    gizmos.line(ORIGIN, Z, Color::BLUE);   // World Z-axis
}

fn grid(mut gizmos: Gizmos) {
    const SIZE: f32 = 30000.0;
    const SPACING: f32 = 500.0;
    const HALF_SIZE: f32 = 0.5 * SIZE;
    const NUM_LINES: usize = (SIZE / SPACING) as usize;
    // X-axis grid
    const DVECX: Vec3    = Vec3{ x:        0.0, y:    SPACING, z: 0.0 };
    let mut startx: Vec3 = Vec3{ x: -HALF_SIZE, y: -HALF_SIZE, z: 0.1 };
    let mut stopx: Vec3  = Vec3{ x:  HALF_SIZE, y: -HALF_SIZE, z: 0.1 };
    // Y-axis grid
    const DVECY: Vec3    = Vec3{ x:    SPACING, y:        0.0, z: 0.0 };
    let mut starty: Vec3 = Vec3{ x: -HALF_SIZE, y: -HALF_SIZE, z: 0.1 };
    let mut stopy: Vec3  = Vec3{ x: -HALF_SIZE, y:  HALF_SIZE, z: 0.1 };  
    for _ in 0..=NUM_LINES {
        gizmos.line(startx, stopx, Color::DARK_GRAY); // Draw X-lines
        gizmos.line(starty, stopy, Color::DARK_GRAY); // Draw Y-lines
        startx += DVECX; // Update X line coordinates
        stopx  += DVECX;        
        starty += DVECY; // Update Y line coordinates
        stopy  += DVECY
    }
}


// /// A list of lines with a start and end position
// #[derive(Debug, Clone)]
// pub struct LineList {
//     pub lines: Vec<(Vec3, Vec3)>,
// }

// impl From<LineList> for Mesh {
//     fn from(line: LineList) -> Self {
//         // This tells wgpu that the positions are list of lines
//         // where every pair is a start and end point
//         let mut mesh = Mesh::new(PrimitiveTopology::LineList);

//         let vertices: Vec<_> = line.lines.into_iter().flat_map(|(a, b)| [a, b]).collect();
//         mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
//         mesh
//     }
// }

// /// A list of points that will have a line drawn between each consecutive points
// #[derive(Debug, Clone)]
// pub struct LineStrip {
//     pub points: Vec<Vec3>,
// }

// impl From<LineStrip> for Mesh {
//     fn from(line: LineStrip) -> Self {
//         // This tells wgpu that the positions are a list of points
//         // where a line will be drawn between each consecutive point
//         let mut mesh = Mesh::new(PrimitiveTopology::LineStrip);

//         mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, line.points);
//         mesh
//     }
// }