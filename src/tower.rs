use structopt::StructOpt;
use amethyst::{
    ecs::prelude::*,
    prelude::*,
    core::math::{Point2, Point3},
    renderer::types::TextureData,
    renderer::{debug_drawing::{DebugLines, DebugLinesComponent, DebugLinesParams}},
    window::ScreenDimensions,
};
use geo::{Rect};


use crate::camera;

use crate::image::TwImage;
use crate::args_cli::Opt;
use crate::inputshandler::{get_drop_file, get_moved_mouse, TwInputsHandler, alt_mouse_pressed,
                           mouse_released, alt_mouse_released, key_pressed, key_released,
                           ctrl_mouse_pressed, ctrl_mouse_released, mouse_pressed};

use crate::utils::{list_valid_files};




use std::sync::{Arc, Mutex};
use std::ffi::{OsStr, OsString};
use std::collections::HashMap;
use std::path::Path;

use std::time::Duration;


pub const BACKGROUNDCOLOR: [f32; 4] = [0.0, 0.0, 0.0, 1.0];
pub const BACKGROUNDCOLOR2: [f32; 4] = [0.1, 0.1, 0.1, 1.0];
pub const WINDOWWIDTH: f32 = 1024.0;
pub const WINDOWHEIGHT: f32 = 720.0;


pub struct TowerData {
    pub twimage_count: f32,
    pub scene_rect: Rect<f32>,
    pub active_rect: Rect<f32>,
    pub scene_middle_point: Point2<f32>,
    pub cache: Arc<Mutex<HashMap<String, (TwImage, TextureData)>>>,
    pub working_dir: OsString,
    pub file_to_cache: Vec<OsString>,
    pub files_order: Vec<OsString>,
    pub inputs_path: Vec<String>,
    pub debug_line_start: Point3<f32>,
    pub debug_line_end: Point3<f32>,
}

impl Default for TowerData {
    fn default() -> Self {
        Self {
            twimage_count: 0.0,
            scene_rect: Rect::new((0.0, 0.0), (0.0, 0.0)),
            active_rect: Rect::new((0.0, 0.0), (0.0, 0.0)),
            cache: Arc::new(Mutex::new(HashMap::new())),
            working_dir: OsStr::new(".").to_owned(),
            file_to_cache: Vec::new(),
            files_order: Vec::new(),
            scene_middle_point: Point2::new(0.0, 0.0),
            inputs_path: Vec::new(),
            debug_line_start: Point3::new(0.0, 0.0, 0.0),
            debug_line_end: Point3::new(0.0, 0.0, 0.0),
        }
    }
}

#[derive(Default)]
pub struct Tower;

impl<'a> SimpleState for Tower {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        // init camera
        camera::initialise_camera(world);
        // command line arguments
        let opt = Opt::from_args();
        // init tower data
        let mut tower_data = TowerData::default();
        // get file to cache
        tower_data.inputs_path = opt.inputs.iter().map(|input| input.to_owned()).collect::<Vec<_>>();
        if let Some(last_input_path) = tower_data.inputs_path.last() {
            tower_data.working_dir = Path::new(last_input_path).parent().unwrap().as_os_str().to_owned();
        }
        tower_data.file_to_cache = list_valid_files(&tower_data.working_dir);
        tower_data.files_order = tower_data.file_to_cache.clone();
        world.insert(tower_data);
        // init twinputshandler
        let mut tw_inputs_handler = TwInputsHandler::default();
        tw_inputs_handler.stopwatch.start();
        tw_inputs_handler.double_click_stopwatch.start();
        tw_inputs_handler.window_zoom_factor = 1.0;
        world.insert(tw_inputs_handler);

        // DEBUG
        // Setup debug lines as a resource
        world.insert(DebugLines::new());
        // Configure width of lines. Optional step
        world.insert(DebugLinesParams { line_width: 2.0 });
        world.register::<DebugLinesComponent>();
    }

    fn handle_event(&mut self, data: StateData<'_, GameData<'_, '_>>, event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = event {
            // drop file event
            if let Some(drop_file) = get_drop_file(&event) {
                {
                    info!("{:?}", drop_file);
                    let mut tw_in = data.world.fetch_mut::<TwInputsHandler>();
                    tw_in.last_dropped_file_path.push(drop_file);
                }
            }
            // mouse move event
            if let Some(mouse_pos) = get_moved_mouse(&event) {
                {
                    let mut tw_in = data.world.fetch_mut::<TwInputsHandler>();
                    let screen = data.world.fetch::<ScreenDimensions>();
                    tw_in.mouse_position = Some(((mouse_pos.x as f32) * screen.hidpi_factor() as f32, (mouse_pos.y as f32) * screen.hidpi_factor() as f32));
                    let p = tw_in.mouse_position.unwrap().clone();
                    if tw_in.mouse_position_history.len() == 2 {
                        tw_in.mouse_position_history.remove(0);
                        tw_in.mouse_position_history.insert(1, p);

                    } else {
                        tw_in.mouse_position_history.push(p);
                    }
                }
            }
            // alt mouse pressed event
            if let Some(button) = alt_mouse_pressed(&event) {
                {
                    let mut tw_in = data.world.fetch_mut::<TwInputsHandler>();
                    tw_in.alt_mouse_button_pressed = Some(button);
                }
            }
            // alt mouse release
            if let Some(_button) = alt_mouse_released(&event) {
                {
                    let mut tw_in = data.world.fetch_mut::<TwInputsHandler>();
                    tw_in.alt_mouse_button_pressed = None;
                }
            }
            // ctrl mouse pressed event
            if let Some(button) = ctrl_mouse_pressed(&event) {
                {
                    let mut tw_in = data.world.fetch_mut::<TwInputsHandler>();
                    tw_in.ctrl_mouse_button_pressed = Some(button);
                    tw_in.mouse_world_clicked_position = tw_in.mouse_world_position;
                }
            }
            // ctrl mouse release
            if let Some(_button) = ctrl_mouse_released(&event) {
                {
                    let mut tw_in = data.world.fetch_mut::<TwInputsHandler>();
                    tw_in.ctrl_mouse_button_pressed = None;
                }
            }
            // mouse pressed event
            if let Some(button) = mouse_pressed(&event) {
                {
                    let mut tw_in = data.world.fetch_mut::<TwInputsHandler>();
                    tw_in.mouse_button_pressed = Some(button);
                    tw_in.mouse_world_clicked_position = tw_in.mouse_world_position;
                    if Duration::from_millis(300) >= tw_in.double_click_stopwatch.elapsed() {
                        tw_in.mouse_double_clicked = Some(button);
                    }
                    tw_in.double_click_stopwatch.restart();
                }
            }
            // mouse released event
            if let Some(_button) = mouse_released(&event) {
                {
                    let mut tw_in = data.world.fetch_mut::<TwInputsHandler>();
                    tw_in.mouse_button_pressed = None;
                    tw_in.alt_mouse_button_pressed = None;
                    tw_in.ctrl_mouse_button_pressed = None;
                    tw_in.mouse_world_clicked_position = None;
                    tw_in.mouse_double_clicked = None;
                }
            }
            // keyboard pressed
            if let Some(key_code) = key_pressed(&event) {
                {
                    let mut tw_in = data.world.fetch_mut::<TwInputsHandler>();
                    tw_in.keys_pressed.push(key_code);
                }
            }
            // keyboard released
            if let Some(key_code) = key_released(&event) {
                {
                    let mut tw_in = data.world.fetch_mut::<TwInputsHandler>();
                    tw_in.keys_pressed.retain(|x| *x != key_code);
                }
            }
        }
        Trans::None
    }
}

