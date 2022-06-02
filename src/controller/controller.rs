
use std::{sync::{Arc, atomic::{AtomicBool, self}, RwLock}};

use flume::Receiver;
use winit::event::{VirtualKeyCode, ElementState};

use crate::{controller::controller_input::MouseInputType, view::renderer::Vertex, model::{game_object::GameObject, model::Model}, drawable_object::static_object::StaticObject};

use super::{controller_input::ControllerInput, game_state::GameState, button_constants::{W_BUTTON, D_BUTTON, S_BUTTON, A_BUTTON, MOUSE_LEFT, MOUSE_RIGHT, MOUSE_MIDDLE, SPACE_BAR}};

use spin_sleep::LoopHelper;


type KeyboundFunction = fn(Arc<RwLock<GameState>>);

pub fn handle_input_loop(thread_running: Arc<AtomicBool>, receiver: Receiver<ControllerInput>, game_state: Arc<RwLock<GameState>>, model_pointer:  Arc<Model>){


    let mut keybinds = load_default_keybinds();


    while thread_running.load(atomic::Ordering::Relaxed){
        let inp = receiver.recv();
        if let Ok(input) = inp{

            //Here, the actual logic gets processed, everything around this is just to keep the loop alive and shut it down when needed
            match input{
                ControllerInput::MouseInput { action } => process_mouse_input(action, game_state.clone(), &keybinds, model_pointer.clone()),
                ControllerInput::KeyboardInput { key, state } => process_keyboard_input(key, state, game_state.clone(), &keybinds, model_pointer.clone()),
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
fn no_action(_game_state: Arc<RwLock<GameState>>){

}

//TODO, execute these actions on correct key press
fn up_action(game_state: Arc<RwLock<GameState>>){
    let mut lock = game_state.write().unwrap();

    lock.camera_movement_speed = (lock.camera_movement_speed.0 - 0.5, lock.camera_movement_speed.1);
}

fn load_default_keybinds() -> Vec<Option<KeyboundFunction>>{
    let mut ret = Vec::new();
    //TODO: add a config file for bound defaults, fallback to code, if none is present
    //see button_constants.rs, to figure out how the indices represent different keys

    ret.resize(8, None);
    let fn_pointer: KeyboundFunction = up_action;
    ret[W_BUTTON] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = no_action;
    ret[D_BUTTON] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = no_action;
    ret[S_BUTTON] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = no_action;
    ret[A_BUTTON] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = place_debug_object_action;
    ret[MOUSE_LEFT] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = no_action;
    ret[MOUSE_RIGHT] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = no_action;
    ret[MOUSE_MIDDLE] = Some(fn_pointer);
    let fn_pointer: KeyboundFunction = no_action;
    ret[SPACE_BAR] = Some(fn_pointer);
    return ret;
}


fn process_mouse_input(action: MouseInputType, game_state: Arc<RwLock<GameState>>, keybinds: &Vec<Option<KeyboundFunction>>, model_pointer:  Arc<Model>){
    match action{
        MouseInputType::Move(x, y) => mouse_moved_action(x,y, game_state.clone()),
        MouseInputType::Click { button, state } =>
            if let Some(func) = keybinds[MOUSE_LEFT] {
                func(game_state.clone())
            },
        MouseInputType::Scroll { delta: _, phase: _ } => (),
        MouseInputType::EnteredWindow => (),
        MouseInputType::LeftWindow =>(),
    }
}

fn place_debug_object_action(game_state: Arc<RwLock<GameState>>){}

fn process_keyboard_input(key_input: Option<VirtualKeyCode>, state: ElementState, game_state: Arc<RwLock<GameState>>, keybinds: &Vec<Option<KeyboundFunction>>, model_pointer:  Arc<Model>){
    if  let Some(key) = key_input {
        game_state.is_poisoned();
        println!("Input: {:?}, {:?}", key, state);

    }
}

fn mouse_moved_action(x: f32, y: f32, game_state: Arc<RwLock<GameState>>){
    let mut lock = game_state.write().expect("Could not save current cursor position to gameState!");
    let w_len = lock.window_dimensions.0 as f32 / 2.0;
    let h_len = lock.window_dimensions.1 as f32 / 2.0;
    lock.cursor_pos_relative = ( (x - w_len) / w_len , (y - h_len) / h_len);
    println!("Cursor moved to {} / {}", lock.cursor_pos_relative.0, lock.cursor_pos_relative.1);
}


pub fn handle_communication_loop(running: Arc<AtomicBool>, render_sender: Arc<RwLock<Vec<Vertex>>>, game_objects: Arc<RwLock<Vec<Box<dyn GameObject + Send + Sync>>>>, static_objects : Arc<RwLock<Vec<StaticObject>>>, game_state: Arc<RwLock<GameState>>, model_pointer:  Arc<Model>){

    let mut loop_helper = LoopHelper::builder()
    .report_interval_s(0.5) // report every half a second
    .build_with_target_rate(69.0); // limit to 90 FPS if possible


    let mut current_fps = None;

    while running.load(atomic::Ordering::Relaxed){
        let _delta = loop_helper.loop_start(); // or .loop_start_s() for f64 seconds



        let mut ret_vector = Vec::new();
        let camera_pos = game_state.read().unwrap().camera_pos;
        let lock = game_objects.read().unwrap();

        lock.iter().for_each(|o| o.construct_vertices(camera_pos).into_iter().for_each(|v| ret_vector.push(v)));

        drop(lock);
        let lock = static_objects.read().unwrap();

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