//! Functions that adapt brain meshes to three-d data structures for rendering.

use std::rc::Rc;

use three_d::{Mesh, CPUMesh, context::Glstruct};
use crate::{f32tou32, ColoredBrainMesh};
use crate::error::{Result};


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

