use std::{thread::JoinHandle, sync::{Arc, atomic::AtomicBool}};

use flume::{Sender, Receiver};
use tokio::runtime::Handle;

use crate::controller::controller_input::ControllerInput;

use super::renderer::Vertex;

pub(crate) async fn go( mut threads_vec: Vec<JoinHandle<()>>, running: Arc<AtomicBool>, controller_sender: Sender<ControllerInput>, vertex_receiver: Receiver<Vec<Vertex>>, _rt: Handle){
    println!("Did not open a window");
    loop{
        
    }
}