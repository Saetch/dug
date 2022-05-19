use std::{sync::{RwLock, Arc}, thread::JoinHandle};
use wgpu::Features;
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};



struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
}

impl State {
    // Creating some of the wgpu types requires async code
    async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        ).await.unwrap();   
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER,            //you can get a list of supported features by calling adapter.features() or device.features()

                limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                label: None,
            },
            None, // Trace path
        ).await.unwrap();


        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,                      //should not be 0, otherwise it might crash
            height: size.height,                    //should not be 0, otherwise it might crash
            present_mode: wgpu::PresentMode::Mailbox,           //Fifo corresponds to V-Sync, waiting for refresh, Mailbox will stop visible tearing, but impact performance slightly, immediate fastest, but with some tearing
        };
        surface.configure(&device, &config);


        Self {
            surface,
            device,
            queue,
            config,
            size,
        }
    }
     

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn input(&mut self, event: &WindowEvent) -> bool {
        false
    }

    fn update(&mut self) {
    }

    fn render(&mut self) -> Result<(), wgpu::SurfaceError> {

        //get the frame to render to
        let output = self.surface.get_current_texture()?;

        //this is a handle to a texture that can be computed
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        //an encoder is needed to construct the actual commands that get submitted to the gpu
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        //a render pass is a part of a program in which the given view is drawn to.
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        
        drop(_render_pass);

            // submit will accept anything that implements IntoIter
        self.queue.submit(std::iter::once(encoder.finish()));
        output.present();


        Ok(())
    }
}

pub(crate) async fn rendering_run(running: Arc<RwLock<bool>>, mut threads_vec: Vec<JoinHandle<()>>){
    env_logger::init();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new().with_title("Driven UnderGround!").with_visible(true).build(&event_loop).unwrap();
    
    let state_future = State::new(&window);
    let mut state = state_future.await;

        
    event_loop.run(move |event, _, control_flow| match event {
        Event::RedrawRequested(window_id) if window_id == window.id() => {
            state.update();
            match state.render() {
                Ok(_) => {}
                // Reconfigure the surface if lost
                Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                // The system is out of memory, we should probably quit
                Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,
                // All other errors (Outdated, Timeout) should be resolved by the next frame
                Err(e) => eprintln!("{:?}", e),
            }
        }
        Event::WindowEvent {
            ref event,
            window_id,
        } if window_id == window.id() => if !state.input(event) {match event {
            //These Window-Events are prebaked, we only need to know which ones to respond to and how
            WindowEvent::Resized(physical_size) => {
                state.resize(*physical_size);
            }
            WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                // new_inner_size is &&mut so we have to dereference it twice
                state.resize(**new_inner_size);
            }
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
        }

    }
    Event::MainEventsCleared => {
        // RedrawRequested will only trigger once, unless we manually
        // request it.
        //this event will be continuously submitted
        window.request_redraw();
    }
        _ => {}
    });
}