/// Wrappers around neuroformats structs representing FreeSurfer data.
/// These wrappers contain aditional information required for visualizing the data.

use neuroformats::{FsLabel, FsAnnot, FsCurv};

#[derive(Debug, Clone, PartialEq)]
pub struct FsLabelDisplay {
    pub label: FsLabel,
    pub num_surface_verts: usize,
    pub color_bin_inside: [u8; 4],
    pub color_bin_outside: [u8; 4],
    // pub colormap: x,
}


#[derive(Debug, Clone, PartialEq)]
pub struct FsAnnotDisplay {
    pub annot: FsAnnot,
    pub unmatched_region_index: usize,
}

pub struct FsCurvDisplay {
    pub curv: FsCurv,
    // pub colormap: x,
}

