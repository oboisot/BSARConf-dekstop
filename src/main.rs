//! Demonstrates how to use transparency in 3D.
//! Shows the effects of different blend modes.
//! The `fade_transparency` system smoothly changes the transparency over time.

use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCameraPlugin, PanOrbitCamera};
use std::f32::consts::TAU;

// use bevy::input::mouse::{MouseWheel,MouseMotion};
// use bevy::render::camera::Projection;
// use bevy::window::PrimaryWindow;

fn main() {
    App::new()
        .insert_resource(Msaa::default())
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(AmbientLight {color: Color::WHITE, brightness: 1.0})
        .add_plugins((DefaultPlugins, PanOrbitCameraPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, axes)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // opaque plane, uses `alpha_mode: Opaque` by default
    commands.spawn(PbrBundle {
        mesh: meshes.add(shape::Plane::from_size(30000.0).into()),
        material: materials.add(Color::hex("8b8989").unwrap().into()),
        transform: Transform::from_rotation(
            Quat::from_mat3(&Mat3{ x_axis: Vec3::Y, y_axis: Vec3::Z, z_axis: Vec3::X }) // Set plane in the XY-plane (rather than XZ-plane)
        ),
        ..default()
    });
    // opaque sphere
    commands.spawn(PbrBundle {
        mesh: meshes.add(
            Mesh::try_from(shape::Icosphere {
                radius: 500.0,
                subdivisions: 3,
            })
            .unwrap(),
        ),
        material: materials.add(Color::rgba(0.7, 0.2, 0.1, 0.5).into()),
        transform: Transform::from_xyz(0.0, 0.0, 1000.0).with_scale(Vec3::new(1.0, 1.0, 0.5)),
        ..default()
    });
    // Camera
    commands.spawn((
        // Note we're setting the initial position below with alpha, beta, and radius, hence
        // we don't set transform on the camera.
        Camera3dBundle::default(),
        PanOrbitCamera {
            // Set focal point (what the camera should look at)
            focus: Vec3::ZERO,
            // Set the starting position, relative to focus (overrides camera's transform).
            alpha: Some(TAU / 8.0),
            beta: Some(TAU / 8.0),
            radius: Some(20000.0),
            // Set limits on rotation and zoom
            // alpha_upper_limit: Some(TAU / 4.0),
            // alpha_lower_limit: Some(-TAU / 4.0),
            // beta_upper_limit: Some(TAU / 3.0),
            // beta_lower_limit: Some(-TAU / 3.0),
            // zoom_upper_limit: Some(5.0),
            // zoom_lower_limit: Some(1.0),
            // Adjust sensitivity of controls
            orbit_sensitivity: 1.5,
            pan_sensitivity: 0.5,
            zoom_sensitivity: 0.5,
            // Allow the camera to go upside down
            allow_upside_down: false,
            // Change the controls (these match Blender)
            button_orbit: MouseButton::Left,
            button_pan: MouseButton::Right,
            modifier_pan: None,
            // Reverse the zoom direction
            reversed_zoom: false,
            ..default()
        },
    ));
}

fn axes(mut gizmos: Gizmos) {
    gizmos.line(Vec3::ZERO, Vec3::X * 500.0, Color::RED);    // World X-axis
    gizmos.line(Vec3::ZERO, Vec3::Y * 500.0, Color::GREEN);  // World Y-axis
    gizmos.line(Vec3::ZERO, Vec3::Z * 500.0, Color::BLUE);   // World Z-axis
}

// /// Spawn a camera
// fn spawn_camera(mut commands: Commands) {
//     let translation = Vec3::new(5e3, -10e3, 10e3);
//     let radius = translation.length();
//     commands.spawn((
//         Camera3dBundle {
//             transform: Transform::from_translation(translation)
//                 .looking_at(Vec3::ZERO, Vec3::Z),
//             ..Default::default()
//         },
//         PanOrbitCamera {
//             radius,
//             ..Default::default()
//         },
//     ));
// }

// /// Tags an entity as capable of panning and orbiting.
// #[derive(Component)]
// struct PanOrbitCamera {
//     /// The "focus point" to orbit around. It is automatically updated when panning the camera
//     pub focus: Vec3,
//     pub radius: f32,
//     pub upside_down: bool,
// }

// impl Default for PanOrbitCamera {
//     fn default() -> Self {
//         PanOrbitCamera {
//             focus: Vec3::ZERO,
//             radius: 5.0,
//             upside_down: false,
//         }
//     }
// }

// /// Pan the camera with middle mouse click, zoom with scroll wheel, orbit with right mouse click.
// fn pan_orbit_camera(
//     mut ev_motion: EventReader<MouseMotion>,
//     mut ev_scroll: EventReader<MouseWheel>,    
//     mut orbit_query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>,
//     window: Query<&Window, With<PrimaryWindow>>,
//     input_mouse: Res<Input<MouseButton>>
// ) {
//     // change input mapping for orbit and panning here
//     let orbit_button = MouseButton::Left;
//     let pan_button = MouseButton::Right;

//     let mut pan = Vec2::ZERO;
//     let mut rotation_move = Vec2::ZERO;
//     let mut scroll = 0.0;
//     let mut orbit_button_changed = false;

//     if input_mouse.pressed(orbit_button) {
//         for ev in ev_motion.iter() {
//             rotation_move += ev.delta;
//         }
//     } else if input_mouse.pressed(pan_button) {
//         // Pan only if we're not rotating at the moment
//         for ev in ev_motion.iter() {
//             pan += ev.delta;
//         }
//     }
//     for ev in ev_scroll.iter() {
//         scroll += ev.y;
//     }
//     if input_mouse.just_released(orbit_button) || input_mouse.just_pressed(orbit_button) {
//         orbit_button_changed = true;
//     }

//     for (mut pan_orbit, mut transform, projection) in orbit_query.iter_mut() {
//         if orbit_button_changed {
//             // only check for upside down when orbiting started or ended this frame
//             // if the camera is "upside" down, panning horizontally would be inverted, so invert the input to make it correct
//             let up = transform.rotation * Vec3::Z;
//             pan_orbit.upside_down = up.z <= 0.0;
//         }

//         // Get primary window size
        
//         let window = get_primary_window_size(&window);
//         let mut any = false;
//         if rotation_move.length_squared() > 0.0 {
//             any = true;            
//             let delta_x = {
//                 let delta = rotation_move.x / window.x * std::f32::consts::TAU;
//                 if pan_orbit.upside_down { -delta } else { delta }
//             };
//             let delta_y = rotation_move.y / window.y * std::f32::consts::PI;
//             let yaw = Quat::from_rotation_z(-delta_x);
//             let roll = Quat::from_rotation_x(-delta_y);
//             transform.rotation = yaw * transform.rotation; // rotate around global z axis
//             transform.rotation = transform.rotation * roll; // rotate around local x axis
//         } else if pan.length_squared() > 0.0 {
//             any = true;
//             // make panning distance independent of resolution and FOV,
//             if let Projection::Perspective(projection) = projection {
//                 pan *= Vec2::new(projection.fov * projection.aspect_ratio, projection.fov) / window;
//             }
//             // translate by local axes
//             let right = transform.rotation * Vec3::X * -pan.x;
//             let up = transform.rotation * Vec3::Y * pan.y;
//             // make panning proportional to distance away from focus point
//             let translation = (right + up) * pan_orbit.radius;
//             pan_orbit.focus += translation;
//         } else if scroll.abs() > 0.0 {
//             any = true;
//             pan_orbit.radius -= scroll * pan_orbit.radius * 0.2;
//             // dont allow zoom to reach zero or you get stuck
//             pan_orbit.radius = f32::max(pan_orbit.radius, 0.05);
//         }

//         if any {
//             // emulating parent/child to make the yaw/z-axis rotation behave like a turntable
//             // parent = x and z rotation
//             // child = y-offset
//             let rot_matrix = Mat3::from_quat(transform.rotation);
//             transform.translation = pan_orbit.focus + rot_matrix.mul_vec3(Vec3::new(0.0, 0.0, pan_orbit.radius));
//         }
//     }

//     // consume any remaining events, so they don't pile up if we don't need them
//     // (and also to avoid Bevy warning us about not checking events every frame update)
//     ev_motion.clear();
// }

// fn get_primary_window_size(window: &Query<&Window, With<PrimaryWindow>>) -> Vec2 {
//     let window = window.single(); // Get the primary window size
//     Vec2::new(window.width(), window.height())
// }
