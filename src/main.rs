use std::{thread::{ spawn, self, JoinHandle}, sync::{Arc,  atomic::AtomicBool, RwLock}};
use constants::{WINDOW_INIT_X, WINDOW_INIT_Y};
use controller::{controller_input::ControllerInput, controller::handle_communication_loop, game_state::GameState};
use drawable_object::static_object::StaticObject;
use model::{model::{ Model}, game_object::GameObject};
use view::renderer::Vertex;
use crate::{view::renderer::vulkano_render, controller::controller::handle_input_loop};

mod controller;
mod view;
mod drawable_object;
mod constants;
mod model;

fn main(){
    


    let (threads_vec,
        controller_sender,
        render_receiver,
         running)
          = start_threads();
    //this will lock the current thread (main) in the event loop. Since this creates a new Window, it should be called from the main thread,
    //otherwise it will lead to cross-platform compatibility problems
    vulkano_render(threads_vec, running, controller_sender, render_receiver);
}


fn start_threads()-> (Vec<JoinHandle<()>>, flume::Sender<ControllerInput>, Arc<RwLock<Vec<Vertex>>>, Arc<AtomicBool>){

    let running = Arc::new(AtomicBool::new(true));


    let (game_state_arc, game_objects, static_objects) = create_game_structs();

    let thread_running = running.clone();
    let thread_game_objects = game_objects.clone();
    let thread_static_objects = static_objects.clone();

    let model = Arc::new(Model::new());
    let thread_mod = model.clone();
    let model_thread = spawn(move ||{
        thread_mod.model_loop(thread_running, thread_game_objects, thread_static_objects);
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
    let thread_game_objects = game_objects.clone();
    let thread_static_objects = static_objects.clone();
    let thread_game_state = game_state_arc.clone();
    //let (wakeup_sender, wakeup_receiver) = flume::bounded(1);             //if decided to wake up the controller communication thread instead of letting it run all the time
    let render_receiver = Arc::new(RwLock::new(Vec::new()));
    let render_sender = render_receiver.clone();



    let controller_communication_thread = thread::spawn(move ||{
        handle_communication_loop(thread_running, render_sender, thread_game_objects, thread_static_objects, thread_game_state, thread_mod);
    });


    return (vec![model_thread, controller_thread, controller_communication_thread], sender, render_receiver, running);
}

/**
 * here, the actual construction of shared data types needed for the different threads is done
 */
fn create_game_structs() -> (Arc<RwLock<GameState>>, Arc<RwLock<Vec<Box<dyn GameObject + Send + Sync>>>>, Arc<RwLock<Vec<StaticObject>>>){
    //GameObjects will inevitably be of different sizes in memory, so in Order to put them in a vector, which has a set size
    //per entry, it is needed to allocate them on the heap and only put a pointer (Box<>) in the vector
    let game_objects :Arc<RwLock<Vec<Box<dyn GameObject + Send + Sync>>>> = Arc::new(RwLock::new(Vec::new()));
    let static_objects: Arc<RwLock<Vec<StaticObject>>> = Arc::new(RwLock::new(Vec::new()));
    let game_state_arc = Arc::new(RwLock::new(GameState::new((WINDOW_INIT_X, WINDOW_INIT_Y))));

    return (game_state_arc, game_objects, static_objects);
}