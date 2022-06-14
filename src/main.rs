use std::{thread::{ spawn, sleep}, sync::{Arc, RwLock}, time::Duration, net::{TcpStream, TcpListener}};

use rendering::rendering_run;

mod rendering;
fn main(){
    
    let running = Arc::new(RwLock::new(true));

    let idle_thread_running = running.clone();

    let idle_thread = spawn(move ||{
        while *idle_thread_running.read().unwrap(){
            
            println!("Idling! Later I'll be doing stuff for the game!");
            

            sleep(Duration::from_millis(4000));
        }
        println!("Oh no! I'm getting terminated! Brhsshh!");
    });

    let threads_vec = vec![idle_thread];
    //this will lock the current thread (main) in the event loop. Since this creates a new Window, it should be called from the main thread,
    //otherwise it will lead to cross-platform compatibility problems

    pollster::block_on(rendering_run(running, threads_vec));
}


pub fn primes_below(n: u64) -> Vec<u64> {
    let mut primes = vec![2];
    let mut i = 3;
    while i < n {
        if is_prime(i) {
            primes.push(i);
        }
        i += 2;
    }
    primes
}

pub fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }
        i += 2;
    }
    true
}