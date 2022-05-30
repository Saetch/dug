use std::{thread::{ spawn, sleep}, sync::{Arc,  atomic::AtomicBool}, time::Duration};
use std::sync::atomic;
use crate::view::renderer::vulkano_render;
mod view;
fn main(){
    
    let running = Arc::new(AtomicBool::new(true));

    let idle_thread_running = running.clone();

    let idle_thread = spawn(move ||{
        while idle_thread_running.load(atomic::Ordering::Relaxed){
            
            println!("Idling! Later I'll be doing stuff for the game!");
            
            
            sleep(Duration::from_millis(4000));
        }
        println!("Oh no! I'm getting terminated! Brhsshh!");
    });

    let threads_vec = vec![idle_thread];
    //this will lock the current thread (main) in the event loop. Since this creates a new Window, it should be called from the main thread,
    //otherwise it will lead to cross-platform compatibility problems
    vulkano_render(threads_vec, running);
}