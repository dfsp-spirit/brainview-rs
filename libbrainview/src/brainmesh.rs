//! Datastructures modeling brain meshes.

use neuroformats::BrainMesh;
use crate::{color_from_data, error::{Result}};

/// Models a vertex-colored BrainMesh.
#[derive(Debug, Clone, PartialEq)]
pub struct ColoredBrainMesh {
    pub mesh : BrainMesh,
    pub vertex_colors: Vec<u8>,
}

impl ColoredBrainMesh {

    /// Construct a ColoredBrainMesh from a BrainMesh with n vertices and vertex colors given as n*4 u8 values representing RGBA color values for each vertex.
    pub fn from_brainmesh_and_colors(b_mesh: &BrainMesh, colors: Vec<u8>) -> Result<ColoredBrainMesh> {
        let cb_mesh = ColoredBrainMesh {
            mesh: b_mesh.clone(),
            vertex_colors: colors
        };
        Ok(cb_mesh)
    }

    /// Construct a ColoredBrainMesh from a BrainMesh and per-vertex data.
    pub fn from_brainmesh_and_data(b_mesh: &BrainMesh, data: Vec<f32>) -> Result<ColoredBrainMesh> {
        let cb_mesh = ColoredBrainMesh {
            mesh: b_mesh.clone(),
            vertex_colors: color_from_data(data)
        };
        Ok(cb_mesh)
    }

    /// Get the vertex colors as u8 vector. For each vertex, 4 consecutive u8 values represent the red, green, blue, and alpha channel values, respectively.
    pub fn colors_rgba_u8(&self) -> Vec<u8> {
        self.vertex_colors.clone()
    }
}

