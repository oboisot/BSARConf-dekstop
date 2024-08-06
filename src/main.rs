//! Demonstrates how to use transparency in 3D.
//! Shows the effects of different blend modes.
//! The `fade_transparency` system smoothly changes the transparency over time.

use bevy::{prelude::*, render::mesh::ConeAnchor};
use smooth_bevy_cameras::LookTransformPlugin;
use smooth_bevy_cameras::controllers::orbit::*;
// use bevy::math::prelude::Plane3d;
// use bevy::render::render_resource::Face;

mod assets;
// use assets::controls::pan_orbit_controls::{PanOrbitCamera, pan_orbit_camera};
use assets::mesh::antenna_cone::Cone as AntennaCone;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 500.0})
        .add_plugins(DefaultPlugins
            .set(
                WindowPlugin {
                    primary_window: Some(Window {
                        position: WindowPosition::Automatic,
                        resolution: [800.0, 600.0].into(),
                        title: "BSAR Configurator".to_string(),
                        ..Default::default()
                        }),
                    ..default()
                }
            )
            .set(
                AssetPlugin {
                    file_path: "assets".to_string(),
                    ..Default::default()
                }
            )
        )
        .add_plugins(LookTransformPlugin)
        .add_plugins(OrbitCameraPlugin { override_input_system: false })
        .add_systems(Startup, setup)
        .add_systems(Update, axes)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // asset_server: Res<AssetServer>
) {
    // opaque plane, uses `alpha_mode: Opaque` by default
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(
                Plane3d::new(Vec3::Z, Vec2::splat(15000.0)).mesh().subdivisions(0)
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.54, 0.54, 0.54),
                perceptual_roughness: 1.0,
                ..default()
            }),
            ..Default::default()
        }
    );
    // Cone
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(AntennaCone {
                    radius: 500.0,
                    height: 1000.0,
                    radial_segments: 360,
                    height_segments: 18,
                    wireframe: false
                }),
            // material: materials.add(Color::RED.into()),
            material: materials.add(StandardMaterial{
                base_color: Color::WHITE,
                // double_sided: true,
                // cull_mode: Some(Face::Front),
                cull_mode: None,
                ..Default::default()
            }),
            transform: Transform::from_scale(Vec3::new(1.0, 0.25, 2.0))
                .with_rotation(Quat::from_mat3(&Mat3 { x_axis: Vec3::Y, y_axis: Vec3::X, z_axis: -Vec3::Z }))
                .with_translation(Vec3::new(-1000.0, 0.0, 1000.0)),
            ..Default::default()
        }
    );
    //.with_children(|parent| {
    //     parent.spawn(PbrBundle {
    //         mesh: meshes.add(shape::Circle {
    //             radius: 500.0,
    //             vertices: 360
    //         }.into()),
    //         material: materials.add(StandardMaterial{
    //             base_color: Color::GREEN,
    //             cull_mode: None,
    //             ..Default::default()
    //         }),
    //         transform: Transform::from_rotation(Quat::from_rotation_y(std::f32::consts::FRAC_PI_2)),
    //         //     .with_translation(Vec3::new(1000.0, 0.0, 0.0)),
    //         ..Default::default()
    //     });
    // });

    // Cone
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Cone {
                radius: 500.0,
                height: 1000.0
            }.mesh()
             .resolution(360)
             .anchor(ConeAnchor::Tip)),
            material: materials.add(StandardMaterial{
                base_color: Color::BLACK,
                double_sided: true,
                // cull_mode: Some(Face::Front),
                cull_mode: None,
                ..Default::default()
            }),
            // transform: Transform::from_scale(Vec3::new(1.0, 0.25, 2.0))
            //     .with_rotation(Quat::from_mat3(&Mat3 { x_axis: Vec3::Y, y_axis: Vec3::X, z_axis: -Vec3::Z }))
            //     .with_translation(Vec3::new(-1000.0, 0.0, 1000.0)),
            ..Default::default()
        }
    );
    // // opaque sphere
    // const WGS84_EQUATORIAL_RADIUS_M: f64 = 6378137.0;
    // const WGS84_POLAR_RADIUS_M:      f64 = (1.0 - 1.0/298.257223563) * WGS84_EQUATORIAL_RADIUS_M;
    // commands.spawn(PbrBundle {
    //     mesh: meshes.add(shape::UVSphere {
    //             radius: 1.0,
    //             sectors: 360,
    //             stacks: 180
    //         }.into()),
    //     material: materials.add(StandardMaterial{
    //         base_color_texture: Some(asset_server.load("textures/world.200405.3x5400x2700.png")),
    //         base_color: Color::WHITE,
    //         ..Default::default()
    //     }),
    //     transform: Transform::from_xyz(0.0, 0.0, -6500e3)
    //         .with_rotation(Quat::from_rotation_z(std::f32::consts::PI))
    //         .with_scale(Vec3::new(
    //             WGS84_EQUATORIAL_RADIUS_M as f32,
    //             WGS84_POLAR_RADIUS_M as f32,
    //             WGS84_EQUATORIAL_RADIUS_M as f32
    //         )),
    //     ..default()
    // });
    // Camera
    let camera_bundle = OrbitCameraBundle::new(
        OrbitCameraController::default(),
        Vec3::new(10e3, -10e3, 10e3), // eye
        Vec3::ZERO, // target
        Vec3::Z //
    );
    commands.spawn(camera_bundle)
            .insert(Camera3dBundle::default());
}

fn axes(mut gizmos: Gizmos) {
    const ORIGIN: Vec3 = Vec3{ x: 0.0, y: 0.0, z: 0.0 };
    const X: Vec3 = Vec3{ x: 500.0, y:   0.0, z: 0.0 };
    const Y: Vec3 = Vec3{ x:   0.0, y: 500.0, z: 0.0 };
    const Z: Vec3 = Vec3{ x:   0.0, y:   0.0, z: 500.0 };
    gizmos.line(ORIGIN, X, Srgba::RED);    // World X-axis
    gizmos.line(ORIGIN, Y, Srgba::GREEN);  // World Y-axis
    gizmos.line(ORIGIN, Z, Srgba::BLUE);   // World Z-axis
}
