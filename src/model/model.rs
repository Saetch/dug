use std::{sync::{atomic::{AtomicBool, self, AtomicU8}, Arc}, time::Duration, thread::sleep, cell::RefCell, rc::Rc};

use futures::executor;
use rand::{thread_rng};
use tokio::sync::RwLock as AsyncRwLock;
use tokio::join;
use crate::{ model::game_object::{ GameObject}, drawable_object::static_object::{StaticObject}};

use super::game_object::debug_object::DebugObject;


pub struct Model{
    //GameObjects will inevitably be of different sizes in memory, so in Order to put them in a vector, which has a set size
    //per entry, it is needed to allocate them on the heap and only put a pointer (Box<>) in the vector

    //in order to enable simultaneous access to the model and its subsequent data types, it is needed to ensure that it is immutable
    //the only way to ensure this is to make every data field that can be changed interiorly mutable, by wrapping its type into something
    //like arc. data that is accessed by the model in the loop and only be the model itself should probably be moved into the loop itself
    pub game_objects: Arc<AsyncRwLock<Vec<Box<dyn GameObject + Send + Sync>>>>,
    pub static_objects: Arc<AsyncRwLock<Vec<StaticObject>>>,
    static_elements_per_row: AtomicU8,
   
    
}




impl Model {

    pub fn new() -> Self{
        Model{
            game_objects: Arc::new(AsyncRwLock::new(Vec::new())),
            static_objects: Arc::new(AsyncRwLock::new(Vec::new())),
            static_elements_per_row: AtomicU8::new(0),
        }
    }


    pub fn model_loop(&self, thread_running: Arc<AtomicBool>){


        executor::block_on(self.construct_game_logic());
    
    
        while thread_running.load(atomic::Ordering::Relaxed){

            println!("Idling! Later I'll be doing stuff for the game!");
            
            
            sleep(Duration::from_millis(10000));
        }
        println!("Oh no! I'm getting terminated! Brhsshh!");
    }


/**
 * logic for creating the game background and needed objects
 */
    async fn construct_game_logic(&self){   

        join!(self.construct_static_objects(), self.construct_game_objects());
    
    }



    async fn construct_static_objects(&self){

        if self.static_objects.read().await.len() > 100{
            print!("something something, more than 100 static objects?! Wow!");
        }
    }

    async fn construct_game_objects(&self){
            
        let _rng = thread_rng();
        let mut lock = self.game_objects.write().await;
        let debug_object = Box::new(DebugObject::new((0.0, 3.0), 0));
        lock.push(debug_object);
    /*
        let mut lock = self.game_objects.write().await;
        for i in 0..5500{
            let new_debug_object = DebugObject::new_with_size((i as f64 *0.4, 0.0), 2, 0.2);
    
            lock.push(Box::new(new_debug_object));
        }
        for i in 0..5500{
            let new_debug_object = DebugObject::new_with_size((i as f64 *0.4, -0.4), 3, 0.2);
    
            lock.push(Box::new(new_debug_object));
        }
        for i in 0..5500{
            let new_debug_object = DebugObject::new_with_size((i as f64 *0.4, -0.8), 3, 0.2);
    
            lock.push(Box::new(new_debug_object));
        }
*/


        //this is supposed to be the amount of static objects that make up one row in the current level.
        //e.g: the current level is a grid of 15x10, so this value will be a 15 and is used for offsetting
        self.static_elements_per_row.store(5, atomic::Ordering::Relaxed);



    }
}







