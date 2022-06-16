use std::{sync::{Arc, atomic::AtomicBool}, thread::JoinHandle, time::SystemTime};
use bytemuck::{Pod, Zeroable};
use flume::{Sender, Receiver};

use tokio::runtime::{Handle};
use wgpu::{ util::DeviceExt};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder}, dpi::{Size, PhysicalSize},
};

use crate::{controller::controller_input::{ControllerInput, MouseInputType}, constants::{WINDOW_INIT_X, WINDOW_INIT_Y}};

use super::renderer_init::{self};


    // To create a buffer that will store the shape of our triangle.
    // We use #[repr(C)] here to force rustc to not do anything funky with our data, although for this
    // particular example, it doesn't actually change the in-memory representation. This can be understood as represent this data as it would be in C Code
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
    pub struct Vertex {
        pub(crate) position: [f32; 2],
        pub(crate) tex_i: u32,
        pub(crate) tex_coords: [f32; 2],
    }

//unsafe impl bytemuck::Pod for Vertex {}   use these for implementing Pod and Zeroable for structs, that cant derive these traits
//unsafe impl bytemuck::Zeroable for Vertex {}

//create a function that returns the descriptor for the vertex, that describes how the vertices are used
impl Vertex {
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,         //defines how wide a vertex is (C: sizeof()), in order to read the next vertex, the shader will read this many bytes further into the buffer
            step_mode: wgpu::VertexStepMode::Vertex,                                    //how often the pipeline should move to the next vertex
            attributes: &[
                wgpu::VertexAttribute {                                                 //define general attributes of the vertices, here it is just a plain 1 to 1 attribution
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x2,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Uint32,
                },
                wgpu::VertexAttribute{
                    offset: (std::mem::size_of::<[f32;2]>() + std::mem::size_of::<u32>()) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}


pub(crate) async fn wgpu_render( mut threads_vec: Vec<JoinHandle<()>>, running: Arc<AtomicBool>, controller_sender: Sender<ControllerInput>, vertex_receiver: Receiver<Vec<Vertex>>, _rt: Handle) {
    env_logger::init();
    let mut ctr_sender = Some(controller_sender);
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().with_title("Driven UnderGround!").with_visible(false).with_min_inner_size(Size::Physical(PhysicalSize{width: WINDOW_INIT_X, height: WINDOW_INIT_Y})).build(&event_loop).unwrap();
    
    let (            
        surface,
        device,
        queue,
        mut config,
        mut size,
        bkcolor,
        render_pipeline,
        diffuse_bind_group,
    ) = renderer_init::new(&window);

    let mut last_render= SystemTime::now();
    window.set_visible(true);
    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            //we could trigger this Event by calling window.request_redraw(), for example in MainEventsCleared, but rendering right there is faster due to reduced function overhead
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => {match event {
            //These Window-Events are prebaked, we only need to know which ones to respond to and how
            WindowEvent::Resized(physical_size) => {
                if physical_size.width > 0 && physical_size.height > 0 {
                    size = *physical_size;
                    config.width = size.width;
                    config.height = size.height;
                    surface.configure(&device, &config);
                    if let Some(ref controller_sender) = ctr_sender{
                        controller_sender.send(ControllerInput::WindowResized { dimensions: (physical_size.width , physical_size.height )  }).expect("Could not send window resized info to the controller");
                    }
                }
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so we have to dereference it twice
                if new_inner_size.width > 0 && new_inner_size.height > 0 {
                    size = **new_inner_size;
                    config.width = size.width;
                    config.height = size.height;
                    surface.configure(&device, &config);
                }
            }
            WindowEvent::CloseRequested
             => {
                running.store(false, std::sync::atomic::Ordering::SeqCst);
                //dropping the sender will result in an Err Result on the controller thread recv() method
                ctr_sender = None;
                    (vertex_receiver).recv().unwrap();   //make the sender thread complete one more loop, in order to make it realize the running bool was set to false
                
                while let Some(cur_thread) = threads_vec.pop() {
                    cur_thread.join().unwrap();
                }
                
                *control_flow = ControlFlow::Exit;

            }
           WindowEvent::KeyboardInput { device_id: _ , input, is_synthetic: _ }
             => {
            //these if let statements are needed, since the ctr_sender is an option, meaning it can be dropped at runtime
            //this dropping is used in order to stop the controller thread. (SEE above: ctr_sender = None;) This might not be strictly MVC here, but it saves a lot of overhead, since the Window has to be managed by the view here
            if let Some(ref controller_sender) = ctr_sender{
                controller_sender.send(ControllerInput::KeyboardInput { key: input.virtual_keycode, state : input.state }).expect("Could not send keyboard input details to controller thread!");
            }
            
        }
            WindowEvent::MouseInput { device_id: _, state , button: btn, .. }
             => {
                if let Some(ref controller_sender) = ctr_sender{
                controller_sender.send(ControllerInput::MouseInput { action: MouseInputType::Click { button: *btn, state: *state } }).expect("Could not send mouse click input details to controller thread!");
                }
            }
            WindowEvent::CursorLeft { device_id: _ }
             => {
                    if let Some(ref controller_sender) = ctr_sender{

                controller_sender.send( ControllerInput::MouseInput { action: MouseInputType::LeftWindow }).expect("Could not send cursor left info to controller thread");
                }   
            }
            WindowEvent::CursorEntered { device_id: _ }
             => {
                if let Some(ref controller_sender) = ctr_sender{

                controller_sender.send( ControllerInput::MouseInput { action: MouseInputType::EnteredWindow }).expect("Could not send cursor entered info to controller thread");
                }
            }
            WindowEvent::CursorMoved { device_id: _, position, .. }
             => {

                if let Some(ref controller_sender) = ctr_sender{
                controller_sender.send( ControllerInput::MouseInput { action: MouseInputType::Move(position.x as f32, position.y as f32) }).expect("Could not send mouse moved input details to controller thread");
                }
            }
            WindowEvent::MouseWheel { device_id: _, delta, phase , ..}
             => {
                if let Some(ref controller_sender) = ctr_sender{

                controller_sender.send( ControllerInput::MouseInput { action: MouseInputType::Scroll { delta: *delta, phase: *phase } }).expect("Could not send mouse wheel input details to controller thread");
                }
            }
            _ => {}
        }

    }
    Event::MainEventsCleared => {

        let result: Result<(), wgpu::SurfaceError> = {
        
        //check for performance, this is only loosely true, since these actions are not 0 cost, but might be enough for now.
        //16.6ms are needed for 60fps (that is 16666 qs)
        let now = SystemTime::now();

        let _time_passed_in_ms = last_render.elapsed().unwrap().as_micros();

        //println!("qs passed since last rendering: {}", time_passed_in_ms);
        last_render = now;

        let vertices: Vec<Vertex> = vertex_receiver.recv().unwrap();

        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(&vertices),
                usage: wgpu::BufferUsages::VERTEX,
            });


        //get the frame to render to
        let output = surface.get_current_texture();
        if output.is_err() {
            return;
        }
        let output = output.unwrap();
        //this is a handle to a texture that can be computed
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        //an encoder is needed to construct the actual commands that get submitted to the gpu
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        //a render pass is a part of a program in which the given view is drawn to.
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {          //color attachments describe where we are going to draw to
                view: &view,                                                //created view as target, to render to the screen, this generally is the texture destination of the colors
                resolve_target: None,                                       //texture that will receive the resolved output, this is the same as view unless multisampling is enabled
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(bkcolor),
                    store: true,
                },

            }],
            depth_stencil_attachment: None,
        });
        render_pass.set_pipeline(&render_pipeline); // 2.
        render_pass.set_bind_group(0, &diffuse_bind_group, &[]);   
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..(vertices.len() as u32), 0..1);

        drop(render_pass);                     //this is needed, because in the previous step, the _render_pass object borrowed encoder mutably,
                                                //  and thus we need to drop that borrow in order to use the encoder in the next step

            // submit will accept anything that implements IntoIter
        queue.submit(std::iter::once(encoder.finish()));
        output.present();


        Ok(())};

        //this event will be continuously submitted
        match result {
            Ok(_) => {}
            // Reconfigure the surface if lost
            Err(wgpu::SurfaceError::Lost) => {
                if size.width > 0 && size.height > 0 {

                    config.width = size.width;
                    config.height = size.height;
                    surface.configure(&device, &config);
                }
            },
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
            // All other errors (Outdated, Timeout) should be resolved by the next frame
            Err(e) => eprintln!("{:?}", e),
        }
    }
        _ => {}
    });
}