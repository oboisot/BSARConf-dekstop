use bevy::render::mesh::{Indices, Mesh, PrimitiveTopology};

/// A cone which stands on the YZ plane with
/// vertical axis in the X axis.
#[derive(Clone, Copy, Debug)]
pub struct Cone {
    /// Base radius of the cone in the XY plane.
    pub radius: f32,
    /// Height of the cone in the Z axis.
    pub height: f32,
    /// Number of radial segments of the cone circle(s)
    pub radial_segments: usize,
    /// Number of height segments
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
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(cone.radial_segments * cone.height_segments);
        let mut normals: Vec<[f32; 3]> = Vec::with_capacity(cone.radial_segments * cone.height_segments);
        let mut uvs: Vec<[f32; 2]> = Vec::with_capacity(cone.radial_segments * cone.height_segments);
        let mut indices: Vec<u32> = Vec::with_capacity(cone.radial_segments * cone.height_segments * 2 * 3);

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
                let nx = -sin_alpha;
                let ny = cos_alpha * cos_theta;
                let nz = cos_alpha * sin_theta;                
                normals.push([nx, ny, nz]);

                // uv
                uvs.push([u, v]);
            }
        }

        Mesh::new(PrimitiveTopology::TriangleList)
            .with_indices(Some(Indices::U32(indices)))
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vertices)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    }
}
