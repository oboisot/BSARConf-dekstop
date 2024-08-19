use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        event::EventReader,
        prelude::Query
    },
    input::{
        mouse::{MouseButton, MouseMotion, MouseScrollUnit, MouseWheel}, ButtonInput
    },
    math::{Quat, Vec2, Vec3},
    prelude::{Camera3dBundle, DetectChanges, Res, Transform}
};

use std::f32::consts::PI;

// see: https://bevy-cheatbook.github.io/cookbook/pan-orbit-camera.html

// Bundle to spawn our custom camera easily
#[derive(Bundle, Default)]
pub struct PanOrbitCameraBundle {
    pub camera: Camera3dBundle,
    pub state: PanOrbitState,
    pub settings: PanOrbitSettings,
}

// The internal state of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitState {
    pub center: Vec3,
    pub z_focus: f32,
    pub radius: f32,
    pub pitch: f32,
    pub yaw: f32,
}

/// The configuration of the pan-orbit controller
#[derive(Component)]
pub struct PanOrbitSettings {
    /// World units per pixel of mouse motion
    pub pan_sensitivity: f32,
    /// Radians per pixel of mouse motion
    pub orbit_sensitivity: f32,
    /// Exponent per pixel of mouse motion
    pub zoom_sensitivity: f32,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
}

impl Default for PanOrbitState {
    fn default() -> Self {
        PanOrbitState {
            center: Vec3::ZERO,
            z_focus: 0.0f32,
            radius: 25000.0f32,
            pitch: 60.0f32.to_radians(),
            yaw: 45.0f32.to_radians(),
        }
    }
}

impl Default for PanOrbitSettings {
    fn default() -> Self {
        PanOrbitSettings {
            pan_sensitivity: 0.001, // 1000 pixels per world unit
            orbit_sensitivity: 0.2f32.to_radians(), // 0.5 degree per pixel
            zoom_sensitivity: 0.01,
            scroll_line_sensitivity: 16.0, // 1 "line" == 16 "pixels of motion"
            scroll_pixel_sensitivity: 1.0,
        }
    }
}

pub fn pan_orbit_camera(
    mbi: Res<ButtonInput<MouseButton>>,
    mut evr_motion: EventReader<MouseMotion>,
    mut evr_scroll: EventReader<MouseWheel>,
    mut q_camera: Query<(
        &PanOrbitSettings,
        &mut PanOrbitState,
        &mut Transform,
    )>,
) {
    const PITCH_MAX_RAD: f32 = 100.0 * PI / 180.0;

    // First, accumulate the total amount of
    // mouse motion and scroll, from all pending events:
    let total_motion: Vec2 = evr_motion.read().map(|ev| ev.delta).sum();
    // Scroll
    let mut total_scroll_lines = Vec2::ZERO;
    let mut total_scroll_pixels = Vec2::ZERO;
    for ev in evr_scroll.read() {
        match ev.unit {
            MouseScrollUnit::Line => {
                total_scroll_lines.x += ev.x;
                total_scroll_lines.y -= ev.y;
            }
            MouseScrollUnit::Pixel => {
                total_scroll_pixels.x += ev.x;
                total_scroll_pixels.y -= ev.y;
            }
        }
    }

    for (settings, mut state, mut transform) in &mut q_camera {
        // Check how much of each thing we need to apply.
        // Accumulate values from motion and scroll,
        // based on our configuration settings.

        let mut total_pan = Vec2::ZERO;
        if mbi.pressed(MouseButton::Right) {
            total_pan += total_motion * settings.pan_sensitivity;
        }

        let mut total_orbit = Vec2::ZERO;
        if mbi.pressed(MouseButton::Left) {
            total_orbit += total_motion * settings.orbit_sensitivity;
        }

        let mut total_zoom = Vec2::ZERO;
        total_zoom -= total_scroll_lines * settings.scroll_line_sensitivity * settings.zoom_sensitivity;
        total_zoom -= total_scroll_pixels * settings.scroll_pixel_sensitivity * settings.zoom_sensitivity;

        let mut any = false;
        if total_zoom != Vec2::ZERO {
            any = true;
            state.radius *= (-total_zoom.y).exp();
        }

        // To ORBIT, we change our pitch and yaw values
        if total_orbit != Vec2::ZERO {
            any = true;
            state.yaw -= total_orbit.x;
            state.pitch -= total_orbit.y;
            // Limits pitch angles
            if state.pitch >= PITCH_MAX_RAD {
                state.pitch = PITCH_MAX_RAD;
            }
            if state.pitch <= 0.0  {
                state.pitch = 0.0;
            }
        }        

        if total_pan != Vec2::ZERO {
            any = true;
            let radius = state.radius;
            // Used to compensate the Up axis projection in the world Y-axis relative to pitch angle.
            // Up norm projected onto the Y-axis is: norm(up)|Y = norm(up)*cos(pitch)
            // When pitch is close to 90° it goes down to zero, so we compensate this value to allow
            // a y panning which keeps its moving speed.
            // Furhermore, the sign change in cos when pitch is greater than 90° keeps the plane
            // movement correct
            let mut cpitch = state.pitch.cos();
            if cpitch == 0.0 {
                cpitch = 1.0
            }
            state.center -= transform.right() * total_pan.x * radius;       // note: minus sign because center is moved contrary to the horizontal movement
            state.center += transform.up() * total_pan.y * radius / cpitch; // note: plus sign becaus vertical movement is inverted on screen (screen y is positive downside)
            state.center.z = state.z_focus;
        }

        if any || state.is_added() {
            // Camera referential is: X - right, Y - up, Z - out of screen.
            // Note: What is "seen on screen" is like if the camera was rotated then put back in its
            //       initial state, so that what is drawn on screen is moved.
            transform.rotation = Quat::from_rotation_z(state.yaw)
                * Quat::from_rotation_x(state.pitch); // Rx'(pitch)*Rz(yaw) intrisinc <=> Rz(yaw)*Ry(pitch) extrinsic
            transform.translation = state.center + transform.back() * state.radius;
        }
    }
}
