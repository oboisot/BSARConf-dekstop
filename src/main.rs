use bevy::{
    prelude::*,
    render::mesh::ConeAnchor
};

mod assets;
use assets::mesh::antenna_cone::Cone as AntennaCone;
use assets::mesh::lines::LineList;
use assets::controls::pan_orbit_controls::{pan_orbit_camera, PanOrbitCameraBundle, PanOrbitState};
use assets::mesh::axis_helper::{axis_helper_commands_spawn, axis_helper_children_spawn};

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight{color: Color::WHITE, brightness: 1500.0})
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
    // asset_server: Res<AssetServer>
) {
    const HALF_PLANE_SIZE: f32 = 15000.0;
    // opaque plane
    commands.spawn(
        PbrBundle {
            mesh: meshes.add(
                Plane3d::new(Vec3::Z, Vec2::splat(HALF_PLANE_SIZE)).mesh().subdivisions(0)
            ),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb_u8(100, 100, 100),
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
                base_color: Color::srgb_u8(0, 51, 102),
                ..default()
            }),
            ..Default::default()
        }
    );

    // Antenna cone
    let ac_radius_m = 500.0f32;
    let ac_length_m = 1000.0f32;
    let elv_deg = 18.0f32;
    let azi_deg = 5.0f32;

    commands.spawn(
        PbrBundle {
            mesh: meshes.add(AntennaCone {
                radius: 1.0,
                height: 1.0,
                radial_segments: 360,
                height_segments: 18,
                wireframe: true
            }),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            }),
            transform: Transform::from_scale(Vec3::new(ac_length_m, ac_radius_m, ac_radius_m))
                .with_translation(Vec3::new(-1000.0, 0.0, 3000.0))
                .with_rotation(
                    Quat::from_mat3(&Mat3{
                        x_axis: Vec3::Y,
                        y_axis: Vec3::X,
                        z_axis: -Vec3::Z
                    })
                ),
            ..Default::default()
        }
    ).with_children(|parent| {
        axis_helper_children_spawn(parent, &mut meshes, &mut materials, 1.0);
        // parent.spawn(
        //     PbrBundle {
        //         mesh: meshes.add(AntennaCone {
        //             radius: 1.0,
        //             height: 1.0,
        //             radial_segments: 360,
        //             height_segments: 18,
        //             wireframe: false
        //         }),
        //         material: materials.add(StandardMaterial {
        //             base_color: Color::srgba(0.0, 0.0, 0.0, 0.5),
        //             alpha_mode: AlphaMode::Blend,
        //             ..Default::default()
        //         }),
        //         transform: Transform::from_scale(Vec3::new(1.0, 0.3, 1.0)),
        //         ..Default::default()
        //     }
        // );
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

    
    
    axis_helper_commands_spawn(&mut commands, meshes, materials, 500.0);

    // Camera
    commands.spawn(PanOrbitCameraBundle::default());

    
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
