mod constants;
mod mesh;
mod scene;

use scene::{
    pan_orbit_camera, PanOrbitCameraBundle, PanOrbitState,
    entities::{spawn_world, spawn_axis_helper}
};

use bevy::{
    prelude::*,
    math::DVec3,
    render::mesh::ConeAnchor
};
use bevy_mod_picking::prelude::*;
use std::f32::consts::FRAC_PI_2;

use crate::constants::ENU_TO_NED_ROT;

// The internal state of the Carrier
#[derive(Component)]
pub struct CarrierState {
    /// Carrier orientation in World frame (NED referential)
    pub heading_deg: f64,
    pub elevation_deg: f64,
    pub bank_deg: f64,
    ///
    pub height_m: f64,
    /// Carrier to Antenna phase center lever arms (in NED Carier frame)
    pub lever_arms_m: DVec3,

    // pub position_m: f64
}



// The internal state of the Antenna
#[derive(Component)]
pub struct AntennaState {
    /// Antenna orientation relative to Carrier
    pub heading_deg: f64,
    pub elevation_deg: f64,
    pub bank_deg: f64,
}

// The internal state of the Antenna
#[derive(Component)]
pub struct AntennaBeamState {
    /// Antenna 3d beam widths
    pub elevation_beam_width_deg: f64,
    pub azimuth_beam_width_deg: f64,
}

impl Default for CarrierState {
    fn default() -> Self {
        Self {
            heading_deg: 0.0,
            elevation_deg: 0.0,
            bank_deg: 0.0,
            height_m: 300.0,
            lever_arms_m: DVec3::ZERO,
        }
    }
}

impl Default for AntennaState {
    fn default() -> Self {
        Self {
            heading_deg: 90.0,
            elevation_deg: -60.0,
            bank_deg: 0.0,
        }
    }
}

impl Default for AntennaBeamState {
    fn default() -> Self {
        Self {
            elevation_beam_width_deg: 18.0,
            azimuth_beam_width_deg: 22.0
        }
    }
}

//
#[derive(Component)]
struct Tx;

#[derive(Component)]
struct Rx;


//
#[derive(Component)]
struct TxCarrierRefMarker;

#[derive(Component)]
struct TxAntennaRefMarker;

#[derive(Component)]
struct TxAntennaConeMarker;

// #[derive(Component)]
// struct RxCarrierRefMarker;

// We can use a dynamic highlight that builds a material based on the entity's base material. This
// allows us to "tint" a material by leaving all other properties - like the texture - unchanged,
// and only modifying the base color. The highlighting plugin handles all the work of caching and
// updating these materials when the base material changes, and swapping it out during pointer
// events.
//
// Note that this works for *any* type of asset, not just bevy's built in materials.
const HIGHLIGHT_TINT: Highlight<StandardMaterial> = Highlight {
    hovered: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color.with_alpha(1.0),
        ..matl.to_owned()
    })),
    pressed: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl.base_color.with_alpha(1.0),
        ..matl.to_owned()
    })),
    selected: Some(HighlightKind::new_dynamic(|matl| StandardMaterial {
        base_color: matl
            .base_color
            .mix(&Color::srgba(-0.4, -0.4, 0.8, 0.8), 0.5), // pressed is a different blue
        ..matl.to_owned()
    }))
};


fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::BLACK))
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
                init_tx_antenna_cone_opening,
                init_carrier
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
    commands.spawn(PanOrbitCameraBundle::default());

    // let _world = spawn_world(&mut commands, &mut meshes, &mut materials);
    spawn_world(&mut commands, &mut meshes, &mut materials);

    // Transmitter
    let tx_carrier = spawn_axis_helper(&mut commands, &mut meshes, &mut materials, 150.0);
    let tx_antenna = spawn_axis_helper(&mut commands, &mut meshes, &mut materials, 100.0);
    let tx_antenna_beam = commands.spawn(
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
            PickableBundle::default(), // <- Makes the mesh pickable.
            HIGHLIGHT_TINT,            // Override the global highlighting settings for this mesh
            TxAntennaConeMarker // Add a marker component to Tx Antenna Cone entity
        )
    ).id();

    commands // Antenna cone is the child of tx_antenna_ref...
        .entity(tx_antenna)
        .insert(TxAntennaRefMarker) // Add a marker component to Tx Antenna entity
        .add_child(tx_antenna_beam);
    commands // Which is the child of 
        .entity(tx_carrier)
        .insert(TxCarrierRefMarker) // Add a marker component to Tx Carrier entity
        .add_child(tx_antenna);


    // Transmitter
    let rx_carrier = spawn_axis_helper(&mut commands, &mut meshes, &mut materials, 150.0);
    let rx_antenna = spawn_axis_helper(&mut commands, &mut meshes, &mut materials, 100.0);
    let rx_antenna_beam = commands.spawn(
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
                        base_color: Color::srgba(0.0, 0.0, 0.0, 0.3),
                        alpha_mode: AlphaMode::Blend,
                        unlit: true,
                        ..Default::default()
                    }
                ),
                transform: Transform::from_rotation(Quat::from_rotation_z(FRAC_PI_2)), // Cone along X-axis
                ..Default::default()
            },
            AntennaBeamState::default()
        )
    ).id();
    
    commands
        .entity(rx_antenna)
        .insert(AntennaState::default())
        .add_child(rx_antenna_beam);
    commands
        .entity(rx_carrier)
        .insert(
            CarrierState {
                heading_deg: 90.0,
                elevation_deg: 10.0,
                ..Default::default()
            }
        )
        .insert(Rx)
        .add_child(rx_antenna);
}


fn init_tx_carrier_transform(
    mut query: Query<
        &mut Transform,
        With<TxCarrierRefMarker>
    >
) {
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
    transform.translation = Vec3::new(0.0, 0.0, -0.0);
    transform.rotation = Quat::from_euler(
        EulerRot::ZYX,
        45.0f32.to_radians(),  // Heading
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



fn init_carrier(
    mut query_carrier: Query<(&CarrierState, &mut Transform), With<Rx>>,
    // mut query_antenna: Query<(&AntennaState, &mut Transform), With<Rx>>,
    // mut query_antenna_beam: Query<(&AntennaBeamState, &mut Transform), With<Rx>>,
) {
    let (carrier, mut carrier_transform) = query_carrier.get_single_mut().expect("Can't get `Rx Carrier` transform");
    // let (antenna, mut antenna_transform) = query_antenna.get_single_mut().expect("Can't get `Rx Antenna` transform");
    // let (antenna_beam, mut antenna_beam_transform) = query_antenna_beam.get_single_mut().expect("Can't get `Rx Antenna` transform");

    // Carrier transform
    carrier_transform.translation = Vec3::new(0.0, 0.0, 3000.0);
    carrier_transform.rotation = ENU_TO_NED_ROT.to_owned() * 
        Quat::from_euler(
            EulerRot::ZYX,
            carrier.heading_deg.to_radians() as f32,
            carrier.elevation_deg.to_radians() as f32,
            carrier.bank_deg.to_radians() as f32
    );

    // // Antenna transform
    // antenna_transform.translation = carrier.lever_arms_m.as_vec3();
    // antenna_transform.rotation = Quat::from_euler(
    //     EulerRot::ZYX,
    //     antenna.heading_deg.to_radians() as f32,
    //     antenna.elevation_deg.to_radians() as f32,
    //     antenna.bank_deg.to_radians() as f32
    // );

    // // Antenna beam transform
    // antenna_beam_transform.scale = Vec3::new(
    //     1.0, // Azimuth aperture
    //     1.0,
    //     0.5  // Elevation aperture
    // );
}