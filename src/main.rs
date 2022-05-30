use std::{thread::{ spawn, sleep, self, JoinHandle}, sync::{Arc,  atomic::AtomicBool}, time::Duration};
use std::sync::atomic;
use controller::controller_input::ControllerInput;

use crate::{view::renderer::vulkano_render, controller::controller::handle_input_loop};
mod controller;
mod view;
fn main(){
    


    let (threads_vec,
        controller_sender,
         running)
          = start_threads();
    //this will lock the current thread (main) in the event loop. Since this creates a new Window, it should be called from the main thread,
    //otherwise it will lead to cross-platform compatibility problems
    vulkano_render(threads_vec, running, controller_sender);
}


fn start_threads()-> (Vec<JoinHandle<()>>, flume::Sender<ControllerInput>, Arc<AtomicBool>){



    let (sender, receiver) = flume::unbounded::<ControllerInput>();

    let running = Arc::new(AtomicBool::new(true));

    let thread_running = running.clone();

    let model_thread = spawn(move ||{
        while thread_running.load(atomic::Ordering::Relaxed){
            
            println!("Idling! Later I'll be doing stuff for the game!");
            
            
            sleep(Duration::from_millis(4000));
        }
        println!("Oh no! I'm getting terminated! Brhsshh!");
    });

    let thread_running = running.clone();

    let controller_thread = thread::spawn(move ||{


        handle_input_loop(thread_running, receiver);

        println!("Oh no! I'm getting terminated! Brhsshh!");
    });


    return (vec![model_thread, controller_thread], sender, running);
}