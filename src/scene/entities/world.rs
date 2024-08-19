use bevy::{
    asset::Assets,
    color::LinearRgba,
    ecs::prelude::Commands,
    math::{
        primitives::Plane3d,
        Vec3, Vec2
    },
    pbr::StandardMaterial,
    prelude::{BuildChildren, Entity, Mesh, Meshable, PbrBundle, ResMut, Transform}
};

use crate::{
    scene::entities::spawn_axis_helper,
    mesh::LineList
};

const HALF_PLANE_SIZE: f32 = 15000.0;
const GRID_SIZE: f32 = 500.0;

pub fn spawn_world(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {

    // opaque plane
    let world_plane = commands.spawn(
        PbrBundle {
            mesh: meshes.add(
                Plane3d::new(
                    Vec3::Z,
                    Vec2::splat(HALF_PLANE_SIZE)
                ).mesh()
                .subdivisions(0)
            ),
            material: materials.add(
                StandardMaterial {
                    base_color: LinearRgba::rgb(0.545098039, 0.537254902, 0.537254902).into(),
                    // reflectance: 0.0,
                    unlit: true,
                    ..Default::default()
            }),
            transform: Transform::from_translation( // ensure that what is draw on the plane is correctly seen
                Vec3::new(0.0, 0.0, -0.1)
            ),
            ..Default::default()
        }
    ).id();

    // Plane grid
    let half_num_lines = (HALF_PLANE_SIZE / GRID_SIZE).floor() as usize;
    let mut lines = Vec::<(Vec3, Vec3)>::with_capacity(4 * half_num_lines);
    // X-lines
    let mut y: f32;
    for i in 1..=half_num_lines {
        y = GRID_SIZE * i as f32;
        lines.push(
            (Vec3::new(-HALF_PLANE_SIZE, y, 0.0), Vec3::new(HALF_PLANE_SIZE, y, 0.0))
        );
        lines.push(
            (Vec3::new(-HALF_PLANE_SIZE, -y, 0.0), Vec3::new(HALF_PLANE_SIZE, -y, 0.0))
        );
    }
    // Y-lines
    let mut x: f32;
    for i in 1..=half_num_lines {
        x = GRID_SIZE * i as f32;
        lines.push(
            (Vec3::new(x, -HALF_PLANE_SIZE, 0.0), Vec3::new(x, HALF_PLANE_SIZE, 0.0))
        );
        lines.push(
            (Vec3::new(-x, -HALF_PLANE_SIZE, 0.0), Vec3::new(-x, HALF_PLANE_SIZE, 0.0))
        );
    }

    let world_grid = commands.spawn(
        PbrBundle {
            mesh: meshes.add(
                LineList { lines }
            ),
            material: materials.add(
                StandardMaterial {
                    // base_color: LinearRgba::rgb(0.737254902, 0.737254902, 0.737254902).into(),
                    base_color: LinearRgba::rgb(0.325490196, 0.407843137, 0.470588235).into(),
                    unlit: true,
                    ..Default::default()
            }),
            ..Default::default()
        }
    ).id();

    // Center X-line
    let center_x_line = commands.spawn(
        PbrBundle {
            mesh: meshes.add(
                LineList { 
                    lines: vec![(Vec3::new(-HALF_PLANE_SIZE, 0.0, 0.0), Vec3::new(HALF_PLANE_SIZE, 0.0, 0.0))]
                }
            ),
            material: materials.add(
                StandardMaterial {
                    base_color: LinearRgba::RED.into(),
                    unlit: true,
                    ..Default::default()
            }),
            ..Default::default()
        }
    ).id();

    // Center Y-line
    let center_y_line = commands.spawn(
        PbrBundle {
            mesh: meshes.add(
                LineList { 
                    lines: vec![(Vec3::new(0.0, -HALF_PLANE_SIZE, 0.0), Vec3::new(0.0, HALF_PLANE_SIZE, 0.0))]
                }
            ),
            material: materials.add(
                StandardMaterial {
                    base_color: LinearRgba::GREEN.into(),
                    unlit: true,
                    ..Default::default()
            }),
            ..Default::default()
        }
    ).id();

    // World axis helper
    let world_axis_helper = spawn_axis_helper(
        commands,
        meshes,
        materials,
        500.0
    );

    commands
        .entity(world_plane)
        .push_children(&[
            world_grid,
            center_x_line,
            center_y_line,
            world_axis_helper
        ])
        .id()
}