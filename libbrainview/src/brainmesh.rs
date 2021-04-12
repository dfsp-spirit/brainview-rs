//! Datastructures modeling brain meshes.

use std::path::{Path};

use neuroformats::{BrainMesh, read_curv, read_surf, read_annot, read_label};
use crate::{FsLabelDisplay, color_from_data, error::{Result, BrainviewError}};
use crate::vertexcolor::VertexColor;

/// Models a vertex-colored BrainMesh for a single hemisphere.
#[derive(Debug, Clone, PartialEq)]
pub struct ColoredBrainMesh {
    pub mesh : BrainMesh,
    pub vertex_colors: Vec<u8>,
}

/// Models two vertex-colored BrainMeshes, one per hemisphere.
#[derive(Debug, Clone, PartialEq)]
pub struct ColoredBrain {
    pub lh : ColoredBrainMesh,
    pub rh : ColoredBrainMesh,
}

#[derive(Debug, Clone, PartialEq)]
pub enum FsDisplayable {
    Curv,
    Annot,
    Label,
}

impl FsDisplayable {
    pub fn from_str(fd_str : &str) -> Result<FsDisplayable> {
        let fsd = match fd_str {
            "annot"=> Ok(FsDisplayable::Annot),
            "curv" => Ok(FsDisplayable::Curv),            
            "label"=> Ok(FsDisplayable::Label),
            _ => Err(BrainviewError::StringNotFsDisplayable)
        };
        fsd
    }
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
    pub fn from_freesurfer_curv(surface_file_path : &str, curv_file_path: &str) -> Result<ColoredBrainMesh> {
        let surface_file = &Path::new(surface_file_path);
        let curv_file = &Path::new(curv_file_path);
        let surface = read_surf::<&Path>(&surface_file).expect("Cannot read surface file");
        let curv = read_curv::<&Path>(&curv_file).expect("Cannot read curv file");
        let cb_mesh = ColoredBrainMesh {
            mesh: surface.mesh.clone(),
            vertex_colors: color_from_data(curv.data)
        };
        Ok(cb_mesh)
    }

    /// Construct a ColoredBrainMesh from morphometry data files in a FreeSurfer directory. This typically represents a single hemisphere.
    pub fn from_freesurfer_curv_base(base_path : &str, surface_file : &str, curv_file: &str) -> Result<ColoredBrainMesh> {
        let base_path : &Path = &Path::new(base_path);
        let surface_file : &Path = &Path::new(surface_file);
        let curv_file : &Path = &Path::new(curv_file);
        let surface_file = base_path.join(&Path::new("surf")).join(surface_file);
        let curv_file = base_path.join(&Path::new("surf")).join(curv_file);
        ColoredBrainMesh::from_freesurfer_curv(surface_file.to_str().unwrap(), curv_file.to_str().unwrap())        
    }


    /// Construct a ColoredBrainMesh from brain atlas surface parcellation files in a FreeSurfer directory. This typically represents a single hemisphere.
    pub fn from_freesurfer_annot_base(base_path : &str, surface_file : &str, annot_file: &str) -> Result<ColoredBrainMesh> {
        let base_path : &Path = &Path::new(base_path);
        let surface_file : &Path = &Path::new(surface_file);
        let annot_file : &Path = &Path::new(annot_file);
        let surface_file = base_path.join(&Path::new("surf")).join(surface_file);
        let annot_file = base_path.join(&Path::new("label")).join(annot_file);
        ColoredBrainMesh::from_freesurfer_annot(surface_file.to_str().unwrap(), annot_file.to_str().unwrap())        
    }

    /// Construct a ColoredBrainMesh from brain atlas surface parcellation files in a FreeSurfer directory. This typically represents a single hemisphere.
    pub fn from_freesurfer_annot(surface_file_path : &str, annot_file_path: &str) -> Result<ColoredBrainMesh> {
        let surface_file = &Path::new(surface_file_path);
        let annot_file = &Path::new(annot_file_path);
        let surface = read_surf::<&Path>(&surface_file).expect("Cannot read surface file");
        let annot = read_annot::<&Path>(&annot_file).expect("Cannot read annot file");
        let cb_mesh = ColoredBrainMesh {
            mesh: surface.mesh.clone(),
            vertex_colors: annot.vertex_colors(true, 0),
        };
        Ok(cb_mesh)
    }


    /// Construct a ColoredBrainMesh from a label file in a FreeSurfer directory. This typically represents a single hemisphere.
    pub fn from_freesurfer_label_base(base_path : &str, surface_file : &str, label_file: &str) -> Result<ColoredBrainMesh> {
        let base_path : &Path = &Path::new(base_path);
        let surface_file : &Path = &Path::new(surface_file);
        let label_file : &Path = &Path::new(label_file);
        let surface_file = base_path.join(&Path::new("surf")).join(surface_file);
        let label_file = base_path.join(&Path::new("label")).join(label_file);
        ColoredBrainMesh::from_freesurfer_label(surface_file.to_str().unwrap(), label_file.to_str().unwrap())
    }

    /// Construct a ColoredBrainMesh from a label file in a FreeSurfer directory. This typically represents a single hemisphere.
    pub fn from_freesurfer_label(surface_file_path : &str, label_file_path: &str) -> Result<ColoredBrainMesh> {
        let surface_file = &Path::new(surface_file_path);
        let label_file = &Path::new(label_file_path);
        
        let surface = read_surf::<&Path>(&surface_file).expect("Cannot read surface file.");
        let label = read_label::<&Path>(&label_file).expect("Cannot read label file");

        let red : [u8; 4] = [255, 0, 0, 255];
        let white : [u8; 4] = [255, 255, 255, 255];
        let label_display = FsLabelDisplay { label : label, num_surface_verts: surface.mesh.num_vertices(), color_bin_inside: red, color_bin_outside: white };

        let cb_mesh = ColoredBrainMesh {
            mesh: surface.mesh.clone(),
            vertex_colors: label_display.vertex_color_rgba(), // via VertexColor trait.
        };
        Ok(cb_mesh)
    }

    /// Construct a ColoredBrainMesh from a label, annot or vurv file in a FreeSurfer directory. This typically represents a single hemisphere.
    pub fn from_freesurfer_type_base(base_path : &str, surface_file : &str, vis_file: &str, vis_type: &FsDisplayable) -> Result<ColoredBrainMesh> {
        if *vis_type == FsDisplayable::Annot {
            ColoredBrainMesh::from_freesurfer_annot_base(base_path, surface_file, vis_file)
        } else if *vis_type == FsDisplayable::Curv {
            ColoredBrainMesh::from_freesurfer_curv_base(base_path, surface_file, vis_file)
        } else if *vis_type == FsDisplayable::Label {
            ColoredBrainMesh::from_freesurfer_label_base(base_path, surface_file, vis_file)
        } else {
            panic!("Unsupported FsDisplayable");
        }
    }

    /// Construct a ColoredBrainMesh from a label, annot or vurv file in a FreeSurfer directory. This typically represents a single hemisphere.
    pub fn from_freesurfer_type(surface_file : &str, vis_file: &str, vis_type: &FsDisplayable) -> Result<ColoredBrainMesh> {
        if *vis_type == FsDisplayable::Annot {
            ColoredBrainMesh::from_freesurfer_annot(surface_file, vis_file)
        } else if *vis_type == FsDisplayable::Curv {
            ColoredBrainMesh::from_freesurfer_curv(surface_file, vis_file)
        } else if *vis_type == FsDisplayable::Label {
            ColoredBrainMesh::from_freesurfer_label(surface_file, vis_file)
        } else {
            panic!("Unsupported FsDisplayable");
        }
    }


    /// Get the vertex colors as u8 vector. For each vertex, 4 consecutive u8 values represent the red, green, blue, and alpha channel values, respectively.
    pub fn colors_rgba_u8(&self) -> Vec<u8> {
        self.vertex_colors.clone()
    }
    
}


/// A simple wrapper struct around two ColoredBrainMesh instances, for both hemispheres of a brain.
impl ColoredBrain {

    /// Construct a ColoredBrain from a base path and surface and display type strings.
    pub fn from_freesurfer_type_base(base_path : &str, surface_file_no_hemi : &str, vis_file_no_hemi: &str, vis_type: &FsDisplayable) -> Result<ColoredBrain> {
        let lh_surface_file = &format!("lh.{}", surface_file_no_hemi);
        let rh_surface_file = &format!("rh.{}", surface_file_no_hemi);
        let lh_vis_file = &format!("lh.{}", vis_file_no_hemi);
        let rh_vis_file = &format!("rh.{}", vis_file_no_hemi);
        let lh = ColoredBrainMesh::from_freesurfer_type_base(base_path, lh_surface_file, lh_vis_file, vis_type).unwrap();
        let rh = ColoredBrainMesh::from_freesurfer_type_base(base_path, rh_surface_file, rh_vis_file, vis_type).unwrap();
        let cb = ColoredBrain { lh : lh, rh : rh };
        Ok(cb)
    }

    /// Construct a ColoredBrain from absolute file paths for both hemis.
    pub fn from_freesurfer_type(lh_surface_file : &str, rh_surface_file : &str, lh_vis_file: &str, rh_vis_file: &str, vis_type: &FsDisplayable) -> Result<ColoredBrain> {
        let lh = ColoredBrainMesh::from_freesurfer_type(lh_surface_file, lh_vis_file, vis_type).unwrap();
        let rh = ColoredBrainMesh::from_freesurfer_type(rh_surface_file, rh_vis_file, vis_type).unwrap();
        let cb = ColoredBrain { lh : lh, rh : rh };
        Ok(cb)
    }

    /// Returns a vector of owned (cloned) ColoredBrainMeshes from this ColoredBrain. The order is lh, rh.
    pub fn to_vec(&self) -> Vec<ColoredBrainMesh> {
        let v = vec![self.lh.clone(), self.rh.clone()];
        v
    }
}
