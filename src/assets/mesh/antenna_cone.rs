use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};

/// A cone which stands on the YZ plane with
/// vertical axis in the X axis and cone apex at 
/// the origin.
#[derive(Clone, Copy, Debug)]
pub struct Cone {
    /// Base radius of the cone in the XY plane.
    pub radius: f32,
    /// Height of the cone in the Z axis.
    pub height: f32,
    /// Number of radial segments of the cone circle(s). Must be greater or equal to 3.
    pub radial_segments: usize,
    /// Number of height segments. Must be greater or equal to 1.
    pub height_segments: usize,
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            radius: 1.0f32,
            height: 1.0f32,
            radial_segments: 36usize,
            height_segments: 1usize,
        }
    }
}

impl From<Cone> for Mesh {
    fn from(cone: Cone) -> Self {        
        debug_assert!(cone.radius > 0.0);
        debug_assert!(cone.height > 0.0);
        debug_assert!(cone.radial_segments >= 3);
        debug_assert!(cone.height_segments >= 1);

        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(cone.radial_segments * cone.height_segments);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(cone.radial_segments * cone.height_segments);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(cone.radial_segments * cone.height_segments);
        let mut indices: Vec<u32> = Vec::with_capacity(cone.radial_segments * (1 + 2 * (cone.height_segments - 1)));

        // Helper variables
        let inv_height_segments = 1.0 / (cone.height_segments as f32);
        let inv_radial_segments = 1.0 / (cone.radial_segments as f32);
        let theta_step = std::f32::consts::TAU * inv_radial_segments;
        let inv_radial_length = 1.0 / (cone.height.hypot(cone.radius)); // 1/sqrt(H² + R²)
        let cos_alpha = cone.height * inv_radial_length;
        let sin_alpha = cone.radius * inv_radial_length;

        for k in 0..cone.height_segments {
            let v = (k as f32) * inv_height_segments; // (u,V) coordinate
            let height_radius = v * cone.radius; // radius of the current height segment

            for i in 0..cone.radial_segments {
                let u = (i as f32) * inv_radial_segments; // (U,v) coordinate
                let (sin_theta, cos_theta) = ((i as f32) * theta_step).sin_cos();

                // Vertex
                let x = height_radius;
                let y = height_radius * cos_theta;
                let z = height_radius * sin_theta;
                vertices.push([x, y, z]);

                // Normal
                let x = -sin_alpha;
                let y = cos_alpha * cos_theta;
                let z = cos_alpha * sin_theta;                
                normals.push([x, y, z]);

                // uv
                uvs.push([u, v]);
            }
        }

        for i in 0..cone.radial_segments {
            indices.push(0);
            indices.push(i as u32 + 1);
            indices.push(i as u32 + 2);
        }

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_indices(Some(Indices::U32(indices)))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}
