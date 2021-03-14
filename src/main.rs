

use neuroformats::{read_surf, read_curv};
use libbrainview::{ColoredBrainMesh, color_from_data, scene};

fn main() {
    //let args: Vec<String> = std::env::args().collect();


    // Read brain meshes for both hemis.
    let lh_white = read_surf("resources/subjects_dir/subject1/surf/lh.white").unwrap();
    let rh_white = read_surf("resources/subjects_dir/subject1/surf/rh.white").unwrap();

    // Read brain morphometry data
    let lh_curv = read_curv("resources/subjects_dir/subject1/surf/lh.thickness").unwrap();
    let rh_curv = read_curv("resources/subjects_dir/subject1/surf/rh.thickness").unwrap();

    // generate colors from morph data
    let lh_brain_colors = color_from_data(lh_curv.data);
    let rh_brain_colors = color_from_data(rh_curv.data);

    let lh_cbmesh = ColoredBrainMesh::from_brainmesh_and_colors(&lh_white.mesh, lh_brain_colors).unwrap();
    let rh_cbmesh = ColoredBrainMesh::from_brainmesh_and_colors(&rh_white.mesh, rh_brain_colors).unwrap();

    scene(vec![lh_cbmesh, rh_cbmesh])
}
