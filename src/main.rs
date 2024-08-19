mod mesh;
mod scene;

use scene::{
    pan_orbit_camera, PanOrbitCameraBundle, PanOrbitState,
    entities::{spawn_world, spawn_axis_helper}
};

use bevy::{
    prelude::*,
    render::{
        mesh::ConeAnchor,
        camera::Exposure
    }
};
use bevy_mod_picking::prelude::*;
use lazy_static::lazy_static;
use std::f32::consts::FRAC_PI_2;

lazy_static!(

    static ref ENU_TO_NED_ROT: Quat = Quat::from_mat3(&Mat3 { // ENU -> NED rotation
        x_axis: Vec3::Y,
        y_axis: Vec3::X,
        z_axis: -Vec3::Z
    });

);

//
#[derive(Component)]
struct TxCarrierRefMarker;

#[derive(Component)]
struct TxAntennaRefMarker;

#[derive(Component)]
struct TxAntennaConeMarker;

// #[derive(Component)]
// struct RxCarrierRefMarker;


fn main() {
    App::new()
        .insert_resource(Msaa::default())
        // .insert_resource(ClearColor(Color::BLACK))
        // .insert_resource(AmbientLight{color: Color::WHITE, brightness: 1500.0})
        // .insert_resource(AmbientLight::default())
        // .insert_resource( // no need of an AmbientLight with "unlit: true" for materials
        //     AmbientLight {
        //         brightness: 80.0,
        //         color: LinearRgba::WHITE.into()
        //     }
        // )
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
        .add_plugins(DefaultPickingPlugins) // Includes a mesh raycasting backend by default
        .add_systems(Startup, setup_scene)
        .add_systems(PostStartup,
            (
                init_tx_carrier_transform,
                init_tx_antenna_transform,
                init_tx_antenna_cone_opening
            )
        )
        .add_systems(Update, pan_orbit_camera.run_if(any_with_component::<PanOrbitState>))
        .run();
}

fn setup_scene(
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
                // exposure: Exposure::INDOOR,
                ..Default::default()
            },
            ..Default::default()
        }
    );

    // let _world = spawn_world(&mut commands, &mut meshes, &mut materials);
    spawn_world(&mut commands, &mut meshes, &mut materials);

    // Transmitter
    let tx_carrier_ref = spawn_axis_helper(&mut commands, &mut meshes, &mut materials, 100.0);
    let tx_antenna_ref = spawn_axis_helper(&mut commands, &mut meshes, &mut materials, 50.0);
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
                        base_color: Color::srgba(1.0, 1.0, 1.0, 0.3),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..Default::default()
                    }
                ),
                transform: Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)), // Cone along X-axis
                ..Default::default()
            },
            PickableBundle::default(),
            TxAntennaConeMarker // Add a marker component to Tx Antenna Cone entity
        )
    ).id();

    commands // Antenna cone is the child of tx_antenna_ref...
        .entity(tx_antenna_ref)
        .insert(TxAntennaRefMarker) // Add a marker component to Tx Antenna entity
        .add_child(tx_antenna_cone);
    commands // Which is the child of 
        .entity(tx_carrier_ref)
        .insert(TxCarrierRefMarker) // Add a marker component to Tx Carrier entity
        .add_child(tx_antenna_ref);

}


fn init_tx_carrier_transform(mut query: Query<&mut Transform, With<TxCarrierRefMarker>>) {
    let mut transform = query
        .get_single_mut()
        .expect("Can't get `TxCarrierRef` transform");

    transform.translation = Vec3::new(-5000.0, 0.0, 3000.0);
    transform.rotation = ENU_TO_NED_ROT.clone();
}

fn init_tx_antenna_transform(mut query: Query<&mut Transform, With<TxAntennaRefMarker>>) {
    let mut transform = query
        .get_single_mut()
        .expect("Can't get `TxCarrierRef` transform");
    transform.translation = Vec3::new(0.0, 2.58, -0.53);
    transform.rotation = Quat::from_euler(
        EulerRot::ZYX,
        90.0f32.to_radians(),  // Heading
        -60.0f32.to_radians(), // Elevation
        0.0                    // Bank
    );
}

fn init_tx_antenna_cone_opening(
    mut query: Query<&mut Transform, With<TxAntennaConeMarker>>,
) {
    let mut transform = query
        .get_single_mut()
        .expect("Can't get `TxCarrierRef` transform");

    transform.scale = Vec3::new(
        1.0, // Azimuth aperture
        1.0,
        0.5  // Elevation aperture
    );
}