use bevy::{render::{mesh::{Indices, Mesh, PrimitiveTopology}, render_asset::RenderAssetUsages}};
use bevy::color::LinearRgba;

/// A builder used for creating a [`Mesh`] with a [`Cone`] shape.
#[derive(Clone, Copy, Debug)]
pub struct Arrow {
    /// Arrow length
    pub length: f32,
    /// 
    pub head_length: f32,
    ///
    pub head_width: f32,
    ///
    pub color: LinearRgba
}

impl Default for Arrow {
    fn default() -> Self {
        Self {
            length: 1.0,
            head_length: 0.1,
            head_width: 0.025,
            color: LinearRgba::WHITE
        }
    }
}
