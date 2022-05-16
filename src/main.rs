use std::thread::Thread;

use rendering::vulkano_render;
mod rendering;
fn main(){

    let idle_thread = std::thread::spawn(move ||{
        while true{
            ;
            println!("Oydling!");
        }
        println!("FINISHING!");
    });

    //this will lock the current thread (main in the event loop)
    vulkano_render();
}