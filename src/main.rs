use std::{thread::Thread, sync::{Arc, RwLock}, time::Duration};

use rendering::vulkano_render;
mod rendering;
fn main(){
    
    let running = Arc::new(RwLock::new(true));

    let idle_thread_running = running.clone();

    let idle_thread = std::thread::spawn(move ||{
        while *idle_thread_running.read().unwrap(){
            ;
            println!("Oydling! Later I'll be doing stuff for the game!");
            ;
            ;
            std::thread::sleep(Duration::from_millis(4000));
        }
        println!("FINISHING!");
    });

    let threads_vec = vec![idle_thread];
    //this will lock the current thread (main in the event loop)
    vulkano_render(threads_vec, running);
}