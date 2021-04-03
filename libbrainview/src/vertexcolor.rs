/// The vertexcolor trait and implementations.

use crate::fs_display::{FsLabelDisplay, FsAnnotDisplay, FsCurvDisplay};
use crate::color_from_data;

pub trait VertexColor {
    fn vertex_color_rgba(&self) -> Vec<u8>;
}

impl VertexColor for FsLabelDisplay {    
    fn vertex_color_rgba(&self) -> Vec<u8> {
        if self.label.is_binary() {
            binary_colors_for_data(self.label.is_surface_vertex_in_label(self.num_surface_verts), self.color_bin_inside, self.color_bin_outside)
        } else {
            color_from_data(self.label.as_surface_data(self.num_surface_verts, f32::NAN))
        }
    }
}

impl VertexColor for FsAnnotDisplay {    
    fn vertex_color_rgba(&self) -> Vec<u8> {
        self.annot.vertex_colors(true, self.unmatched_region_index)        
    }
}

impl VertexColor for FsCurvDisplay {    
    fn vertex_color_rgba(&self) -> Vec<u8> {
        color_from_data(self.curv.data.clone())
    }
}


fn binary_colors_for_data(data: Vec<bool>, inside_color : [u8; 4], outside_color : [u8; 4]) -> Vec<u8> {
    let mut col : Vec<u8> = Vec::with_capacity(data.len() * 4);
    for bval in data {
        if bval {
            col.push(inside_color[0]);
            col.push(inside_color[1]);
            col.push(inside_color[2]);
            col.push(inside_color[3]);
        } else {
            col.push(outside_color[0]);
            col.push(outside_color[1]);
            col.push(outside_color[2]);
            col.push(outside_color[3]);
        }        
    }
    col
}



