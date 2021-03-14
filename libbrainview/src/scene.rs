
use three_d::*;
use crate::{ColoredBrainMesh, mesh_from_colored_brain_mesh};

pub fn scene(meshes : Vec<ColoredBrainMesh>) { 
    let window = Window::new("Cortical thickness", Some((1280, 720))).unwrap();
    let bg_color_rgba : [f32; 4] = [1.0, 1.0, 1.0, 1.0]; // Background color.
    let context = window.gl();

    let scene_center = vec3(0.0, 0.0, 0.0); // TODO: compute from meshes or translate meshes to center = 0,0,0.
    let scene_radius = 300.0; // TODO: compute mesh max entend
    let mut camera = CameraControl::new(Camera::new_perspective(&context, scene_center + scene_radius * vec3(0.6, 0.3, 1.0).normalize(), scene_center, vec3(0.0, 1.0, 0.0),
                                             degrees(45.0), window.viewport().aspect(), 0.1, 1000.0).unwrap());
                                             
    let mut threed_meshes : Vec<Mesh> = Vec::with_capacity(meshes.len());
    for cbm in meshes.iter() {
        threed_meshes.push(mesh_from_colored_brain_mesh(&cbm, &context).unwrap());
    }
                                         

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

        Screen::write(&context, &ClearState::color_and_depth(bg_color_rgba[0], bg_color_rgba[1], bg_color_rgba[2], bg_color_rgba[3], 1.0), || {
            let transformation = if do_transform { Mat4::from_angle_y(radians((frame_input.accumulated_time * 0.0005) as f32)) } else { Mat4::identity()};
            for mesh in threed_meshes.iter() {
                mesh.render_color(RenderStates::default(), frame_input.viewport, &transformation, &camera)?;
            }
            Ok(())
        }).unwrap();
        
        FrameOutput::default()
    }).unwrap();
}
