

//use neuroformats::{read_surf, read_curv};
use libbrainview::{ColoredBrainMesh, FsDisplayable, scene, SceneSettings};
use structopt::StructOpt;



// Try: cargo run -- --basedir resources/subjects_dir/subject1/ --right-vis-type label --right-vis rh.entorhinal_exvivo.label
#[derive(Debug, StructOpt)]
#[structopt(name = "brainviewer", about = "A simple viewer for surface-based neuroimaging data in FreeSurfer formats.")]
struct Opt {

    #[structopt(short = "b", long = "basedir", default_value = ".", env = "FS_SUBJECT")]
    basedir: String,

    /// Mesh file for left brain hemisphere. Ignored unless hemi is 'lh'.
    #[structopt(short = "q", long = "left-surf", default_value = "lh.white")]
    lh_surf: String,

    /// Mesh file for right brain hemisphere. Ignored unless hemi is 'rh'.
    #[structopt(short = "w", long = "right-surf", default_value = "rh.white")]
    rh_surf: String,

    #[structopt(short = "a", long = "left-vis-type", default_value = "curv")]
    lh_vis_type: String,

    #[structopt(short = "s", long = "right-vis-type", default_value = "curv")]
    rh_vis_type: String,

    #[structopt(short = "z", long = "left-vis", default_value = "lh.thickness")]
    lh_vis_file: String,

    #[structopt(short = "x", long = "right-vis", default_value = "rh.thickness")]
    rh_vis_file: String,
}


fn main() {
    let opts = Opt::from_args();
    println!("{:?}", opts);

    // * Read brain meshes for both hemis:
    //let lh_white = read_surf("resources/subjects_dir/subject1/surf/lh.white").unwrap();
    //let rh_white = read_surf("resources/subjects_dir/subject1/surf/rh.white").unwrap();
    //
    // * Read brain morphometry data:
    //let lh_curv = read_curv("resources/subjects_dir/subject1/surf/lh.thickness").unwrap();
    //let rh_curv = read_curv("resources/subjects_dir/subject1/surf/rh.thickness").unwrap();
    //
    // * Create ColoredBrainMeshes from the meshes and morphometry data:
    //let lh_cbmesh = ColoredBrainMesh::from_brainmesh_and_data(&lh_white.mesh, lh_curv.data).unwrap();
    //let rh_cbmesh = ColoredBrainMesh::from_brainmesh_and_data(&rh_white.mesh, rh_curv.data).unwrap();


    // * Automatically create a ColoredBrainMesh from the given files in the standard FreeSurfer output directory structure.
    //let lh_cbmesh = ColoredBrainMesh::from_freesurfer_curv("resources/subjects_dir/subject1", "lh.white", "lh.thickness").unwrap();
    //let rh_cbmesh = ColoredBrainMesh::from_freesurfer_curv("resources/subjects_dir/subject1", "rh.white", "rh.thickness").unwrap();

    // * Automatically create a ColoredBrainMesh from the given annot files in the standard FreeSurfer output directory structure.
    //let lh_cbmesh = ColoredBrainMesh::from_freesurfer_annot("resources/subjects_dir/subject1", "lh.white", "lh.aparc.annot").unwrap();
    //let rh_cbmesh = ColoredBrainMesh::from_freesurfer_annot("resources/subjects_dir/subject1", "rh.white", "rh.aparc.annot").unwrap();

    // * Automatically create a ColoredBrainMesh from the given label files in the standard FreeSurfer output directory structure.
    //let lh_cbmesh = ColoredBrainMesh::from_freesurfer_label("resources/subjects_dir/subject1", "lh.white", "lh.entorhinal_exvivo.label").unwrap();
    //let rh_cbmesh = ColoredBrainMesh::from_freesurfer_label("resources/subjects_dir/subject1", "rh.white", "rh.entorhinal_exvivo.label").unwrap();

    let mut meshes : Vec<ColoredBrainMesh> = Vec::new();

    if opts.lh_vis_type != "none" {
        let lh_fd_type = FsDisplayable::from_str(&opts.lh_vis_type).expect("Invalid value for parameter 'lh_vis_type'.");
        meshes.push(ColoredBrainMesh::from_freesurfer_type(&opts.basedir, &opts.lh_surf, &opts.lh_vis_file, lh_fd_type).unwrap());
    }
    if opts.rh_vis_type != "none" {
        let rh_fd_type = FsDisplayable::from_str(&opts.rh_vis_type).expect("Invalid value for parameter 'rh_vis_type'.");
        meshes.push(ColoredBrainMesh::from_freesurfer_type(&opts.basedir, &opts.rh_surf, &opts.rh_vis_file, rh_fd_type).unwrap());
    }

   

    // Visualize the ColoredBrainMeshes.
    let scenesettings = SceneSettings::default();  // Can be used to change resolution, background color, etc.
    scene(meshes, Some(scenesettings))
}
