


// The internal state of the Carrier
#[derive(Component)]
pub struct CarrierState {
    /// Carrier orientation in World frame (NED referential)
    pub heading_rad: f64,
    pub elevation_rad: f64,
    pub bank_rad: f64,
    /// Carrier to Antenna phase center lever arms (in NED Carier frame)
    pub lever_arms_m: DVec3,

    pub position_m: f64
}

// The internal state of the Antenna
#[derive(Component)]
pub struct AntennaState {
    /// Antenna orientation relative to Carrier
    pub heading_rad: f64,
    pub elevation_rad: f64,
    pub bank_rad: f64,
}

// The internal state of the Antenna
#[derive(Component)]
pub struct AntennaBeamState {
    /// Antenna 3d beam widths
    pub elevation_beam_width_rad: f64,
    pub azimuth_beam_width_rad: f64,
}

//
#[derive(Component)]
struct Tx;

#[derive(Component)]
struct Rx;


fn update_carrier(mut query: Query<&mut Transform, With<CarrierMarker>>) {
    let mut transform = query
        .get_single_mut()
        .expect("Can't get `TxCarrierRef` transform");

    transform.translation = Vec3::new(-5000.0, 0.0, 3000.0);
    transform.rotation = ENU_TO_NED_ROT.clone();
}

fn update_antenna(mut query: Query<&mut Transform, With<AntennaMarker>>) {
    let mut transform = query
        .get_single_mut()
        .expect("Can't get `TxCarrierRef` transform");

    transform.translation = Vec3::new(352.0, 2.58, -0.53);
    transform.rotation = Quat::from_euler(
        EulerRot::ZYX,
        90.0f32.to_radians(),  // Heading
        -60.0f32.to_radians(), // Elevation
        0.0                    // Bank
    );
}

fn update_antenna_beam(
    mut query: Query<&mut Transform, With<AntennaBeamMarker>>,
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
    mut query_tx: Query<(&mut Transform, CarrierState, AntennaState), With<Tx>>
) {
    let (mut transform, carrier, antenna) = query_tx
                                            .get_single_mut()
                                            .expect("Can't get `Tx Carrier` transform");

    transform.translation = Vec3::new(-5000.0, 0.0, 3000.0);
    transform.rotation = ENU_TO_NED_ROT.clone();
}