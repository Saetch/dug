
use std::sync::{Arc, atomic::{AtomicBool, self}};

use flume::Receiver;

use crate::controller::controller_input::MouseInputType;

use super::controller_input::ControllerInput;

pub fn handle_input_loop(thread_running: Arc<AtomicBool>, receiver: Receiver<ControllerInput>){

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


}


fn process_mouse_input(action: MouseInputType){
    match action{
        MouseInputType::Move(x, y) => mouse_moved_action(x,y),
        MouseInputType::Click { button, state } => (),
        MouseInputType::Scroll { delta, phase } => (),
        MouseInputType::EnteredWindow => (),
        MouseInputType::LeftWindow =>(),
    }
}

//TODO: the actual size of the Window should be compared to these values, so the location of the cursor on the viewPort can be calculated
fn mouse_moved_action(x: f64, y: f64){
    println!("Mouse moved to: {} / {}",x,y);
}