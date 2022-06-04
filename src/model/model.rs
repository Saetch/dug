use std::{sync::{atomic::{AtomicBool, self}, Arc, RwLock}, time::Duration, thread::sleep};

use rand::{thread_rng};

use crate::{ model::game_object::{ GameObject}, drawable_object::static_object::{StaticObject}};

use super::game_object::debug_object::DebugObject;


pub struct Model{
    //GameObjects will inevitably be of different sizes in memory, so in Order to put them in a vector, which has a set size
    //per entry, it is needed to allocate them on the heap and only put a pointer (Box<>) in the vector

    //in order to enable simultaneous access to the model and its subsequent data types, it is needed to ensure that it is immutable
    //the only way to ensure this is to make every data field that can be changed interiorly mutable, by wrapping its type into something
    //like arc. data that is accessed by the model in the loop and only be the model itself should probably be moved into the loop itself
    pub game_objects: Arc<RwLock<Vec<Box<dyn GameObject + Send + Sync>>>>,
    pub static_objects: Arc<RwLock<Vec<StaticObject>>>,
   
}



impl Model {

    pub fn new() -> Self{
        Model{
            game_objects: Arc::new(RwLock::new(Vec::new())),
            static_objects: Arc::new(RwLock::new(Vec::new())),
           
        }
    }


    pub fn model_loop(&self, thread_running: Arc<AtomicBool>){


        self.construct_game_logic();
    
    
        while thread_running.load(atomic::Ordering::Relaxed){

            println!("Idling! Later I'll be doing stuff for the game!");
            
            
            sleep(Duration::from_millis(10000));
        }
        println!("Oh no! I'm getting terminated! Brhsshh!");
    }



    fn construct_game_logic(&self){

        //do logic for creating the game background
    
        let _rng = thread_rng();
        if self.static_objects.read().expect("Could not access static_objects for initialization!").len() > 100{
            print!("something something, more than 100 static objects?!");
        }
    
        let mut lock = self.game_objects.write().expect("Could not access game objects for initialization!");
        for i in 0..100_000{
            let new_debug_object = DebugObject::new((i as f64 *0.4, 0.0));
    
            lock.push(Box::new(new_debug_object));
        }
    
    }
}






