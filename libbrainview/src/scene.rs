
use three_d::*;
use crate::{ColoredBrainMesh, mesh_from_colored_brain_mesh};

/// Settings, like background color, that can be used to customize the appearance of a scene.
pub struct SceneSettings {
    pub bg_color : [f32; 4],
    pub window_size : (u32, u32),
    pub window_title : String,
    pub mouse_rotate_speed_factor: f32,
}

impl SceneSettings {
    pub fn default() -> Self {
        SceneSettings {
            bg_color : [1.0, 1.0, 1.0, 1.0],
            window_size : (1280, 720),
            window_title : String::from("Scene"),
            mouse_rotate_speed_factor : 3.0,
        }
    }
}

pub fn scene(meshes : Vec<ColoredBrainMesh>, scenesettings : Option<SceneSettings>) { 
    let scenesettings = scenesettings.unwrap_or(SceneSettings::default());
    let window = Window::new(&scenesettings.window_title, Some(scenesettings.window_size)).unwrap();
    let bg_color_rgba = scenesettings.bg_color;
    let context = window.gl();

    let scene_center = vec3(0.0, 0.0, 0.0); // TODO: compute from meshes or translate meshes to center = 0,0,0.
    let scene_radius = 300.0; // TODO: compute mesh max entend (over all meshes)
    let cam_move_speed : f32 = scene_radius / 40.;
    let mut camera = CameraControl::new(Camera::new_perspective(&context, scene_center + scene_radius * vec3(0.6, 0.3, 1.0).normalize(), scene_center, vec3(0.0, 1.0, 0.0),
                                             degrees(45.0), window.viewport().aspect(), 0.1, 1000.0).unwrap());
                                             
    let mut threed_meshes : Vec<Mesh> = Vec::with_capacity(meshes.len());
    for cbm in meshes.iter() {
        threed_meshes.push(mesh_from_colored_brain_mesh(&cbm, &context).unwrap());
    }
                                         

    // main loop
    let mut cam_rotating = false;   // Whether the user is currently rotating the cam with the mouse.
    let mut do_transform = true;   // Whether the brain mesh is auto-rotating. Can be toggled on/off.
    let mouse_rotate_speed_factor : f32 = scenesettings.mouse_rotate_speed_factor;
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
                    if *kind == Key::P && *state == State::Pressed
                    {
                        do_transform = !do_transform;
                    }

                    // WASD cam controls, R+F is up/down
                    if *kind == Key::W && *state == State::Pressed
                    {
                        camera.translate(&vec3(cam_move_speed, 0.0, 0.0)).unwrap();
                    }
                    if *kind == Key::S && *state == State::Pressed
                    {
                        camera.translate(&vec3(-cam_move_speed, 0.0, 0.0)).unwrap();
                    }
                    if *kind == Key::A && *state == State::Pressed
                    {
                        camera.translate(&vec3(0.0, cam_move_speed, 0.0)).unwrap();
                    }
                    if *kind == Key::D && *state == State::Pressed
                    {
                        camera.translate(&vec3(0.0, -cam_move_speed, 0.0)).unwrap();
                    }
                    if *kind == Key::R && *state == State::Pressed
                    {
                        camera.translate(&vec3(0.0, 0.0, cam_move_speed)).unwrap();
                    }
                    if *kind == Key::F && *state == State::Pressed
                    {
                        camera.translate(&vec3(0.0, 0.0,  -cam_move_speed)).unwrap();
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
