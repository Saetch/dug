use std::{sync::{atomic::{AtomicBool, self}, Arc, RwLock}, time::Duration, thread::sleep};

use crate::{ model::game_object::{ GameObject}, drawable_object::static_object::StaticObject};

pub fn model_loop(thread_running: Arc<AtomicBool>){

    //GameObject will inevitably be of different sizes in memory, so in Order to put them in a vector, which has a set size
    //per entry, it is needed to allocate them on the heap and only put a pointer (Box<>) in the vector
    let mut game_objects :Arc<RwLock<Vec<Box<dyn GameObject>>>> = Arc::new(RwLock::new(Vec::new()));
    let mut static_objects: Arc<RwLock<Vec<StaticObject>>> = Arc::new(RwLock::new(Vec::new()));



    let mut lock = game_objects.write().unwrap();

    lock.iter_mut().for_each(|mut e| e.process_animation(0.02));

    while thread_running.load(atomic::Ordering::Relaxed){
            
        println!("Idling! Later I'll be doing stuff for the game!");
        
        
        sleep(Duration::from_millis(4000));
    }
    println!("Oh no! I'm getting terminated! Brhsshh!");
}