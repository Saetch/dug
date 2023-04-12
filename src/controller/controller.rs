
use std::{sync::{Arc, atomic::{AtomicBool, self}, RwLock}};

use flume::{Receiver, Sender};
use tokio::{join};
use winit::event::{VirtualKeyCode, ElementState, MouseScrollDelta};

use crate::{controller::{controller_input::MouseInputType, button_mapping::{load_default_keybinds, key_action_pressed, key_action_released}}, view::renderer::Vertex, model::{game_object::{debug_object::DebugObject}, model::Model}, drawable_object::{drawable_object::DrawableObject}};

use super::{controller_input::ControllerInput, game_state::{GameState, CamKeyPressed}, button_constants::{MOUSE_LEFT}};

use spin_sleep::LoopHelper;


pub(crate) type KeyboundFunction = fn(&Arc<RwLock<GameState>>, &Arc<Model>);

pub fn handle_input_loop(thread_running: Arc<AtomicBool>, receiver: Receiver<ControllerInput>, game_state: Arc<RwLock<GameState>>, model_pointer:  Arc<Model>){


    let keybinds = load_default_keybinds();


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

    println!("Oh no! I'm getting terminated! Brhsshh! That's the end of the handle_input_loop");

}

#[allow(dead_code)]
pub(crate) fn no_action(_game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){

}

pub(crate) fn up_action(_game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    todo!();
}

#[inline]
pub(crate) fn camera_down_action(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut lock = game_state.write().expect("Could not write to gameState in camera_down_action");
    match lock.camera_movement.1 {
        CamKeyPressed::Positive => (),
        CamKeyPressed::Negative => lock.camera_movement.1 = CamKeyPressed::Both,
        CamKeyPressed::None => lock.camera_movement.1 = CamKeyPressed::Positive,
        CamKeyPressed::Both => (),
    }
}
#[inline]
pub(crate) fn camera_up_action(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut lock = game_state.write().expect("Could not write to gameState in camera_up_action");
    match lock.camera_movement.1 {
        CamKeyPressed::Positive => lock.camera_movement.1 = CamKeyPressed::Both,
        CamKeyPressed::Negative => (),
        CamKeyPressed::None => lock.camera_movement.1 = CamKeyPressed::Negative,
        CamKeyPressed::Both => (),
    }
}
#[inline]
pub(crate) fn camera_right_action(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut lock = game_state.write().expect("Could not write to gameState in camera_right_action");
    match lock.camera_movement.0 {
        CamKeyPressed::Positive => (),
        CamKeyPressed::Negative => lock.camera_movement.0 = CamKeyPressed::Both,
        CamKeyPressed::None => lock.camera_movement.0 = CamKeyPressed::Positive,
        CamKeyPressed::Both => (),
    }
}
#[inline]
pub(crate) fn camera_left_action(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut lock = game_state.write().expect("Could not write to gameState in camera_left_action");
    match lock.camera_movement.0 {
        CamKeyPressed::Positive => lock.camera_movement.0 = CamKeyPressed::Both,
        CamKeyPressed::Negative => (),
        CamKeyPressed::None => lock.camera_movement.0 = CamKeyPressed::Negative,
        CamKeyPressed::Both => (),
    }
}
#[inline]
pub(crate) fn camera_down_action_released(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut lock = game_state.write().expect("Could not write to gameState in camera_down_action_released");
    match lock.camera_movement.1 {
        CamKeyPressed::Positive => lock.camera_movement.1 = CamKeyPressed::None,
        CamKeyPressed::Negative => (),
        CamKeyPressed::None => (),
        CamKeyPressed::Both => lock.camera_movement.1 = CamKeyPressed::Negative,
    }
}
#[inline]
pub(crate) fn camera_up_action_released(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut lock = game_state.write().expect("Could not write to gameState in camera_up_action_released");
    match lock.camera_movement.1 {
        CamKeyPressed::Positive => (),
        CamKeyPressed::Negative => lock.camera_movement.1 = CamKeyPressed::None,
        CamKeyPressed::None => (),
        CamKeyPressed::Both => lock.camera_movement.1 = CamKeyPressed::Positive,
    }
}
#[inline]
pub(crate) fn camera_right_action_released(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut lock = game_state.write().expect("Could not write to gameState in camera_right_action_released");
    match lock.camera_movement.0 {
        CamKeyPressed::Positive => lock.camera_movement.0 = CamKeyPressed::None,
        CamKeyPressed::Negative => (),
        CamKeyPressed::None => (),
        CamKeyPressed::Both => lock.camera_movement.0 = CamKeyPressed::Negative,
    }
}

#[inline]
pub(crate) fn camera_left_action_released(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut lock = game_state.write().expect("Could not write to gameState in camera_left_action_released");
    match lock.camera_movement.0 {
        CamKeyPressed::Positive => (),
        CamKeyPressed::Negative => lock.camera_movement.0 = CamKeyPressed::None,
        CamKeyPressed::None => (),
        CamKeyPressed::Both => lock.camera_movement.0 = CamKeyPressed::Positive,
    }
}

#[inline]
pub(crate) fn half_screen_width_ingame_regular(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut game_state_lock = game_state.write().unwrap();
    game_state_lock.window_dimensions_ingame = (1.0, 1.0);
}
#[inline]
pub(crate) fn half_screen_width_ingame_2times(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut game_state_lock = game_state.write().unwrap();
    game_state_lock.window_dimensions_ingame = (2.0, 2.0);
}
#[inline]
pub(crate) fn half_screen_width_ingame_point5times(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    let mut game_state_lock = game_state.write().unwrap();
    game_state_lock.window_dimensions_ingame = (0.5, 0.5);
}

#[inline]
fn process_mouse_input(action: MouseInputType, game_state: &Arc<RwLock<GameState>>, keybinds: &Vec<(Option<KeyboundFunction>, Option<KeyboundFunction>)>, model_pointer:  &Arc<Model>){
    match action{
        MouseInputType::Move(x, y) => mouse_moved_action(x,y, game_state),
        MouseInputType::Click { button: _, state } =>
        if state == ElementState::Pressed{
            //.0 for keydown action. TODO: Implement correct behavior that distinguishes between keyup, keydown
            if let Some(func) = keybinds[MOUSE_LEFT].0 {
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
    let mut lock = model.game_objects.blocking_write();
    let new_object = DebugObject::new(mouse_coords, 0);
    lock.push(Box::new(new_object));


}
#[inline]
pub(crate) fn simulate_mouse_wheel_up(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    process_mouse_scroll(MouseScrollDelta::LineDelta(0.0, 1.0), game_state);
}
#[inline]
pub(crate) fn simulate_mouse_wheel_down(game_state: &Arc<RwLock<GameState>>, _model: &Arc<Model>){
    process_mouse_scroll(MouseScrollDelta::LineDelta(0.0, -1.0), game_state);
}
#[inline]
fn process_keyboard_input(key_input: Option<VirtualKeyCode>, state: ElementState, game_state: &Arc<RwLock<GameState>>, keybinds: &Vec<(Option<KeyboundFunction>, Option<KeyboundFunction>)>, model:  &Arc<Model>){
    if  let Some(key) = key_input {
        match state {
            ElementState::Pressed => key_action_pressed(key, game_state, keybinds, model),
            ElementState::Released => key_action_released(key, game_state, keybinds, model),
        }

    }
}
#[inline]
fn mouse_moved_action(x: f32, y: f32, game_state: &Arc<RwLock<GameState>>){
    //println!("Mouse moved to ({}, {})", x, y);
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

    //println!("cursor relative: {:?}", c_p_r);
    //println!("cursor ingame: {:?}", c_p_ig);
    drop(lock);
}

/**
 * similiar to mouse_moved_action, except the mouse didn't move and the recalculating is needed because the zoom level changed
 */
#[inline]
fn recalculate_mouse_pos(game_state: &Arc<RwLock<GameState>>){
    let lock = game_state.read().expect("Could not read current gameState in recalculate_mouse_pos!");
    let screen_center_pos = lock.camera_pos;
    let half_screen_width = lock.window_dimensions_ingame.0;
    let half_screen_height = lock.window_dimensions_ingame.1;
    let mouse_pos = (lock.cursor_pos_relative.0 as f64, lock.cursor_pos_relative.1 as f64);
    drop(lock);
    let mut lock = game_state.write().expect("Could not save recalculated cursor position!");
    lock.cursor_pos_ingame = (screen_center_pos.0 +half_screen_width*mouse_pos.0, screen_center_pos.1 + half_screen_height*mouse_pos.1);
}

//process mouse_wheel
#[inline]
pub fn process_mouse_scroll(delta: MouseScrollDelta, game_state: &Arc<RwLock<GameState>>){

    match delta {
        MouseScrollDelta::LineDelta(_horizontal, vertical) => {
            let mut lock = game_state.write().expect("Could not write to gameState on mouse scroll!");
            lock.window_dimensions_ingame = (lock.window_dimensions_ingame.0 - vertical as f64*0.1, lock.window_dimensions_ingame.1 - vertical as f64* 0.1);
            drop(lock); //could have given ownership to recalculate_mouse_pos instead and thus circumvented another read->write access, but this type of performance optimization should not be necessary
            recalculate_mouse_pos(game_state);
        },
        MouseScrollDelta::PixelDelta(_) => println!("PixelDeltaMouseInputDetected! Not implemented"),
    }

    
    

}

#[inline]
pub async fn handle_communication_loop(running: Arc<AtomicBool>, vertex_sender: Sender<Vec<Vertex>>, game_state: Arc<RwLock<GameState>>, model_pointer:  Arc<Model>){

   
    let mut loop_helper = LoopHelper::builder()
    .report_interval_s(1.0) 
    .build_with_target_rate(102.0); // limit to FPS if possible
    let mut current_fps = None;
    let mut delta: f64 = 0.0;
    while running.load(atomic::Ordering::Relaxed){
        //let delta = now.duration_since(last_executed).unwrap().as_secs_f64();
        let lock = game_state.read().expect("Could not read gameState in communication loop!");
        let speed = lock.cam_speed as f64;
        let cam_mov:(f64, f64) = (match &lock.camera_movement.0 {
            CamKeyPressed::Positive => 1.0,
            CamKeyPressed::Negative => -1.0,
            CamKeyPressed::None => 0.0,
            CamKeyPressed::Both => 0.0,
        }, match &lock.camera_movement.1 {
            CamKeyPressed::Positive => 1.0,
            CamKeyPressed::Negative => -1.0,
            CamKeyPressed::None => 0.0,
            CamKeyPressed::Both => 0.0,
        });
        let camera_pos = lock.camera_pos;
        let win_dimensions = lock.window_dimensions_ingame;
        drop(lock);
        let new_cam_pos = (cam_mov.0 * speed *win_dimensions.0 * delta + camera_pos.0, cam_mov.1 * speed* win_dimensions.1 *  delta + camera_pos.1);
        let vec1fut = iterate_through_static_objects(&model_pointer, new_cam_pos, win_dimensions);
        let vec2fut = iterate_through_game_objects(&model_pointer, new_cam_pos, win_dimensions);
        
        let (mut ret_vector, additional_vector) = join!(vec1fut, vec2fut);  //this is async, but single threaded, which will result in the computation continuing even if one of the two vectors are currently occupied
        ret_vector.extend(additional_vector);
        match vertex_sender.send(ret_vector){
            Ok(_) => (),
            Err(e) => println!("{:?}", e),
        }
        if let Some(fps) = loop_helper.report_rate() {
            current_fps = Some(fps);
            //println!("FPS: {:?}", current_fps);

        }

        loop_helper.loop_sleep(); // sleeps to achieve a X FPS rate This is a crate function and not just a regular sleep

        game_state.write().unwrap().camera_pos = new_cam_pos;
        delta = loop_helper.loop_start_s(); // or .loop_start_s() for f64 seconds






    }

    println!("Gracefully stopping the communications thread");
}

#[inline]
async fn iterate_through_static_objects(model: &Arc<Model>, new_cam_pos:(f64, f64), win_dimensions: (f64,f64)) -> Vec<Vertex>{
    let lock = model.static_objects.read().await;
    lock.iter().map(|o| o.construct_vertices(new_cam_pos, win_dimensions)).into_iter().flatten().collect()
}


#[inline]
async fn iterate_through_game_objects(model: &Arc<Model>, new_cam_pos:(f64, f64), win_dimensions: (f64,f64)) -> Vec<Vertex>{
    let lock = model.game_objects.read().await;
    lock.iter().map(|o| o.construct_vertices(new_cam_pos, win_dimensions)).into_iter().flatten().collect()
}