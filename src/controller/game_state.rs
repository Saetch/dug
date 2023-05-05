use std::sync::{atomic::AtomicU8};


pub struct GameState{
    pub game_state_id: AtomicU8,
    pub camera_pos:  (f64, f64),
    pub cursor_pos_relative: (f32, f32),
    pub cursor_pos_ingame: (f64, f64),
    pub window_dimensions: (u32, u32),
    pub window_dimensions_ingame: (f64, f64),
    pub window_dimensions_ratio: f64,
    pub camera_movement: (CamKeyPressed, CamKeyPressed),
    pub cam_speed: f32,
}

pub enum CamKeyPressed{
    Positive,
    Negative,
    None,
    Both
}


impl GameState {
    pub fn new(window_dimensions: (u32, u32)) -> Self{
        const WINDOW_DIMENSIONS_STARTUP: (f64, f64) = (1.6, 0.9);
        //game_state_id is supposed to hold information about what part of the game you're in
        GameState { 
            game_state_id: AtomicU8::new(1),
            camera_pos: (0.0,0.0), cursor_pos_relative: (0.0, 0.0), cursor_pos_ingame: (0.0, 0.0),
            window_dimensions: window_dimensions,
            window_dimensions_ingame: WINDOW_DIMENSIONS_STARTUP,   //display format 16:9 or similiar
            camera_movement: (CamKeyPressed::None, CamKeyPressed::None),
            cam_speed: 1.0f32,
            window_dimensions_ratio : WINDOW_DIMENSIONS_STARTUP.0 / WINDOW_DIMENSIONS_STARTUP.1,

         }
    }



    #[allow(dead_code)]
    pub fn mouse_pos_relative(&self) -> (f32, f32){
        return self.cursor_pos_relative;
    }
}