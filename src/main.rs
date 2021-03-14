

use three_d::*;
use neuroformats::{read_surf, read_curv};
use libbrainview::{color_from_data, mesh_from_colored_brain_mesh};
use libbrainview::ColoredBrainMesh;

fn main() {
    //let args: Vec<String> = std::env::args().collect();

    let window = Window::new("Cortical thickness", Some((1280, 720))).unwrap();
    let context = window.gl();

    let scene_center = vec3(0.0, 0.0, 0.0); // TODO: compute from meshes or translate meshes to center = 0,0,0.
    let scene_radius = 300.0; // TODO: compute mesh max entend
    let mut camera = CameraControl::new(Camera::new_perspective(&context, scene_center + scene_radius * vec3(0.6, 0.3, 1.0).normalize(), scene_center, vec3(0.0, 1.0, 0.0),
                                             degrees(45.0), window.viewport().aspect(), 0.1, 1000.0).unwrap());

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

    let lh_mesh = mesh_from_colored_brain_mesh(&lh_cbmesh, &context).unwrap();
    let rh_mesh = mesh_from_colored_brain_mesh(&rh_cbmesh, &context).unwrap();

    // main loop
    let mut cam_rotating = false;   // Whether the user is rotating the cam with the mouse.
    let mut do_transform = true;   // Whether the brain mesh is auto-rotating. Can be toggled on/off.
    let mouse_rotate_speed_factor : f32 = 3.0;
    window.render_loop(move |frame_input|
    {
        camera.set_aspect(frame_input.viewport.aspect()).unwrap();

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick { state, button, .. } => {
                    cam_rotating = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion { delta, .. } => {
                    if cam_rotating {
                        camera.rotate_around_up((delta.0 as f32) * mouse_rotate_speed_factor, (delta.1 as f32) * mouse_rotate_speed_factor).unwrap();
                    }
                },
                Event::MouseWheel { delta, .. } => {
                    camera.zoom(delta.1 as f32).unwrap();
                },
                Event::Key { state, kind, .. } => {
                    if *kind == Key::R && *state == State::Pressed
                    {
                        do_transform = !do_transform;
                    }
                },
                _ => {}
            }
        }

        Screen::write(&context, &ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0), || {
            let transformation = if do_transform { Mat4::from_angle_y(radians((frame_input.accumulated_time * 0.0005) as f32)) } else { Mat4::identity()};
            lh_mesh.render_color(RenderStates::default(), frame_input.viewport, &transformation, &camera)?;
            rh_mesh.render_color(RenderStates::default(), frame_input.viewport, &transformation, &camera)?;
            Ok(())
        }).unwrap();
        
        FrameOutput::default()
    }).unwrap();
}
