use std::{sync::{atomic::{AtomicBool, self}, Arc, RwLock}, time::Duration, thread::sleep};

use crate::{ model::game_object::{ GameObject}, drawable_object::static_object::{StaticObject}};

pub fn model_loop(thread_running: Arc<AtomicBool>, game_objects: Arc<RwLock<Vec<Box<dyn GameObject + Send + Sync>>>>, static_objects : Arc<RwLock<Vec<StaticObject>>>){


    loop {
        let mut lock = game_objects.write().unwrap();
        *lock = Vec::new();

        for _ in 0..5000  {
            
        }


        break;
    }

    while thread_running.load(atomic::Ordering::Relaxed){
            
        println!("Idling! Later I'll be doing stuff for the game!");
        
        
        sleep(Duration::from_millis(4000));
    }
    println!("Oh no! I'm getting terminated! Brhsshh!");
}