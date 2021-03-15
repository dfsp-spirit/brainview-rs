

use neuroformats::{read_surf, read_curv};
use libbrainview::{ColoredBrainMesh, scene, SceneSettings};

fn main() {
    //let args: Vec<String> = std::env::args().collect();

    // Read brain meshes for both hemis:
    let lh_white = read_surf("resources/subjects_dir/subject1/surf/lh.white").unwrap();
    let rh_white = read_surf("resources/subjects_dir/subject1/surf/rh.white").unwrap();

    // Read brain morphometry data:
    let lh_curv = read_curv("resources/subjects_dir/subject1/surf/lh.thickness").unwrap();
    let rh_curv = read_curv("resources/subjects_dir/subject1/surf/rh.thickness").unwrap();

    // Create ColoredBrainMeshes from the meshes and morphometry data:
    let lh_cbmesh = ColoredBrainMesh::from_brainmesh_and_data(&lh_white.mesh, lh_curv.data).unwrap();
    let rh_cbmesh = ColoredBrainMesh::from_brainmesh_and_data(&rh_white.mesh, rh_curv.data).unwrap();

    // Visualize the ColoredBrainMeshes.
    scene(vec![lh_cbmesh, rh_cbmesh], None)
}
