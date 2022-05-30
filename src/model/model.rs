use std::{sync::{atomic::{AtomicBool, self}, Arc}, time::Duration, thread::sleep};

pub fn model_loop(thread_running: Arc<AtomicBool>){



    while thread_running.load(atomic::Ordering::Relaxed){
            
        println!("Idling! Later I'll be doing stuff for the game!");
        
        
        sleep(Duration::from_millis(4000));
    }
    println!("Oh no! I'm getting terminated! Brhsshh!");
}