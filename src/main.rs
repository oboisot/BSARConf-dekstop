mod mesh;
mod scene;

use mesh::LineList;
use scene::{
    pan_orbit_camera, PanOrbitCameraBundle, PanOrbitState,
    spawn_axis_helper
};

use bevy::{
    prelude::*,
    render::{
        mesh::ConeAnchor,
        camera::Exposure
    }
};
use std::f32::consts::FRAC_PI_2;

//
#[derive(Component)]
struct TxCarrierRefMarker;

#[derive(Component)]
struct TxAntennaConeMarker;

#[derive(Component)]
struct RxCarrierRefMarker;


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
        .add_systems(PostStartup, set_tx_carrier_transform)
        .add_systems(Update, pan_orbit_camera.run_if(any_with_component::<PanOrbitState>))
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    // asset_server: Res<AssetServer>
) {

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

    const HALF_PLANE_SIZE: f32 = 15000.0;
    const GRID_SIZE: f32 = 500.0;
    // opaque plane
    let world_plane = commands.spawn(
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
    ).id();

    let mut lines = Vec::<(Vec3, Vec3)>::with_capacity(122);
    let mut y: f32;
    for i in 0..=60 {
        y = -HALF_PLANE_SIZE + GRID_SIZE * i as f32;
        lines.push(
            (Vec3::new(-HALF_PLANE_SIZE, y, 0.0), Vec3::new(HALF_PLANE_SIZE, y, 0.0))
        )
    }
    let mut x: f32;
    for i in 0..=60 {
        x = -HALF_PLANE_SIZE + GRID_SIZE * i as f32;
        lines.push(
            (Vec3::new(x, -HALF_PLANE_SIZE, 0.0), Vec3::new(x, HALF_PLANE_SIZE, 0.0))
        )
    }

    let world_grid = commands.spawn(
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
    ).id();

    let world_axis_helper = spawn_axis_helper(&mut commands, &mut meshes, &mut materials, 500.0);

    commands
        .entity(world_plane)
        .push_children(&[world_grid, world_axis_helper]);


    // Transmitter
    let tx_carrier_ref = spawn_axis_helper(&mut commands, &mut meshes, &mut materials, 100.0);
    let tx_antenna_ref = spawn_axis_helper(&mut commands, &mut meshes, &mut materials, 100.0);
    let tx_antenna_cone = commands.spawn(
        (
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
                transform: Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)) // Cone along X-axis
                    .with_scale(Vec3::new(1.0, 0.75, 1.0)), // opening in Y -> azimuth, Z -> elevation
                ..Default::default()
            },
            TxAntennaConeMarker // Add a marker component to Tx Antenna Cone entity
        )
    ).id();

    commands // Antenna cone is the child of tx_antenna_ref...
        .entity(tx_antenna_ref)
        .add_child(tx_antenna_cone);
    commands // Which is the child of 
        .entity(tx_carrier_ref)
        .insert(TxCarrierRefMarker) // Add a marker component to Tx Carrier entity
        .add_child(tx_antenna_ref);

    //let q = Query<(Entity, &mut Transform), With<TxCarrierRefMarker>>;
    

    // commands.spawn(
    //     SceneBundle {
    //         scene: asset_server.load("models/axis_helper.glb#Scene0"),
    //         transform: Transform::from_scale(Vec3::splat(100.0))
    //             .with_rotation(Quat::from_rotation_z(PI)*Quat::from_rotation_y(FRAC_PI_2)*Quat::from_rotation_z(-FRAC_PI_2))
    //             .with_translation(Vec3::new(-1000.0, 0.0, 3000.0)),
    //         ..Default::default()
    //     }
    // ).with_children(|parent| {
    //     parent.spawn(
    //         SceneBundle {
    //             scene: asset_server.load("models/axis_helper.glb#Scene0"),
    //             transform: Transform::from_rotation(
    //                 // Quat::from_rotation_z(PI*0.25) *// <-> ELEVATION
    //                 // Quat::from_rotation_x(PI*0.25) * // <-> BANK
    //                 // Quat::from_rotation_y(0.25*PI) // <-> HEADING
    //                 Quat::from_rotation_x(0.0) * // <-> BANK
    //                 Quat::from_rotation_z(PI*0.25) * // <-> -ELEVATION
    //                 Quat::from_rotation_y(FRAC_PI_2)   // <-> HEADING
    //             ),
    //             ..Default::default()
    //         }
    //     ).with_children(|parent| {
    //         parent.spawn(
    //             PbrBundle {
    //                 mesh: meshes.add(Cone {
    //                     radius: 1e6,
    //                     height: 1e7
    //                 }.mesh()
    //                 .resolution(360)
    //                 .anchor(ConeAnchor::Tip)),
    //                 material: materials.add(
    //                     StandardMaterial {
    //                         base_color: Color::srgba(1.0, 1.0, 1.0, 0.5),
    //                         alpha_mode: AlphaMode::Blend,
    //                         reflectance: 0.0,
    //                         ..Default::default()
    //                     }
    //                 ),
    //                 transform: Transform::from_scale(0.01*Vec3::new(1.0, 1.0, 0.4))
    //                     .with_rotation(Quat::from_rotation_z(FRAC_PI_2)),
    //                 ..Default::default()
    //             }
    //         );
    //     });
    // });


}


fn set_tx_carrier_transform(mut query: Query<&mut Transform, With<TxCarrierRefMarker>>) {
    let mut transform = query.get_single_mut().expect("tirixywtrgze");

    transform.translation = Vec3::new(-1000.0, 0.0, 2500.0);
}