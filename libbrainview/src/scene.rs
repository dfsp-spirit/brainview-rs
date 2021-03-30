
use three_d::*;
use crate::{ColoredBrainMesh, mesh_from_colored_brain_mesh, brain_mesh_aabb};

/// Settings, like background color, that can be used to customize the appearance of a scene.
pub struct SceneSettings {
    pub bg_color : [f32; 4],
    pub window_size : (u32, u32),
    pub window_title : String,
    pub mouse_rotate_speed_factor: f32,
    pub cam_pan_speed: f32,
    pub cam_zoom_speed_keys: f32,
    pub auto_rotate_speed_factor: f64,
}


impl SceneSettings {

    /// The default scene settings.
    pub fn default() -> Self {
        SceneSettings {
            bg_color : [1.0, 1.0, 1.0, 1.0],
            window_size : (1280, 720),
            window_title : String::from("Scene"),
            mouse_rotate_speed_factor : 3.0,
            cam_pan_speed: 5.0,
            cam_zoom_speed_keys: 5.0,
            auto_rotate_speed_factor: 0.0005,
        }
    }
}


/// Compute the center of the whole scene, i.e., over all meshes. Used to determine 
/// where the camera should look.
fn compute_meshes_center(cb_meshes: &Vec<ColoredBrainMesh>) -> (f32, f32, f32) {
    let (min_x, max_x, min_y, max_y, min_z, max_z) = compute_meshes_minmax_coords(cb_meshes);
    let cx = (min_x + max_x) / 2.0;
    let cy = (min_y + max_y) / 2.0;
    let cz = (min_z + max_z) / 2.0;
    (cx, cy, cz)
}


/// Compute the min max coords (like an axis-aligned bounding box) for the whole scene, i.e.,
/// over all meshes. Used to determine how far the camera
/// should be from the scene center to see everything.
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


/// Compute the radius of the whole scene, i.e., over all meshes. Used to determine how far the camera
/// should be from the scene center to see everything.
fn compute_meshes_max_radius(cb_meshes: &Vec<ColoredBrainMesh>) -> f32 {
    let mm = compute_meshes_minmax_coords(cb_meshes);
    let dx = mm.1 - mm.0;
    let dy = mm.3 - mm.2;
    let dz = mm.5 - mm.4;
    let maxd_xy = if dx > dy { dx } else { dy };
    let max = if maxd_xy > dz { maxd_xy } else { dz }; 
    max / 2.0
}


/// Open a window and render a scene containing the given meshes.
///
/// The SceneSettings are optional, they can be used to customize the visualization. One can navigate in the scene,
/// zoom the camera, etc with the mouse or with key controls.
pub fn scene(meshes : Vec<ColoredBrainMesh>, scenesettings : Option<SceneSettings>) { 
    let scenesettings = scenesettings.unwrap_or(SceneSettings::default());

    // Prepare window
    let window = Window::new(&scenesettings.window_title, Some(scenesettings.window_size)).unwrap();    
    let context = window.gl();
   

    // Setup camera
    let sc = compute_meshes_center(&meshes);
    let scene_center = vec3(sc.0, sc.1, sc.2);
    let scene_radius = compute_meshes_max_radius(&meshes) * 3.0;
    let cam_move_speed : f32 = scene_radius / 40.;
    let mut camera = CameraControl::new(Camera::new_perspective(&context, scene_center + scene_radius * vec3(0.6, 0.3, 1.0).normalize(), scene_center, vec3(0.0, 1.0, 0.0),
                                             degrees(45.0), window.viewport().aspect(), 0.1, 1000.0).unwrap());
                                             
    // Get meshes into three-d datastructure and copy to GPU for rendering:
    let mut threed_meshes : Vec<Mesh> = Vec::with_capacity(meshes.len());
    for cbm in meshes.iter() {
        threed_meshes.push(mesh_from_colored_brain_mesh(&cbm, &context).unwrap());
    }
                                         

    // Render loop.
    let mut is_cam_mouse_rotating = false;     // Whether the user is currently rotating the cam with the mouse.
    let mut are_meshes_auto_rotating = true;   // Whether the brain mesh is auto-rotating. Can be toggled on/off.
    
    window.render_loop(move |frame_input|
    {
        camera.set_aspect(frame_input.viewport.aspect()).unwrap();

        for event in frame_input.events.iter() {
            match event {
                Event::MouseClick { state, button, .. } => {
                    is_cam_mouse_rotating = *button == MouseButton::Left && *state == State::Pressed;
                },
                Event::MouseMotion { delta, .. } => {
                    if is_cam_mouse_rotating {
                        camera.rotate_around_up((delta.0 as f32) * scenesettings.mouse_rotate_speed_factor, (delta.1 as f32) * scenesettings.mouse_rotate_speed_factor).unwrap();
                    }
                },
                Event::MouseWheel { delta, .. } => {
                    camera.zoom(delta.1 as f32).unwrap();
                },
                Event::Key { state, kind, .. } => {
                    if *kind == Key::P && *state == State::Pressed
                    {
                        are_meshes_auto_rotating = !are_meshes_auto_rotating;
                    }

                    // WASD cam controls, R+F is up/down. This movement direction is currently independent of 
                    // the view direction: one always moves along the x/y/z axes. This is rather unintuitive.
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

                    // Pan controls.
                    if *kind == Key::ArrowLeft && *state == State::Pressed
                    {
                        camera.pan(scenesettings.cam_pan_speed, 0.0).unwrap();
                    }
                    if *kind == Key::ArrowRight && *state == State::Pressed
                    {
                        camera.pan(-scenesettings.cam_pan_speed, 0.0).unwrap();
                    }
                    if *kind == Key::ArrowUp && *state == State::Pressed
                    {
                        camera.pan(0.0, scenesettings.cam_pan_speed).unwrap();
                    }
                    if *kind == Key::ArrowDown && *state == State::Pressed
                    {
                        camera.pan(0.0, -scenesettings.cam_pan_speed).unwrap();
                    }

                    // Zoom via keys instead of mouse
                    if *kind == Key::PageUp && *state == State::Pressed
                    {
                        camera.zoom(scenesettings.cam_zoom_speed_keys).unwrap();
                    }
                    if *kind == Key::PageDown && *state == State::Pressed
                    {
                        camera.zoom(-scenesettings.cam_zoom_speed_keys).unwrap();
                    }
                    
                },
                _ => {}
            }
        }

        // Do the actual rendering.
        Screen::write(&context, &ClearState::color_and_depth(scenesettings.bg_color[0], scenesettings.bg_color[1], scenesettings.bg_color[2], scenesettings.bg_color[3], 1.0), || {
            let transformation = if are_meshes_auto_rotating { Mat4::from_angle_y(radians((frame_input.accumulated_time * scenesettings.auto_rotate_speed_factor) as f32)) } else { Mat4::identity()};
            for mesh in threed_meshes.iter() {
                mesh.render_color(RenderStates::default(), frame_input.viewport, &transformation, &camera)?;
            }
            Ok(())
        }).unwrap();
        
        FrameOutput::default()
    }).unwrap();
}

