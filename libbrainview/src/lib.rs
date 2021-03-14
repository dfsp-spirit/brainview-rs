
pub mod error;
pub mod color;
pub mod util;
pub mod brainmesh;
pub mod threed_adapter;
pub mod scene;

pub use color::{color_from_data};
pub use util::{f32tou32, vec32minmax, scale_to_01};
pub use brainmesh::{ColoredBrainMesh};
pub use threed_adapter::{mesh_from_colored_brain_mesh};
pub use scene::scene;

