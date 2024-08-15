use bevy::{
    asset::Assets,
    color::Srgba,
    ecs::prelude::Commands,
    math::{
        primitives::{Cone, Cylinder, Sphere},
        Quat, Vec3
    },
    pbr::StandardMaterial,
    prelude::{BuildChildren, ChildBuilder, Mesh, Meshable, PbrBundle, ResMut, Transform},
    render::mesh::{ConeAnchor, ConeMeshBuilder, CylinderAnchor, CylinderMeshBuilder, SphereMeshBuilder}
};
use lazy_static::lazy_static;
use std::f32::consts::FRAC_PI_2;

lazy_static! {
    static ref CYLINDER_BASE: CylinderMeshBuilder = Cylinder {
        radius: 0.005,
        half_height: 0.45
    }.mesh()
    .resolution(8)
    .segments(1)
    .anchor(CylinderAnchor::Bottom);

    static ref CONE_HEAD: ConeMeshBuilder = Cone {
        radius: 0.05, 
        height: 0.1
    }.mesh()
    .resolution(8)
    .anchor(ConeAnchor::Base);

    static ref SPHERE: SphereMeshBuilder = Sphere {
        radius: 0.0125
    }.mesh();

    static ref RED_MATERIAL: StandardMaterial = StandardMaterial {
        base_color: Srgba::RED.into(),
        reflectance: 0.0,
        ..Default::default()
    };

    static ref GREEN_MATERIAL: StandardMaterial = StandardMaterial {
        base_color: Srgba::GREEN.into(),
        reflectance: 0.0,
        ..Default::default()
    };

    static ref BLUE_MATERIAL: StandardMaterial = StandardMaterial {
        base_color: Srgba::BLUE.into(),
        reflectance: 0.0,
        ..Default::default()
    };

    static ref YELLOW_MATERIAL: StandardMaterial = StandardMaterial {
        base_color: Srgba::new(1.0, 1.0, 0.0, 1.0).into(),
        reflectance: 0.0,
        ..Default::default()
    };
}

// #[derive(Bundle)]
// pub struct AxisHelperBundle {
//     pub transform: Transform,
//     pub
// }


// https://users.rust-lang.org/t/solved-placement-of-mut-in-function-parameters/19891
pub fn axis_helper_commands_spawn(
    commands: &mut Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    scale: f32
) {
    commands.spawn(
        // Y-axis base
        PbrBundle {
            mesh: meshes.add( CYLINDER_BASE.clone() ),
            material: materials.add( GREEN_MATERIAL.clone() ),
            transform: Transform::from_scale(Vec3::splat(scale)),
            ..Default::default()
        }
    ).with_children(|parent| {// Y-axis arrow
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( CONE_HEAD.clone() ),
                material: materials.add( GREEN_MATERIAL.clone() ),
                transform: Transform::from_translation(0.9*Vec3::Y),
                ..Default::default()
            }
        );
    }).with_children(|parent| {// X-axis base
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( CYLINDER_BASE.clone() ),
                material: materials.add( RED_MATERIAL.clone() ),
                transform: Transform::from_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
                ..Default::default()
            }
        );
    }).with_children(|parent| {// X-axis arrow
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( CONE_HEAD.clone() ),
                material: materials.add( RED_MATERIAL.clone() ),
                transform: Transform::from_rotation(Quat::from_rotation_z(-FRAC_PI_2))
                    .with_translation(0.9*Vec3::X),
                ..Default::default()
            }
        );
    }).with_children(|parent| {// Z-axis base
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( CYLINDER_BASE.clone() ),
                material: materials.add( BLUE_MATERIAL.clone() ),
                transform: Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2)),
                ..Default::default()
            }
        );
    }).with_children(|parent| {// Z-axis arrow
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( CONE_HEAD.clone() ),
                material: materials.add( BLUE_MATERIAL.clone() ),
                transform: Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2))
                    .with_translation(0.9*Vec3::Z),
                ..Default::default()
            }
        );
    }).with_children(|parent| {// Origin
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( SPHERE.clone() ),
                material: materials.add( YELLOW_MATERIAL.clone() ),
                ..Default::default()
            }
        );
    });
}
