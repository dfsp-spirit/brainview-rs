//! Datastructures modeling brain meshes.

use std::path::{Path};

use neuroformats::{BrainMesh, read_curv, read_surf, read_annot, read_label};
use crate::{FsLabelDisplay, color_from_data, error::{Result}};
use crate::vertexcolor::VertexColor;

/// Models a vertex-colored BrainMesh, typically for a single hemisphere.
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


    /// Construct a ColoredBrainMesh from morphometry data files in a FreeSurfer directory. This typically represents a single hemisphere.
    pub fn from_freesurfer_dir(base_path : &str, surface_file : &str, morph_file: &str) -> Result<ColoredBrainMesh> {
        let base_path : &Path = &Path::new(base_path);
        let surface_file : &Path = &Path::new(surface_file);
        let morph_file : &Path = &Path::new(morph_file);
        let surface_file = base_path.join(&Path::new("surf")).join(surface_file);
        let morph_file = base_path.join(&Path::new("surf")).join(morph_file);
        
        let surface = read_surf::<&Path>(&surface_file).unwrap();
        let curv = read_curv::<&Path>(&morph_file).unwrap();
        let cb_mesh = ColoredBrainMesh {
            mesh: surface.mesh.clone(),
            vertex_colors: color_from_data(curv.data)
        };
        Ok(cb_mesh)
    }


    /// Construct a ColoredBrainMesh from brain atlas surface parcellation files in a FreeSurfer directory. This typically represents a single hemisphere.
    pub fn from_freesurfer_annot(base_path : &str, surface_file : &str, annot_file: &str) -> Result<ColoredBrainMesh> {
        let base_path : &Path = &Path::new(base_path);
        let surface_file : &Path = &Path::new(surface_file);
        let annot_file : &Path = &Path::new(annot_file);
        let surface_file = base_path.join(&Path::new("surf")).join(surface_file);
        let annot_file = base_path.join(&Path::new("label")).join(annot_file);
        
        let surface = read_surf::<&Path>(&surface_file).unwrap();
        let annot = read_annot::<&Path>(&annot_file).unwrap();
        let cb_mesh = ColoredBrainMesh {
            mesh: surface.mesh.clone(),
            vertex_colors: annot.vertex_colors(true, 0),
        };
        Ok(cb_mesh)
    }


    /// Construct a ColoredBrainMesh from a label file in a FreeSurfer directory. This typically represents a single hemisphere.
    pub fn from_freesurfer_label(base_path : &str, surface_file : &str, label_file: &str) -> Result<ColoredBrainMesh> {
        let base_path : &Path = &Path::new(base_path);
        let surface_file : &Path = &Path::new(surface_file);
        let label_file : &Path = &Path::new(label_file);
        let surface_file = base_path.join(&Path::new("surf")).join(surface_file);
        let label_file = base_path.join(&Path::new("label")).join(label_file);
        
        let surface = read_surf::<&Path>(&surface_file).unwrap();
        let label = read_label::<&Path>(&label_file).unwrap();

        let red : [u8; 4] = [255, 0, 0, 255];
        let white : [u8; 4] = [255, 255, 255, 255];
        let label_display = FsLabelDisplay { label : label, num_surface_verts: surface.mesh.num_vertices(), color_bin_inside: red, color_bin_outside: white };

        let cb_mesh = ColoredBrainMesh {
            mesh: surface.mesh.clone(),
            vertex_colors: label_display.vertex_color_rgba(), // via VertexColor trait.
        };
        Ok(cb_mesh)
    }


    /// Get the vertex colors as u8 vector. For each vertex, 4 consecutive u8 values represent the red, green, blue, and alpha channel values, respectively.
    pub fn colors_rgba_u8(&self) -> Vec<u8> {
        self.vertex_colors.clone()
    }
    
}

