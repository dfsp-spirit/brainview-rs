//! Functions for creating a three-d::Mesh ready for rendering in three-d from a neuroformats::BrainMesh.

use std::rc::Rc;

use neuroformats::BrainMesh;
use three_d::{Mesh, CPUMesh, context::Glstruct};
use crate::f32tou32;
use crate::error::{Result};

#[derive(Debug, Clone, PartialEq)]
pub struct ColoredBrainMesh {
    pub mesh : BrainMesh,
    pub vertex_colors: Vec<u8>,
}

impl ColoredBrainMesh {

    pub fn from_brainmesh_and_colors(b_mesh: &BrainMesh, colors: Vec<u8>) -> Result<ColoredBrainMesh> {
        let cb_mesh = ColoredBrainMesh {
            mesh: b_mesh.clone(),
            vertex_colors: colors
        };
        Ok(cb_mesh)
    }

    /// Get the vertex colors as u8 vector. For each vertex, 4 consecutive u8 values represent the red, green, blue, and alpha channel values, respectively.
    pub fn colors_rgba_u8(&self) -> Vec<u8> {
        self.vertex_colors.clone()
    }
}

/// Create a renderable mesh for three-d from a brain mesh and color data.
pub fn mesh_from_colored_brain_mesh(cb_mesh : &ColoredBrainMesh, context: &Rc<Glstruct>) -> Result<three_d::Mesh> {
    let cpu_mesh = CPUMesh {
        positions : cb_mesh.mesh.vertices.clone(),
        colors : Some(cb_mesh.colors_rgba_u8().clone()),
        indices : Some(f32tou32(cb_mesh.mesh.faces.clone())),
        ..Default::default()
    };
    let mesh = Mesh::new(context, &cpu_mesh).unwrap();
    Ok(mesh)
}


