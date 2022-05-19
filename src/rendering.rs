use std::{sync::{RwLock, Arc}, thread::JoinHandle};
use rand::{thread_rng, Rng};
use wgpu::{Features, include_wgsl};
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

    render_pipeline: wgpu::RenderPipeline,  //The render pipeline is needed for drawing onto a surface, using shaders

    bkcolor: wgpu::Color,
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
                    println!("loading web configuration");
                    wgpu::Limits::downlevel_webgl2_defaults()
                    
                } else {
                    println!("loading default configuration");
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


        let shader = device.create_shader_module(&include_wgsl!("shader.wgsl"));       //here, we could also put the contents of shader.wgsl as a String into the program, but loading it from a file is more convenient. Make sure to have WGSL extension installed if you want to edit the shader.wgsl file
        

        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });


        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[], // 2.
            },
            fragment: Some(wgpu::FragmentState { // 3.              //fragment is optional and thus wrapped in Some(), this is needed for storing color on the surface
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState { // 4.
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),         //replace pixels instead of blending
                    write_mask: wgpu::ColorWrites::ALL,             //specify color channels (R, G, B or similiar) that can be written to. Others will be ignored 
                }],
            }),    
                primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList, // 1.  //every 3 vertices in order are considered a triangle
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw, // 2.            //Ccw: Counter-clockwise. This means, that if the vertices are ordered counter-clockwise, the triangle is facing us (only the front is visible)
                cull_mode: Some(wgpu::Face::Back),
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },    depth_stencil: None, // 1.
            multisample: wgpu::MultisampleState {
                count: 1, // 2.
                mask: !0, // 3.
                alpha_to_coverage_enabled: false, // 4.
            },
            multiview: None, // 5.
        });


        Self {
            surface,
            device,
            queue,
            config,
            size,
            bkcolor: wgpu::Color {            
                r: 0.1,
                g: 0.2,
                b: 0.3,
                a: 1.0,
            },
            render_pipeline: render_pipeline
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

        
        match event {
            WindowEvent::CursorMoved { device_id: _, position: _, modifiers: _ } =>{
                let mut rng = thread_rng();
                let val_changed = rng.gen_range(-0.005..=0.005);
                let typechanged : u8 = rng.gen_range(0..=2);
                match typechanged {
                    0 => self.bkcolor.r += val_changed,
                    1 => self.bkcolor.g +=val_changed,
                    2 => self.bkcolor.b += val_changed,
                    _ => ()
                    
                }
            } 
            _ => ()
        }
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
            color_attachments: &[wgpu::RenderPassColorAttachment {          //color attackments describe where we are going to draw to
                view: &view,                                                //created view as target, to render to the screen, this generally is the texture destination of the colors
                resolve_target: None,                                       //texture that will receive the resolved output, this is the same as view unless multisampling is enabled
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.bkcolor),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        
        drop(_render_pass);                     //this is needed, because in the previous step, the _render_pass object borrowed encoder mutably,
                                                //  and thus we need to drop that borrow in order to use the encoder in the next step

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
                *running.write().unwrap() = false;                          //because no reference to running is saved, the lock is dropped immediately
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