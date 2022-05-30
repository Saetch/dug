
use crate::controller::controller_input::MouseInputType;

use super::controller_input::ControllerInput;

pub fn handle_input(input: ControllerInput){

    match input{
        ControllerInput::MouseInput { action } => process_mouse_input(action),
        ControllerInput::KeyboardInput { key: _key, state: _state } => (),
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