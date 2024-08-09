use bevy::{
    ecs::{
        bundle::Bundle,
        component::Component,
        event::EventReader,
        prelude::Query
    },
    input::{
        ButtonInput,
        mouse::{MouseMotion, MouseScrollUnit, MouseWheel, MouseButton}
    },
    math::{Vec2, Vec3, Quat},
    prelude::{Camera3dBundle, Transform, DetectChanges, Res}
};

use std::f32::consts::FRAC_PI_2;

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
    /// Mouse button to hold for panning
    pub pan_button: MouseButton,
    /// Mouse button to hold for orbiting
    pub orbit_button: MouseButton,
    /// For devices with a notched scroll wheel, like desktop mice
    pub scroll_line_sensitivity: f32,
    /// For devices with smooth scrolling, like touchpads
    pub scroll_pixel_sensitivity: f32,
}

impl Default for PanOrbitState {
    fn default() -> Self {
        PanOrbitState {
            center: Vec3::ZERO,
            radius: 35000.0,
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
            pan_button: MouseButton::Right,
            orbit_button: MouseButton::Left,
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
    const PITCH_MAX_RAD: f32 = FRAC_PI_2 * 1.1;

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
        if mbi.pressed(settings.pan_button) {
            total_pan += total_motion * settings.pan_sensitivity;
        }

        let mut total_orbit = Vec2::ZERO;
        if mbi.pressed(settings.orbit_button) {
            total_orbit += total_motion * settings.orbit_sensitivity;
        }

        let mut total_zoom = Vec2::ZERO;
        total_zoom -= total_scroll_lines
            * settings.scroll_line_sensitivity * settings.zoom_sensitivity;
        total_zoom -= total_scroll_pixels
            * settings.scroll_pixel_sensitivity * settings.zoom_sensitivity;

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
            state.center -= transform.right() * total_pan.x * radius;
            state.center += transform.up() * total_pan.y * radius;
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
