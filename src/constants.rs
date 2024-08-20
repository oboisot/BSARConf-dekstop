use bevy::{
    color::LinearRgba,
    math::{Quat, Vec3, Mat3},
    pbr::StandardMaterial
};

use lazy_static::lazy_static;

lazy_static! {
    /// Material constants

    pub static ref RED_MATERIAL: StandardMaterial = StandardMaterial {
        base_color: LinearRgba::RED.into(),
        unlit: true,
        ..Default::default()
    };

    pub static ref GREEN_MATERIAL: StandardMaterial = StandardMaterial {
        base_color: LinearRgba::GREEN.into(),
        unlit: true,
        ..Default::default()
    };

    pub static ref BLUE_MATERIAL: StandardMaterial = StandardMaterial {
        base_color: LinearRgba::BLUE.into(),
        unlit: true,
        ..Default::default()
    };

    pub static ref YELLOW_MATERIAL: StandardMaterial = StandardMaterial {
        base_color: LinearRgba::new(1.0, 1.0, 0.0, 1.0).into(),
        unlit: true,
        ..Default::default()
    };
}


lazy_static!(
    /// Geometric constants

    pub static ref ENU_TO_NED_ROT: Quat = Quat::from_mat3(&Mat3 { // ENU -> NED rotation
        x_axis: Vec3::Y,
        y_axis: Vec3::X,
        z_axis: -Vec3::Z
    });

);
