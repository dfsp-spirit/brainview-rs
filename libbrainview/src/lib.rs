
pub mod error;
pub mod color;
pub mod util;
pub mod brainmesh;
pub mod threed_adapter;
pub mod scene;
pub mod fs_display;
pub mod vertexcolor;
pub mod fs_filepath;

pub use color::{color_from_data};
pub use util::{f32tou32, scale_to_01};
pub use brainmesh::{ColoredBrainMesh, FsDisplayable};
pub use threed_adapter::{mesh_from_colored_brain_mesh, brain_mesh_aabb};
pub use scene::{scene, SceneSettings};
pub use fs_display::{FsAnnotDisplay, FsCurvDisplay, FsLabelDisplay};
pub use vertexcolor::{VertexColor};




