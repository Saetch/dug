use std::sync::{atomic::AtomicU8};


pub struct GameState{
    pub game_state_id: AtomicU8,
    pub camera_pos:  (f64, f64)
}


impl GameState {
    pub fn new() -> Self{
        //game_state_id is supposed to hold information about what part of the game you're in
        GameState { game_state_id: AtomicU8::new(1), camera_pos: (0.0,0.0) }
    }
}