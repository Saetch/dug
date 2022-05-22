use std::{sync::{RwLock, Arc}, thread::JoinHandle, time::SystemTime};
use bytemuck::{Pod, Zeroable, bytes_of};
use rand::{thread_rng, Rng};
use wgpu::{Features, include_wgsl, util::DeviceExt, Operations};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::{WindowBuilder, Window},
};
use std::time::Duration;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
struct Vertex {
    position: [f32; 3],
    tex_coords: [f32; 2],
}
    //create the actual vertices that should be drawn. This could be updated at compile time
    const VERTICES: &[Vertex] = &[
        Vertex { position: [-0.0868241, 0.49240386, 0.0], tex_coords: [0.4131759, 0.00759614], }, // A
        Vertex { position: [-0.49513406, 0.06958647, 0.0], tex_coords: [0.0048659444, 0.43041354], }, // B
        Vertex { position: [-0.21918549, -0.44939706, 0.0], tex_coords: [0.28081453, 0.949397], }, // C
        Vertex { position: [0.35966998, -0.3473291, 0.0], tex_coords: [0.85967, 0.84732914], }, // D
        Vertex { position: [0.44147372, 0.2347359, 0.0], tex_coords: [0.9414737, 0.2652641], }, // E
    ];
    
    const INDICES: &[u16] = &[
        0, 1, 4,
        1, 2, 4,
        2, 3, 4,
    ];

//unsafe impl bytemuck::Pod for Vertex {}   use these for implementing Pod and Zeroable for structs, that cant derive these traits
//unsafe impl bytemuck::Zeroable for Vertex {}

//create a function that returns the descriptor for the vertex, that describes how the vertices are used
impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,         //defines how wide a vertex is (C: sizeof()), in order to read the next vertex, the shader will read this many bytes further into the buffer
            step_mode: wgpu::VertexStepMode::Vertex,                                    //how often the pipeline should move to the next vertex
            attributes: &[
                wgpu::VertexAttribute {                                                 //define general attributes of the vertices, here it is just a plain 1 to 1 attribution
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                }
            ]
        }
    }
}
struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,

    render_pipeline: wgpu::RenderPipeline,  //The render pipeline is needed for drawing onto a surface, using shaders
    vertex_buffer: wgpu::Buffer,
    index_buffer: wgpu::Buffer, 
    num_indices: u32,
    diffuse_bind_group: wgpu::BindGroup, 

    bkcolor: wgpu::Color,
    last_render: SystemTime,
}

impl State {

    // Creating some of the wgpu types requires async code
    // in order to use these, the new function needs to be async und thus the whole rendering function, but since it does not return anything, we need pollster in main to block and wait
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


        //loading an image from a file
        let diffuse_bytes = include_bytes!("../image_img.png");
        let diffuse_image = image::load_from_memory(diffuse_bytes).unwrap();
        let diffuse_rgba = diffuse_image.to_rgba8();

        use image::GenericImageView;
        let dimensions = diffuse_image.dimensions();

        let texture_size = wgpu::Extent3d {
            width: dimensions.0,
            height: dimensions.1,
            depth_or_array_layers: 1,
        };


        let diffuse_texture = device.create_texture(
            &wgpu::TextureDescriptor {
                // All textures are stored as 3D, we represent our 2D texture
                // by setting depth to 1.
                size: texture_size,
                mip_level_count: 1, 
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                // Most images are stored using sRGB so we need to reflect that here.
                format: wgpu::TextureFormat::Rgba8UnormSrgb,
                // TEXTURE_BINDING tells wgpu that we want to use this texture in shaders
                // COPY_DST means that we want to copy data to this texture
                usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::COPY_DST,
                label: Some("diffuse_texture"),
            }
        );



        queue.write_texture(
            // Tells wgpu where to copy the pixel data
            wgpu::ImageCopyTexture {
                texture: &diffuse_texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            // The actual pixel data
            diffuse_rgba.into_raw().as_slice(),
            // The layout of the texture
            wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: std::num::NonZeroU32::new(4 * dimensions.0),
                rows_per_image: std::num::NonZeroU32::new(dimensions.1),
            },
            texture_size,
        );
        

        // We don't need to configure the texture view much, so let's
        // let wgpu define it.
        let diffuse_texture_view = diffuse_texture.create_view(&wgpu::TextureViewDescriptor::default());        //create a handle to access the texture we just created
        let diffuse_sampler = device.create_sampler(&wgpu::SamplerDescriptor {                                      //a sampler will accept coordinates (X/Y) and return the color data. So this object is asked when the texture is the source of any color operation
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Nearest,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()                        //rest of the fields are initialized with default values
        });

        //bind groups describe resources that a shaders has access to
        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[                     //2 Entries: 1st: Texture, 2nd: Sampler for texture
                    wgpu::BindGroupLayoutEntry {    
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        // This should match the filterable field of the
                        // corresponding Texture entry above.
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });


            //create the actual bind group based on the bind-group-layout. This looks almost identical tho, but it means you could switch these out
            let diffuse_bind_group = device.create_bind_group(
                &wgpu::BindGroupDescriptor {
                    layout: &texture_bind_group_layout,
                    entries: &[
                        wgpu::BindGroupEntry {
                            binding: 0,
                            resource: wgpu::BindingResource::TextureView(&diffuse_texture_view),
                        },
                        wgpu::BindGroupEntry {
                            binding: 1,
                            resource: wgpu::BindingResource::Sampler(&diffuse_sampler),
                        }
                    ],
                    label: Some("diffuse_bind_group"),
                }
            );
            


        let shader = device.create_shader_module(&include_wgsl!("shader.wgsl"));       //here, we could also put the contents of shader.wgsl as a String into the program, but loading it from a file is more convenient. Make sure to have WGSL extension installed if you want to edit the shader.wgsl file
        

        let render_pipeline_layout =
        device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &[&texture_bind_group_layout],
            push_constant_ranges: &[],
        });


        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main", // 1.
                buffers: &[
                    Vertex::desc(),                                 //insert the vertex buffer that was created above
                ], // 2.
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


        let vertex_buffer = device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(VERTICES),
                usage: wgpu::BufferUsages::VERTEX,
            });

            let moment = SystemTime::now();


            let index_buffer = device.create_buffer_init(
                &wgpu::util::BufferInitDescriptor {
                    label: Some("Index Buffer"),
                    contents: bytemuck::cast_slice(INDICES),
                    usage: wgpu::BufferUsages::INDEX,
                }
            );
            let num_indices = INDICES.len() as u32;
            
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
            render_pipeline: render_pipeline,
            vertex_buffer: vertex_buffer,
            last_render: moment,
            index_buffer: index_buffer,
            num_indices: num_indices,
            diffuse_bind_group: diffuse_bind_group
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

        //check for performance, this is only loosely true, since these actions are not 0 cost, but might be enough for now.
        //16.6ms are needed for 60fps (that is 16666 qs)
        let now = SystemTime::now();

        let time_passed_in_ms = self.last_render.elapsed().unwrap().as_micros();

        println!("qs passed since last rendering: {}", time_passed_in_ms);
        self.last_render = now;



        //get the frame to render to
        let output = self.surface.get_current_texture()?;

        //this is a handle to a texture that can be computed
        let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());

        //an encoder is needed to construct the actual commands that get submitted to the gpu
        let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        //a render pass is a part of a program in which the given view is drawn to.
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Render Pass"),
            color_attachments: &[wgpu::RenderPassColorAttachment {          //color attachments describe where we are going to draw to
                view: &view,                                                //created view as target, to render to the screen, this generally is the texture destination of the colors
                resolve_target: None,                                       //texture that will receive the resolved output, this is the same as view unless multisampling is enabled
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(self.bkcolor),
                    store: true,
                },

            }],
            depth_stencil_attachment: None,
        });
            // NEW!
         render_pass.set_pipeline(&self.render_pipeline); // 2.
        render_pass.set_bind_group(0, &self.diffuse_bind_group, &[]);   
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.set_index_buffer(self.index_buffer.slice(..), wgpu::IndexFormat::Uint16); // 1.
        render_pass.draw_indexed(0..self.num_indices, 0, 0..1); // 3. 
        drop(render_pass);                     //this is needed, because in the previous step, the _render_pass object borrowed encoder mutably,
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
            //we could trigger this Event by calling window.request_redraw(), for example in MainEventsCleared, but rendering right there is faster due to reduced function overhead
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

        //this event will be continuously submitted
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
        _ => {}
    });
}