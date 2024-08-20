


// The internal state of the pan-orbit controller
#[derive(Component)]
pub struct SarState {
    pub center: Vec3,
    pub z_focus: f32,
    pub radius: f32,
    pub pitch: f32,
    pub yaw: f32,
}

//
#[derive(Component)]
struct TxCarrierRefMarker;

#[derive(Component)]
struct TxAntennaRefMarker;

#[derive(Component)]
struct TxAntennaConeMarker;

// #[derive(Component)]
// struct RxCarrierRefMarker;