use std::{sync::{RwLock, Arc}, thread::JoinHandle};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};






pub(crate) fn rendering_run(running: Arc<RwLock<bool>>, mut threads_vec: Vec<JoinHandle<()>>){
    env_logger::init();
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new().build(&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => match event {
            WindowEvent::CloseRequested
            | WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => {
                *running.write().unwrap() = false;
                while let Some(thr) = threads_vec.pop(){
                    thr.join().unwrap();
                }
                *control_flow = ControlFlow::Exit
            },
            _ => {}
        },
        _ => {}
    });
}