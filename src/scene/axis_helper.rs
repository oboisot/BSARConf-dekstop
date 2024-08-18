use bevy::{
    asset::Assets,
    color::Srgba,
    ecs::prelude::Commands,
    math::{
        primitives::{Cone, Cylinder, Sphere},
        Quat, Vec3
    },
    pbr::StandardMaterial,
    prelude::{BuildChildren, Entity, Mesh, Meshable, PbrBundle, ResMut, Transform},
    render::mesh::{ConeAnchor, ConeMeshBuilder, CylinderAnchor, CylinderMeshBuilder}
};
use lazy_static::lazy_static;
use std::f32::consts::FRAC_PI_2;

// https://users.rust-lang.org/t/solved-placement-of-mut-in-function-parameters/19891
pub fn spawn_axis_helper(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    size: f32 // 
) -> Entity {

    commands.spawn(
        // Y-axis base
        PbrBundle {
            mesh: meshes.add( make_cylinder_base(size) ),
            material: materials.add( GREEN_MATERIAL.clone() ),
            ..Default::default()
        }
    ).with_children(|parent| {// Y-axis arrow
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( make_cone_head(size)),
                material: materials.add( GREEN_MATERIAL.clone() ),
                transform: Transform::from_translation(0.9*size*Vec3::Y),
                ..Default::default()
            }
        );
    }).with_children(|parent| {// X-axis base
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( make_cylinder_base(size) ),
                material: materials.add( RED_MATERIAL.clone() ),
                transform: Transform::from_rotation(Quat::from_rotation_z(-FRAC_PI_2)),
                ..Default::default()
            }
        );
    }).with_children(|parent| {// X-axis arrow
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( make_cone_head(size) ),
                material: materials.add( RED_MATERIAL.clone() ),
                transform: Transform::from_rotation(Quat::from_rotation_z(-FRAC_PI_2))
                    .with_translation(0.9*size*Vec3::X),
                ..Default::default()
            }
        );
    }).with_children(|parent| {// Z-axis base
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( make_cylinder_base(size) ),
                material: materials.add( BLUE_MATERIAL.clone() ),
                transform: Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2)),
                ..Default::default()
            }
        );
    }).with_children(|parent| {// Z-axis arrow
        parent.spawn(
            PbrBundle {
                mesh: meshes.add( make_cone_head(size) ),
                material: materials.add( BLUE_MATERIAL.clone() ),
                transform: Transform::from_rotation(Quat::from_rotation_x(FRAC_PI_2))
                    .with_translation(0.9*size*Vec3::Z),
                ..Default::default()
            }
        );
    }).with_children(|parent| {// Origin
        parent.spawn(
            PbrBundle {
                mesh: meshes.add(
                    Sphere {
                        radius: 0.0125 * size
                    }.mesh()
                ),
                material: materials.add( YELLOW_MATERIAL.clone() ),
                ..Default::default()
            }
        );
    })
    // Returns the Entity to allow it to be added to another entity
    .id()
}

lazy_static! {

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

#[inline]
fn make_cylinder_base(size: f32) -> CylinderMeshBuilder {
    Cylinder {
        radius: 0.005 * size,
        half_height: 0.45 * size
    }.mesh()
    .resolution(32)
    .segments(1)
    .anchor(CylinderAnchor::Bottom)
}

#[inline]
fn make_cone_head(size: f32) -> ConeMeshBuilder {
    Cone {
        radius: 0.05 * size,
        height: 0.1 * size
    }.mesh()
    .resolution(32)
    .anchor(ConeAnchor::Base)
}
