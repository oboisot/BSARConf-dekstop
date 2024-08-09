//! Demonstrates how to use transparency in 3D.
//! Shows the effects of different blend modes.
//! The `fade_transparency` system smoothly changes the transparency over time.

use bevy::{
    prelude::*,
    render::mesh::ConeAnchor,
    render::render_resource::Face
};
// use bevy::math::prelude::Plane3d;
// use bevy::render::render_resource::Face;

mod assets;
// use assets::mesh::antenna_cone::Cone as AntennaCone;
use assets::controls::pan_orbit_controls::{PanOrbitCameraBundle, PanOrbitState, pan_orbit_camera};

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
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                axes,
                pan_orbit_camera.run_if(any_with_component::<PanOrbitState>)
            )
        )
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // asset_server: Res<AssetServer>
) {
    // opaque plane
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
            mesh: meshes.add(Cone {
                radius: 500.0,
                height: 1000.0
            }.mesh()
             .resolution(360)
             .anchor(ConeAnchor::Tip)),
            material: materials.add(StandardMaterial{
                base_color: Color::srgba(1.0, 1.0, 1.0, 0.25),
                double_sided: false,
                cull_mode: Some(Face::Back),
                ..Default::default()
            }),
            // transform: Transform::from_scale(Vec3::new(1.0, 0.25, 2.0))
            //     .with_rotation(Quat::from_mat3(&Mat3 { x_axis: Vec3::Y, y_axis: Vec3::X, z_axis: -Vec3::Z }))
            //     .with_translation(Vec3::new(-1000.0, 0.0, 1000.0)),
            ..Default::default()
        }//.with_children(|parent| {
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
    );
    // Camera
    commands.spawn(PanOrbitCameraBundle::default());
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
