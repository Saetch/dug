

use wgpu::{include_wgsl, SurfaceConfiguration, Surface, Device, RenderPipeline, BindGroup, Queue};
use winit::{window::Window, dpi::PhysicalSize};

use super::{sprite_loading};




    // Creating some of the wgpu types requires async code
    // in order to use these, the new function needs to be async und thus the whole rendering function, but since it does not return anything, we need pollster in main to block and wait
    pub fn new(window: &Window) -> (Surface, Device, Queue, SurfaceConfiguration, PhysicalSize<u32>, wgpu::Color, RenderPipeline, BindGroup) {

        let size = window.inner_size();

        // The instance is a handle to our GPU
        // Backends::all => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::Backends::VULKAN);
        let surface = unsafe { instance.create_surface(window) };
        let adapter = pollster::block_on(instance.request_adapter(
            &wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            },
        )).unwrap();   
        let (device, queue) = pollster::block_on(adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: (wgpu::Features::ADDRESS_MODE_CLAMP_TO_BORDER | /* <-- this is a bitwise operator, not a logical OR */ wgpu::Features::SAMPLED_TEXTURE_AND_STORAGE_BUFFER_ARRAY_NON_UNIFORM_INDEXING
                | wgpu::Features::TEXTURE_BINDING_ARRAY),            //you can get a list of supported features by calling adapter.features() or device.features()

                limits: wgpu::Limits::default(),
                
                label: None,
            },
            None, // Trace path
        )).unwrap();


        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,                      //should not be 0, otherwise it might crash
            height: size.height,                    //should not be 0, otherwise it might crash
            present_mode: wgpu::PresentMode::Fifo,           //Fifo corresponds to V-Sync, waiting for refresh, Mailbox will stop visible tearing, but impact performance slightly, immediate fastest, but with some tearing
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(&include_wgsl!("shader.wgsl"));       //here, we could also put the contents of shader.wgsl as a String into the program, but loading it from a file is more convenient. Make sure to have WGSL extension installed if you want to edit the shader.wgsl file

        //the i value is supposed to be for which part of the game is loaded -> e.g: not all images need to be loaded, switch them out when loading another level
        let (render_pipeline, diffuse_bind_group) = sprite_loading::load_sprites(0, &device, &queue, &shader, &config);





            


        






            
        (
            surface,
            device,
            queue,
            config,
            size,
            wgpu::Color {            
                r: 0.2,
                g: 0.2,
                b: 0.2,
                a: 1.0,
            },
            render_pipeline,
            diffuse_bind_group,
        )
    }
     

 