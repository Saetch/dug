use std::{thread::{ spawn, self, JoinHandle}, sync::{Arc,  atomic::AtomicBool, RwLock}};
use constants::{WINDOW_INIT_X, WINDOW_INIT_Y};
use controller::{controller_input::ControllerInput, controller::handle_communication_loop, game_state::GameState};
use model::{model::{ Model}};
use view::renderer::Vertex;
use crate::{view::renderer::vulkano_render, controller::controller::handle_input_loop};
mod controller;
mod view;
mod drawable_object;
mod constants;
mod model;

fn main(){
    let rt = tokio::runtime::Runtime::new().unwrap();

    let (threads_vec,
        controller_sender,
        vertex_receiver,
        render_receiver,
         running)
          = start_threads();
    //this will lock the current thread (main) in the event loop. Since this creates a new Window, it should be called from the main thread,
    //otherwise it will lead to cross-platform compatibility problems
   rt.block_on(vulkano_render(threads_vec, running, controller_sender, vertex_receiver, &rt));
    
}


fn start_threads()-> (Vec<JoinHandle<()>>, flume::Sender<ControllerInput>, flume::Receiver<Vec<Vertex>>, Arc<RwLock<Vec<Vertex>>>, Arc<AtomicBool>){

    let running = Arc::new(AtomicBool::new(true));


    let game_state_arc = create_game_structs();

    let thread_running = running.clone();


    let model = Arc::new(Model::new());
    let thread_mod = model.clone();
    let model_thread = spawn(move ||{
        thread_mod.model_loop(thread_running);
    });



    let thread_mod = model.clone();

    let thread_running = running.clone();
    let thread_game_state = game_state_arc.clone();
    let (sender, receiver) = flume::unbounded::<ControllerInput>();

    let controller_thread = thread::spawn(move ||{
        handle_input_loop(thread_running, receiver, thread_game_state, thread_mod);
    });



    let thread_mod = model.clone();

    let thread_running = running.clone();


    let thread_game_state = game_state_arc.clone();
    //let (wakeup_sender, wakeup_receiver) = flume::bounded(1);             //if decided to wake up the controller communication thread instead of letting it run all the time
    let render_receiver = Arc::new(RwLock::new(Vec::new()));
    let render_sender = render_receiver.clone();

    let (vertex_sender, vertex_receiver) = flume::bounded::<Vec<Vertex>>(1);
    let controller_communication_thread = thread::spawn(move ||{
        handle_communication_loop(thread_running, vertex_sender, render_sender, thread_game_state, thread_mod);
    });


    return (vec![model_thread, controller_thread, controller_communication_thread], sender, vertex_receiver, render_receiver, running);
}

/**
 * here, the actual construction of shared data types needed for the different threads is done
 * if performance is ever going to be as critical as possible, it might be reasonable to un-objectify the model, change it to a loop type function only and share the pointers
 * to the different data field directly to the other threads, just like it's done with game_state 
 */
fn create_game_structs() -> Arc<RwLock<GameState>>{


    let game_state_arc = Arc::new(RwLock::new(GameState::new((WINDOW_INIT_X, WINDOW_INIT_Y))));

    return game_state_arc;
}