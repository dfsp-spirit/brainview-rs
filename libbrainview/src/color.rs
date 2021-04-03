

use crate::{scale_to_01};

/// Apply a colormap to the given data, i.e., map the values to colors.
pub fn apply_colormap(data: Vec<f32>, cmap: colorous::Gradient) -> Vec<colorous::Color> {

    let nan_color = colorous::Color{ r: 245 as u8, g: 245 as u8, b: 245 as u8};
    let mut colors : Vec<colorous::Color> = Vec::with_capacity(data.len());
    for v in data.iter() {
        if v.is_nan() {
            colors.push(nan_color);
        } else {
            colors.push(cmap.eval_continuous((*v).into()));
        }
    }
    colors
}


/// Translate `n` colorous::Color values to `4n` u8 values.
pub fn colors_as_u8_4(colors : Vec<colorous::Color>, alpha: u8) -> Vec<u8> {
    let mut col_255 : Vec<u8> = Vec::with_capacity(colors.len() * 4);
    for v in colors.iter() {
        let rgb : [u8; 3] = (*v).into_array();
        for c in rgb.iter() {
            col_255.push(*c);
        }
        col_255.push(alpha as u8);
    }
    col_255
}


/// Apply a colormap and return colors as required for three-d.
pub fn color_from_data(data : Vec<f32>) -> Vec<u8> {
    let mesh_alpha : u8 = 255;
    let gradient = colorous::VIRIDIS;
    colors_as_u8_4(apply_colormap(scale_to_01(data), gradient), mesh_alpha)
}

