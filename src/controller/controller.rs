
use std::sync::{Arc, atomic::{AtomicBool, self}, RwLock};

use flume::Receiver;

use crate::{controller::controller_input::MouseInputType, view::renderer::Vertex, model::game_object::GameObject, drawable_object::static_object::StaticObject};

use super::{controller_input::ControllerInput, game_state::GameState};

pub fn handle_input_loop(thread_running: Arc<AtomicBool>, receiver: Receiver<ControllerInput>, _game_state: Arc<RwLock<GameState>>){

    while thread_running.load(atomic::Ordering::Relaxed){
        let inp = receiver.recv();
        if let Ok(input) = inp{

            //Here, the actual logic gets processed, everything around this is just to keep the loop alive and shut it down when needed
            match input{
                ControllerInput::MouseInput { action } => process_mouse_input(action),
                ControllerInput::KeyboardInput { key: _key, state: _state } => (),
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


fn process_mouse_input(action: MouseInputType){
    match action{
        MouseInputType::Move(x, y) => mouse_moved_action(x,y),
        MouseInputType::Click { button: _, state: _ } => (),
        MouseInputType::Scroll { delta: _, phase: _ } => (),
        MouseInputType::EnteredWindow => (),
        MouseInputType::LeftWindow =>(),
    }
}

//TODO: the actual size of the Window should be compared to these values, so the location of the cursor on the viewPort can be calculated
fn mouse_moved_action(x: f64, y: f64){
    println!("Mouse moved to: {} / {}",x,y);
}


pub fn handle_communication_loop(running: Arc<AtomicBool>, render_sender: single_value_channel::Updater<Option<Vec<Vertex>>>, game_objects: Arc<RwLock<Vec<Box<dyn GameObject + Send + Sync>>>>, static_objects : Arc<RwLock<Vec<StaticObject>>>, game_state: Arc<RwLock<GameState>>){



    while running.load(atomic::Ordering::Relaxed){

        let mut ret_vector = Vec::new();

        let lock = game_objects.read().unwrap();

        lock.iter().for_each(|o| o.construct_vertices(game_state.read().unwrap().camera_pos).into_iter().for_each(|v| ret_vector.push(v)));

        let lock = static_objects.read().unwrap();
        
        

        render_sender.update(Some(ret_vector));

    }
}