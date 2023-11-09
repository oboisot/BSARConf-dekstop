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
    pub radial_segments: u32,
    /// Number of height segments. Must be greater or equal to 1.
    pub height_segments: u32,
    /// If true, the cone will be drawn as wireframe
    pub wireframe: bool
}

impl Default for Cone {
    fn default() -> Self {
        Self {
            radius: 1.0,
            height: 1.0,
            radial_segments: 36,
            height_segments: 1,
            wireframe: false
        }
    }
}

impl From<Cone> for Mesh {
    fn from(cone: Cone) -> Self {        
        debug_assert!(cone.radius > 0.0, "Cone 'radius' must be strictly positive");
        debug_assert!(cone.height > 0.0, "Cone 'height' must be strictly positive");
        debug_assert!(cone.radial_segments >= 3, "Cone 'radial_segments' must be greater or equal to 3");
        debug_assert!(cone.height_segments >= 1, "Cone 'height_segments' must be greater or equal to 1");

        let num_vertices = (cone.radial_segments + 1) * cone.height_segments + 1;
        let num_indices  = if cone.wireframe {
            2 * ((num_vertices - 1) + cone.height_segments * (cone.radial_segments - 1))
        } else {
            3 * cone.radial_segments * (1 + 2 * (cone.height_segments - 1))
        };
        let mut vertices: Vec<[f32; 3]> = Vec::with_capacity(num_vertices as usize);
        let mut normals:  Vec<[f32; 3]> = Vec::with_capacity(num_vertices as usize);
        let mut uvs:      Vec<[f32; 2]> = Vec::with_capacity(num_vertices as usize);
        let mut indices:  Vec<u32>      = Vec::with_capacity(num_indices as usize);

        // Helper variables
            // Inverse segments numbers
        let inv_height_segments = 1.0 / (cone.height_segments as f32);
        let inv_radial_segments = 1.0 / (cone.radial_segments as f32);
            // loop steps
        let height_step = cone.height * inv_height_segments;
        let theta_step = std::f32::consts::TAU * inv_radial_segments;
            // 
        let tan_alpha  = cone.radius / cone.height; // Cone tangent of its half opening angle (= sin_alpha / cos_alpha)
        let inv_radial_length = 1.0 / (cone.height.hypot(cone.radius)); // 1/sqrt(H² + R²)
        let cos_alpha  = cone.height * inv_radial_length;
        let sin_alpha  = cone.radius * inv_radial_length;

        // Apex
        vertices.push([0.0, 0.0, 0.0]);
        normals.push([-1.0, 0.0, 0.0]);
        uvs.push([0.0, 0.0]);
        // vertices, normals and uvs
        for k in 1..=cone.height_segments {
            let v = (k as f32) * inv_height_segments; // (u,V) coordinate
            let height = (k as f32) * height_step; // radius of the current height segment
            let radius = tan_alpha * height;

            for i in 0..=cone.radial_segments {
                let u = (i as f32) * inv_radial_segments; // (U,v) coordinate
                let (sin, cos) = ((i as f32) * theta_step).sin_cos();

                // Vertex
                vertices.push([height,
                               radius * cos,
                               radius * sin]);
                // Normal            
                normals.push([-sin_alpha,
                              cos_alpha * cos,
                              cos_alpha * sin]);
                // uv
                uvs.push([u, v]);
            }
        }

        // indices
        if cone.wireframe {
            for i in 0..num_vertices - 1 {
                indices.extend_from_slice(&[i, i+1]);
            }

            let nrs = cone.radial_segments + 1;
            for i in 2..=cone.radial_segments {
                indices.extend_from_slice(&[0, i]);
                for k in 0..cone.height_segments - 1 {
                    indices.extend_from_slice(&[i + k * nrs, i + (k + 1) * nrs]);
                }
            }
        } else {
            for i in 1..=cone.radial_segments {
                indices.extend_from_slice(&[0, i, i + 1]);
            }
            
            if cone.height_segments >= 2 {
                let ring_step     = cone.radial_segments + 1;
                let mut ring      = 1;
                let mut next_ring = ring + ring_step;
                for _k in 1..cone.height_segments {            
                    for i in 0..cone.radial_segments {
                        indices.extend_from_slice(&[
                            ring + i,
                            next_ring + i,
                            ring + 1 + i,
                            ring + 1 + i,
                            next_ring + i,
                            next_ring + 1 + i     
                        ]);
                    }
                    ring       = next_ring;
                    next_ring += cone.radial_segments + 1;
                }
            }
        }

        // println!("indices.capacity(): {}", indices.capacity());
        // println!("indices.size():     {}", indices.len());
        // println!("indices:            {:?}", indices);
        // println!("vertices.capacity():{}", vertices.capacity());
        // println!("vertices.len():     {}", vertices.len());
        // println!("vertices:           {:?}", vertices);
        // println!("uvs.len():          {}", uvs.len());
        // println!("uvs:                {:?}", uvs);

        let mut mesh = if cone.wireframe {
            Mesh::new(PrimitiveTopology::LineList)
        } else {
            Mesh::new(PrimitiveTopology::TriangleList)
        };

        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, uvs);
        mesh.set_indices(Some(Indices::U32(indices)));
        mesh
    }
}
