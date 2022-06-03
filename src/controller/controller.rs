
use std::{sync::{Arc, atomic::{AtomicBool, self}, RwLock}};

use flume::Receiver;
use winit::event::{VirtualKeyCode, ElementState, MouseScrollDelta};

use crate::{controller::{controller_input::MouseInputType, button_mapping::{load_default_keybinds, key_action_pressed, key_action_released}}, view::renderer::Vertex, model::{game_object::{GameObject, debug_object::DebugObject}, model::Model}, drawable_object::static_object::StaticObject};

use super::{controller_input::ControllerInput, game_state::GameState, button_constants::{W_BUTTON, D_BUTTON, S_BUTTON, A_BUTTON, MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, SPACE_BAR}};

use spin_sleep::LoopHelper;


pub(crate) type KeyboundFunction = fn(&Arc<RwLock<GameState>>, &Arc<Model>);

pub fn handle_input_loop(thread_running: Arc<AtomicBool>, receiver: Receiver<ControllerInput>, game_state: Arc<RwLock<GameState>>, model_pointer:  Arc<Model>){


    let mut keybinds = load_default_keybinds();


    while thread_running.load(atomic::Ordering::Relaxed){
        let inp = receiver.recv();
        if let Ok(input) = inp{

            //Here, the actual logic gets processed, everything around this is just to keep the loop alive and shut it down when needed
            match input{
                ControllerInput::MouseInput { action } => process_mouse_input(action, &game_state, &keybinds, &model_pointer),
                ControllerInput::KeyboardInput { key, state } => process_keyboard_input(key, state, &game_state, &keybinds, &model_pointer),
                ControllerInput::WindowResized { dimensions } => game_state.write().unwrap().window_dimensions = dimensions,
            }

        }else {
            if thread_running.load(atomic::Ordering::SeqCst){
                println!("Could not receive input message from rendering thread!");

            }else{
                println!("Gracefully stopping the controller thread" );

            }
            break;
        }
        
    }

    println!("Oh no! I'm getting terminated! Brhsshh!");

}

#[allow(dead_code)]
pub(crate) fn no_action(_game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){

}

//TODO, execute these actions on correct key press
pub(crate) fn up_action(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    todo!();
}

pub(crate) fn camera_down_action(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    todo!();
}

pub(crate) fn camera_up_action(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    todo!();
}

pub(crate) fn camera_right_action(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    todo!();
}

pub(crate) fn camera_left_action(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    todo!();
}


pub(crate) fn half_screen_width_ingame_regular(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut game_state_lock = game_state.write().unwrap();
    game_state_lock.window_dimensions_ingame = (1.0, 1.0);
}

pub(crate) fn half_screen_width_ingame_2times(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut game_state_lock = game_state.write().unwrap();
    game_state_lock.window_dimensions_ingame = (2.0, 2.0);
}

pub(crate) fn half_screen_width_ingame_point5times(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut game_state_lock = game_state.write().unwrap();
    game_state_lock.window_dimensions_ingame = (0.5, 0.5);
}


fn process_mouse_input(action: MouseInputType, game_state: &Arc<RwLock<GameState>>, keybinds: &Vec<Option<KeyboundFunction>>, model_pointer:  &Arc<Model>){
    match action{
        MouseInputType::Move(x, y) => mouse_moved_action(x,y, game_state),
        MouseInputType::Click { button, state } =>
        if state == ElementState::Pressed{
            if let Some(func) = keybinds[MOUSE_LEFT] {
                func(&game_state, &model_pointer)
            }
        },

        MouseInputType::Scroll { delta, phase : _phase } => process_mouse_scroll(delta, game_state),
        MouseInputType::EnteredWindow => (),
        MouseInputType::LeftWindow =>(),
    }
}

pub(crate) fn place_debug_object_action(game_state: &Arc<RwLock<GameState>>, model: &Arc<Model>){

    let lock = game_state.read().unwrap();
    let mouse_coords = lock.cursor_pos_ingame;
    let mut lock = model.game_objects.write().unwrap();
    let new_object = DebugObject::new(mouse_coords);
    lock.push(Box::new(new_object));


}

pub(crate) fn simulate_mouse_wheel_up(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    process_mouse_scroll(MouseScrollDelta::LineDelta(0.0, 1.0), game_state);
}

pub(crate) fn simulate_mouse_wheel_down(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    process_mouse_scroll(MouseScrollDelta::LineDelta(0.0, -1.0), game_state);
}

fn process_keyboard_input(key_input: Option<VirtualKeyCode>, state: ElementState, game_state: &Arc<RwLock<GameState>>, keybinds: &Vec<Option<KeyboundFunction>>, model:  &Arc<Model>){
    if  let Some(key) = key_input {
        match state {
            ElementState::Pressed => key_action_pressed(key, game_state, keybinds, model),
            ElementState::Released => key_action_released(key, game_state, keybinds, model),
        }

    }
}

fn mouse_moved_action(x: f32, y: f32, game_state: &Arc<RwLock<GameState>>){
    let lock = game_state.read().expect("Could not read current gameState in mouse_moved_action!");
    let screen_center_pos = lock.camera_pos;
    let half_screen_width = lock.window_dimensions_ingame.0;
    let half_screen_height = lock.window_dimensions_ingame.1;
    let mouse_pos = (lock.cursor_pos_relative.0 as f64, lock.cursor_pos_relative.1 as f64);
    let w_len = lock.window_dimensions.0 as f32 / 2.0;
    let h_len = lock.window_dimensions.1 as f32 / 2.0;

    let c_p_r = ( (x - w_len) / w_len , (y - h_len) / h_len);
    let c_p_ig = (screen_center_pos.0 + mouse_pos.0* half_screen_width, screen_center_pos.1 + mouse_pos.1 * half_screen_height);

    drop(lock);
    //spend the least amount of time possible in a write lock
    let mut lock = game_state.write().expect("Could not save current cursor position to gameState!");
    lock.cursor_pos_relative = c_p_r;
    lock.cursor_pos_ingame = c_p_ig ;
    drop(lock);
    println!("Cursor moved to {} / {} -> {} / {}" , c_p_r.0, c_p_r.1, c_p_ig.0, c_p_ig.1);
}

//process mouse_wheel
pub fn process_mouse_scroll(delta: MouseScrollDelta, game_state: &Arc<RwLock<GameState>>){

    match delta {
        MouseScrollDelta::LineDelta(_horizontal, vertical) => {
            let mut lock = game_state.write().expect("Could not write to gameState on mouse scroll!");
            lock.window_dimensions_ingame = (lock.window_dimensions_ingame.0 - vertical as f64*0.1, lock.window_dimensions_ingame.1 - vertical as f64* 0.1);
        },
        MouseScrollDelta::PixelDelta(_) => println!("PixelDeltaMouseInputDetected! Not implemented"),
    }
    

}


pub fn handle_communication_loop(running: Arc<AtomicBool>, render_sender: Arc<RwLock<Vec<Vertex>>>, game_state: Arc<RwLock<GameState>>, model_pointer:  Arc<Model>){

    let mut loop_helper = LoopHelper::builder()
    .report_interval_s(0.5) // report every half a second
    .build_with_target_rate(69.0); // limit to 90 FPS if possible


    let mut current_fps = None;

    while running.load(atomic::Ordering::Relaxed){
        let _delta = loop_helper.loop_start(); // or .loop_start_s() for f64 seconds



        let mut ret_vector = Vec::new();
        let lock = game_state.read().expect("Could not read gameState in communication loop!");
        let camera_pos = lock.camera_pos;
        let win_dimensions = lock.window_dimensions_ingame;
        drop(lock);
        let lock = model_pointer.game_objects.read().unwrap();

        lock.iter().for_each(|o| o.construct_vertices(camera_pos, win_dimensions).into_iter().for_each(|v| ret_vector.push(v)));

        drop(lock);
        let lock = model_pointer.static_objects.read().unwrap();

        lock.iter().for_each(|o| o.construct_vertices(camera_pos).into_iter().for_each(|v| ret_vector.push(v)));
        drop(lock);        

        *render_sender.write().unwrap() = ret_vector;
            
        if let Some(fps) = loop_helper.report_rate() {
            current_fps = Some(fps);
        }
        if let Some(_fp) = current_fps {
            //println!("Rebuild shared vertex_buffer (Vector) per seconds: {}", _fp);
        }



        loop_helper.loop_sleep();
    }

    println!("Gracefully stopping the communications thread");
}