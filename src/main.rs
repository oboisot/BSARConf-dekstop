use std::f32::consts::{FRAC_PI_2, PI};

use bevy::{
    prelude::*,
    render::{
        mesh::ConeAnchor,
        camera::Exposure
    }
};

mod assets;
use assets::mesh::antenna_cone::Cone as AntennaCone;
use assets::mesh::lines::LineList;
use assets::controls::pan_orbit_controls::{pan_orbit_camera, PanOrbitCameraBundle, PanOrbitState};
use assets::mesh::axis_helper::axis_helper_commands_spawn;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        // .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(AmbientLight{color: Color::WHITE, brightness: 1500.0})
        .insert_resource(AmbientLight::default())
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
        .add_systems(Update, pan_orbit_camera.run_if(any_with_component::<PanOrbitState>))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    const HALF_PLANE_SIZE: f32 = 15000.0;
    // opaque plane
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(
                Plane3d::new(Vec3::Z, Vec2::splat(HALF_PLANE_SIZE)).mesh().subdivisions(0)
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb_u8(120, 120, 120),
                ..default()
            }),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, -0.1)),
            ..Default::default()
        }
    );

    let mut lines = Vec::<(Vec3, Vec3)>::with_capacity(122);
    let mut y: f32;
    for i in 0..=60 {
        y = -HALF_PLANE_SIZE + 500.0 * i as f32;
        lines.push(
            (Vec3::new(-HALF_PLANE_SIZE, y, 0.0), Vec3::new(HALF_PLANE_SIZE, y, 0.0))
        )
    }
    let mut x: f32;
    for i in 0..=60 {
        x = -HALF_PLANE_SIZE + 500.0 * i as f32;
        lines.push(
            (Vec3::new(x, -HALF_PLANE_SIZE, 0.0), Vec3::new(x, HALF_PLANE_SIZE, 0.0))
        )
    }

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(
                LineList { lines }
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb_u8(188, 188, 188),//Color::srgb_u8(0, 51, 102),
                ..default()
            }),
            ..Default::default()
        }
    );

    // // Antenna cone
    // let ac_radius_m = 500.0f32;
    // let ac_length_m = 1000.0f32;
    // let elv_deg = 18.0f32;
    // let azi_deg = 5.0f32;

    // commands.spawn(
    //     PbrBundle {
    //         mesh: meshes.add(AntennaCone {
    //             radius: 1.0,
    //             height: 1.0,
    //             radial_segments: 360,
    //             height_segments: 18,
    //             wireframe: true
    //         }),
    //         material: materials.add(StandardMaterial {
    //             base_color: Color::WHITE,
    //             ..Default::default()
    //         }),
    //         transform: Transform::from_scale(Vec3::new(ac_length_m, ac_radius_m, ac_radius_m))
    //             .with_translation(Vec3::new(-1000.0, 0.0, 3000.0))
    //             .with_rotation(
    //                 Quat::from_mat3(&Mat3{
    //                     x_axis: Vec3::Y,
    //                     y_axis: Vec3::X,
    //                     z_axis: -Vec3::Z
    //                 })
    //             ),
    //         ..Default::default()
    //     }
    // );

    commands.spawn(
        SceneBundle {
            scene: asset_server.load("models/axis_helper.glb#Scene0"),
            transform: Transform::from_scale(Vec3::splat(100.0))
                .with_rotation(Quat::from_rotation_z(PI)*Quat::from_rotation_y(FRAC_PI_2)*Quat::from_rotation_z(-FRAC_PI_2))
                .with_translation(Vec3::new(-1000.0, 0.0, 3000.0)),
            ..Default::default()
        }
    ).with_children(|parent| {
        parent.spawn(
            SceneBundle {
                scene: asset_server.load("models/axis_helper.glb#Scene0"),
                transform: Transform::from_rotation(
                    // Quat::from_rotation_z(PI*0.25) *// <-> ELEVATION
                    // Quat::from_rotation_x(PI*0.25) * // <-> BANK
                    // Quat::from_rotation_y(0.25*PI) // <-> HEADING
                    Quat::from_rotation_x(0.0) * // <-> BANK
                    Quat::from_rotation_z(PI*0.25) * // <-> -ELEVATION
                    Quat::from_rotation_y(FRAC_PI_2)   // <-> HEADING
                ),
                ..Default::default()
            }
        ).with_children(|parent| {
            parent.spawn(
                PbrBundle {
                    mesh: meshes.add(Cone {
                        radius: 1e6,
                        height: 1e7
                    }.mesh()
                    .resolution(360)
                    .anchor(ConeAnchor::Tip)),
                    material: materials.add(
                        StandardMaterial {
                            base_color: Color::srgba(1.0, 1.0, 1.0, 0.5),
                            alpha_mode: AlphaMode::Blend,
                            reflectance: 0.0,
                            ..Default::default()
                        }
                    ),
                    transform: Transform::from_scale(0.01*Vec3::new(1.0, 1.0, 0.4))
                        .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
                    ..Default::default()
                }
            );
        });
    });


    // Cone
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(Cone {
                radius: 500.0,
                height: 1000.0
            }.mesh()
             .resolution(360)
             .anchor(ConeAnchor::Tip)),
            // material: materials.add(Color::srgba(1.0, 1.0, 1.0, 0.1)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgba(1.0, 0.0, 0.0, 0.5),
                alpha_mode: AlphaMode::Blend,
                ..Default::default()
            }),
            ..Default::default()
        }).with_children(|parent| {
            parent.spawn(
                SceneBundle {
                    scene: asset_server.load("models/axis_helper.glb#Scene0"),
                    transform: Transform::from_scale(Vec3::splat(500.0))
                        .with_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
                    ..Default::default()
                }
            );
        });
    
    
    // axis_helper_commands_spawn(&mut commands, meshes, materials, 500.0);

    commands.spawn(SceneBundle {
        // This is equivalent to "models/FlightHelmet/FlightHelmet.gltf#Scene0"
        // The `#Scene0` label here is very important because it tells bevy to load the first scene in the glTF file.
        // If this isn't specified bevy doesn't know which part of the glTF file to load.
        scene: asset_server.load("models/axis_helper.glb#Scene0"),
        // You can use the transform to give it a position
        transform: Transform::from_xyz(0.0, 0.0, 0.0)
            .with_rotation(Quat::from_rotation_x(FRAC_PI_2))
            .with_scale(Vec3::splat(500.0)),
        ..Default::default()
    });



    // Camera
    commands.spawn(
        PanOrbitCameraBundle {
            camera: Camera3dBundle {
                camera: Camera {
                    clear_color: ClearColorConfig::Custom(Color::BLACK),
                    ..Default::default()
                },
                exposure: Exposure::INDOOR,
                ..Default::default()
            },
            ..Default::default()
        }
    );

    
}

// fn axes(mut gizmos: Gizmos) {
//     const X: Vec3 = Vec3{ x: 500.0, y:   0.0, z: 0.0 };
//     const Y: Vec3 = Vec3{ x:   0.0, y: 500.0, z: 0.0 };
//     const Z: Vec3 = Vec3{ x:   0.0, y:   0.0, z: 500.0 };
//     gizmos.arrow(Vec3::ZERO, X, Srgba::RED);    // World X-axis
//     gizmos.arrow(Vec3::ZERO, Y, Srgba::GREEN);  // World Y-axis
//     gizmos.arrow(Vec3::ZERO, Z, Srgba::BLUE);   // World Z-axis

//     // const CELL_COUNT: UVec2 = UVec2{ x: 60, y: 60};
//     // const SPACING: Vec2 = Vec2{ x: 500.0, y: 500.0 };
//     // gizmos.grid(
//     //     Vec3::ZERO,
//     //     Quat::IDENTITY,
//     //     CELL_COUNT,
//     //     SPACING,
//     //     Color::srgb_u8(83, 104, 120)
//     // ).outer_edges();
// }
