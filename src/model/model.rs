use std::{sync::{atomic::{AtomicBool, self}, Arc, RwLock}, time::Duration, thread::sleep};

use rand::{thread_rng, Rng};

use crate::{ model::game_object::{ GameObject}, drawable_object::static_object::{StaticObject}};

use super::game_object::debug_object::DebugObject;


pub struct Model{

}



impl Model {

    pub fn new() -> Self{
        Model{}
    }


    pub fn model_loop(&self, thread_running: Arc<AtomicBool>, game_objects: Arc<RwLock<Vec<Box<dyn GameObject + Send + Sync>>>>, static_objects : Arc<RwLock<Vec<StaticObject>>>){


        self.construct_game_logic(game_objects.clone(), static_objects.clone());
    
    
        while thread_running.load(atomic::Ordering::Relaxed){
                
            println!("Idling! Later I'll be doing stuff for the game!");
            
            
            sleep(Duration::from_millis(4000));
        }
        println!("Oh no! I'm getting terminated! Brhsshh!");
    }



    fn construct_game_logic(&self, game_objects: Arc<RwLock<Vec<Box<dyn GameObject + Send + Sync>>>>, static_objects : Arc<RwLock<Vec<StaticObject>>>){

        //do logic for creating the game background
    
        let mut rng = thread_rng();
        if static_objects.read().expect("Could not access static_objects for initialization!").len() > 100{
            print!("something something, more than 100 static objects?!");
        }
    
        let mut lock = game_objects.write().expect("Could not access game objects for initialization!");
        for _ in 0..10{
            let new_debug_object = DebugObject::new((rng.gen_range(-0.5 ..= 0.5), rng.gen_range(-0.5 ..= 0.5)));
    
            lock.push(Box::new(new_debug_object));
        }
    
    }
}






