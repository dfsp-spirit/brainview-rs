
pub mod error;
pub mod color;
pub mod util;
pub mod brainmesh;

pub use color::{apply_colormap, colors_as_u8_4};
pub use util::{f32tou32, vec32minmax, scale_to_01};
pub use brainmesh::{ColoredBrainMesh, mesh_from_colored_brain_mesh};
