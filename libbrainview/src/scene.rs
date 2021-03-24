
use three_d::*;
use crate::{ColoredBrainMesh, mesh_from_colored_brain_mesh, brain_mesh_aabb};

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

fn compute_meshes_center(cb_meshes: &Vec<ColoredBrainMesh>) -> (f32, f32, f32) {
    let (min_x, max_x, min_y, max_y, min_z, max_z) = compute_meshes_minmax_coords(cb_meshes);
    let cx = (min_x + max_x) / 2.0;
    let cy = (min_y + max_y) / 2.0;
    let cz = (min_z + max_z) / 2.0;
    (cx, cy, cz)
}

fn compute_meshes_minmax_coords(cb_meshes: &Vec<ColoredBrainMesh>) -> (f32, f32, f32, f32, f32, f32) {
    if cb_meshes.is_empty() {
        panic!("Mesh list must no be empty.");
    } else {
        let mut min_x = f32::INFINITY;
        let mut min_y = f32::INFINITY;
        let mut min_z = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut max_y = f32::NEG_INFINITY;
        let mut max_z = f32::NEG_INFINITY;
        for cb_mesh in cb_meshes {
            let aabb = brain_mesh_aabb(&cb_mesh).unwrap();
            if aabb.min[0] < min_x { min_x =  aabb.min[0] };
            if aabb.min[1] < min_y { min_y =  aabb.min[1] };
            if aabb.min[2] < min_z { min_z =  aabb.min[2] };

            if aabb.max[0] > max_x { max_x =  aabb.max[0] };
            if aabb.max[1] > max_y { max_y =  aabb.max[1] };
            if aabb.max[2] > max_z { max_z =  aabb.max[2] };
        }
        (min_x, max_x, min_y, max_y, min_z, max_z)
    }    
}

fn compute_meshes_max_radius(cb_meshes: &Vec<ColoredBrainMesh>) -> f32 {
    let mm = compute_meshes_minmax_coords(cb_meshes);
    let dx = mm.1 - mm.0;
    let dy = mm.3 - mm.2;
    let dz = mm.5 - mm.4;
    let maxd_xy = if dx > dy { dx } else { dy };
    let max = if maxd_xy > dz { maxd_xy } else { dz }; 
    max / 2.0
}

pub fn scene(meshes : Vec<ColoredBrainMesh>, scenesettings : Option<SceneSettings>) { 
    let scenesettings = scenesettings.unwrap_or(SceneSettings::default());
    let window = Window::new(&scenesettings.window_title, Some(scenesettings.window_size)).unwrap();
    let bg_color_rgba = scenesettings.bg_color;
    let context = window.gl();

    let sc = compute_meshes_center(&meshes);
    let scene_center = vec3(sc.0, sc.1, sc.2);
    let scene_radius = compute_meshes_max_radius(&meshes) * 3.0; // 300.0; // TODO: compute mesh max entend (over all meshes and)
    println!("Scene center is {} {} {}, radius is {}.", sc.0, sc.1, sc.2, scene_radius);
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
