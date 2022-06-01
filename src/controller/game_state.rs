use std::sync::{atomic::AtomicU8};


pub struct GameState{
    pub game_state_id: AtomicU8,
    pub camera_pos:  (f64, f64),
    pub cursor_pos_relative: (f32, f32),
    pub window_dimensions: (u32, u32),
}


impl GameState {
    pub fn new(window_dimensions: (u32, u32)) -> Self{
        //game_state_id is supposed to hold information about what part of the game you're in
        GameState { game_state_id: AtomicU8::new(1), camera_pos: (0.0,0.0), cursor_pos_relative: (0.0, 0.0), window_dimensions: window_dimensions }
    }


    #[allow(dead_code)]
    pub fn mouse_pos_relative(&self) -> (f32, f32){
        return self.cursor_pos_relative;
    }
}