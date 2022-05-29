
use std::{io::Cursor, sync::{Arc}};
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer},
    descriptor_set::{
        layout::{
            DescriptorSetLayout, DescriptorSetLayoutCreateInfo, DescriptorSetLayoutCreationError,
        },
        PersistentDescriptorSet, WriteDescriptorSet,
    },
    device::{
        physical::{PhysicalDevice, PhysicalDeviceType},
        Device, DeviceCreateInfo, DeviceExtensions, Features, QueueCreateInfo, Queue,
    },
    format::Format,
    image::{
        view::ImageView,  ImageDimensions,  ImmutableImage, MipmapsCount,
        SwapchainImage, ImageUsage,
    },
    impl_vertex,
    instance::{Instance, InstanceCreateInfo},
    pipeline::{
        graphics::{
            color_blend::ColorBlendState,
            vertex_input::BuffersDefinition,
            viewport::{ ViewportState},
        },
        layout::PipelineLayoutCreateInfo,
        GraphicsPipeline, Pipeline,  PipelineLayout,
    },
    render_pass::{  RenderPass, Subpass},
    sampler::{Filter, Sampler, SamplerAddressMode, SamplerCreateInfo},
    swapchain::{
         Surface, Swapchain, SwapchainCreateInfo,
    },

};
use vulkano_win::VkSurfaceBuild;
use winit::{
    window::{Window, WindowBuilder}, event_loop::EventLoop,
};
use crate::rendering::renderer::{Vertex, vs, fs};
pub(crate) fn init() -> (Arc<Device>, Arc<Queue>, Arc<GraphicsPipeline>, Vec<Arc<SwapchainImage<Window>>>, Arc<RenderPass>, EventLoop<()>, Arc<Surface<Window>>, Arc<Swapchain<Window>>, Arc<PersistentDescriptorSet>, Arc<CpuAccessibleBuffer<[Vertex]>>, [Vertex; 12]){
    // instance

// surface

// physical device
// logical device
// queue creation

// swapchain

// render pass
// framebuffers
// vertex buffer
// shaders
// viewport
// pipeline
// command buffers

// event loop





// The first step of any Vulkan program is to create an instance.
//
// When we create an instance, we have to pass a list of extensions that we want to enable.
//
// All the window-drawing functionalities are part of non-core extensions that we need
// to enable manually. To do so, we ask the `vulkano_win` crate for the list of extensions
// required to draw to a window.
// tell vulkan that we need to load extensions in order to render to the viewport
let required_extensions = vulkano_win::required_extensions();

// Now creating the instance.
let instance = Instance::new(InstanceCreateInfo {
    enabled_extensions: required_extensions,
    ..Default::default()
})
.unwrap();

// create the window.
//
// This is done by creating a `WindowBuilder` from the `winit` crate, then calling the
// `build_vk_surface` method provided by the `VkSurfaceBuild` trait from `vulkano_win`. If you
// ever get an error about `build_vk_surface` being undefined in one of your projects, this
// probably means that you forgot to import this trait.
//
// This returns a `vulkano::swapchain::Surface` object that contains both a cross-platform winit
// window and a cross-platform Vulkan surface that represents the surface of the window.
    //create an EventLoop and a surface that correspons to it. Thus we will be able to handle events (changed sizes, mouse clicks, button pressed, refreshs, etc)
let event_loopi = EventLoop::new();
let surface = WindowBuilder::new()          //abstraction of object that can be drawn to. Get the actual window by calling surface.window()
    .with_title("Driven UnderGround!")
    .build_vk_surface(&event_loopi, instance.clone())
    .unwrap();

// Choose device extensions that we're going to use.
// In order to present images to a surface, we need a `Swapchain`, which is provided by the
// `khr_swapchain` extension.
let device_extensions = DeviceExtensions {
    khr_swapchain: true,
    ..DeviceExtensions::none()
};

// We then choose which physical device to use. First, we enumerate all the available physical
// devices, then apply filters to narrow them down to those that can support our needs.
let (physical_device, queue_family) = PhysicalDevice::enumerate(&instance)
    .filter(|&p| {
        // Some devices may not support the extensions or features that your application, or
        // report properties and limits that are not sufficient for your application. These
        // should be filtered out here.
        p.supported_extensions().is_superset_of(&device_extensions)
    })
    .filter_map(|p| {
        // For each physical device, we try to find a suitable queue family that will execute
        // our draw commands.
        //
        // Devices can provide multiple queues to run commands in parallel (for example a draw
        // queue and a compute queue), similar to CPU threads. This is something you have to
        // have to manage manually in Vulkan. Queues of the same type belong to the same
        // queue family.
        //
        // Here, we look for a single queue family that is suitable for our purposes. In a
        // real-life application, you may want to use a separate dedicated transfer queue to
        // handle data transfers in parallel with graphics operations. You may also need a
        // separate queue for compute operations, if your application uses those.
        p.queue_families()
            .find(|&q| 
                // We select a queue family that supports graphics operations. When drawing to
                // a window surface, as we do in this example, we also need to check that queues
                // in this queue family are capable of presenting images to the surface.
                q.supports_graphics() && q.supports_surface(&surface).unwrap_or(false)
            )
            // The code here searches for the first queue family that is suitable. If none is
            // found, `None` is returned to `filter_map`, which disqualifies this physical
            // device.
            .map(|q| (p, q))
    })
    // All the physical devices that pass the filters above are suitable for the application.
    // However, not every device is equal, some are preferred over others. Now, we assign
    // each physical device a score, and pick the device with the
    // lowest ("best") score.
    //
    // In this example, we simply select the best-scoring device to use in the application.
    // In a real-life setting, you may want to use the best-scoring device only as a
    // "default" or "recommended" device, and let the user choose the device themselves.
    .min_by_key(|(p, _)| {
        // We assign a better score to device types that are likely to be faster/better.
        match p.properties().device_type {
            PhysicalDeviceType::DiscreteGpu => 0,
            PhysicalDeviceType::IntegratedGpu => 1,
            PhysicalDeviceType::VirtualGpu => 2,
            PhysicalDeviceType::Cpu => 3,
            PhysicalDeviceType::Other => 4,
        }
    })
    .unwrap();

// Some little debug infos.
println!(
    "Using device: {} (type: {:?})",
    physical_device.properties().device_name,
    physical_device.properties().device_type,
);

// Now initializing the device. This is probably the most important object of Vulkan.
// This basically boils down to loading vulkan
// queues are our wires that can be used to talk to the GPU
// The iterator of created queues is returned by the function alongside the device.
let (device, mut queues) = Device::new(
    // Which physical device to connect to.
    physical_device,
    DeviceCreateInfo {
        // A list of optional features and extensions that our program needs to work correctly.
        // Some parts of the Vulkan specs are optional and must be enabled manually at device
        // creation. In this example the only thing we are going to need is the `khr_swapchain`
        // extension that allows us to draw to a window.
        enabled_extensions: physical_device
            // Some devices require certain extensions to be enabled if they are present
            // (e.g. `khr_portability_subset`). We add them to the device extensions that we're
            // going to enable.
            .required_extensions()
            .union(&device_extensions),
        //add necessary features for runtime buffer array
        enabled_features: Features {
                descriptor_indexing: true,
                shader_uniform_buffer_array_non_uniform_indexing: true,
                runtime_descriptor_array: true,
                descriptor_binding_variable_descriptor_count: true,
                ..Features::none()
            },

        // The list of queues that we are going to use. Here we only use one queue, from the
        // previously chosen queue family.
        queue_create_infos: vec![QueueCreateInfo::family(queue_family)],

        ..Default::default()
    },
)
.unwrap();

// Since we can request multiple queues, the `queues` variable is in fact an iterator. We
// only use one queue in this example, so we just retrieve the first and only element of the
// iterator.
let queue = queues.next().unwrap();             //we can just get the first queue from the family. These are all essentially the same, it's just that only one thread can use one queue, so in order to work with different queues, different threads are needed and vice versa, but this is quite complex

// Before we can draw on the surface, we have to create what is called a swapchain. Creating
// a swapchain allocates the color buffers that will contain the image that will ultimately
// be visible on the screen. These images are returned alongside the swapchain.
let (swapchain, images) = {
    // Querying the capabilities of the surface. When we create the swapchain we can only
    // pass values that are allowed by the capabilities.
    let surface_capabilities = physical_device
        .surface_capabilities(&surface, Default::default())
        .unwrap();

    // Choosing the internal format that the images will have.
    let image_format = Some(
        physical_device
            .surface_formats(&surface, Default::default())
            .unwrap()[0]
            .0,
    );

    // Please take a look at the docs for the meaning of the parameters we didn't mention.
    Swapchain::new(
        device.clone(),
        surface.clone(),
        SwapchainCreateInfo {                                   //this new function returns  an Arc<Swapchain<Window>> aswell as a Vec<Arc<SwapChainImage<Window>>> in a tuple ( , )
            min_image_count: surface_capabilities.min_image_count +1,       // How many buffers to use in the swapchain

            image_format,
            // The dimensions of the window, only used to initially setup the swapchain.
            // NOTE:
            // On some drivers the swapchain dimensions are specified by
            // `surface_capabilities.current_extent` and the swapchain size must use these
            // dimensions.
            // These dimensions are always the same as the window dimensions.
            //
            // However, other drivers don't specify a value, i.e.
            // `surface_capabilities.current_extent` is `None`. These drivers will allow
            // anything, but the only sensible value is the window
            // dimensions.
            //
            // Both of these cases need the swapchain to use the window dimensions, so we just
            // use that.
            image_extent: surface.window().inner_size().into(),

            image_usage: ImageUsage::color_attachment(),        // What the images are going to be used for

            // The alpha mode indicates how the alpha value of the final image will behave. For
            // example, you can choose whether the window will be opaque or transparent.
            composite_alpha: surface_capabilities
                .supported_composite_alpha
                .iter()
                .next()
                .unwrap(),

            ..Default::default()
        },
    )
    .unwrap()
};


impl_vertex!(Vertex, position, tex_i, coords);

//to draw triangles. Thus we need to put 6 vertices per texture, when we want to draw a rectangle (2 triangles together)
let vertices = [
    Vertex {                        //Vertices need to be drawn counter-clockwise in order for them to make sense
        position: [-0.1, -0.9],     //these are the coordinates on the screen for this vertex. The screen coordinates go from -1.0 (LEFT) to 1.0 (RIGHT) AND from -1.0 (UP) to 1.0 (DOWN)
        tex_i: 0,
        coords: [1.0, 0.0],         //these correspond to the coordinates on the actual image to draw, it goes from 0.0 (LEFT) to 1.0 (RIGHT)  AND from 0.0 (UP) to 1.0 (DOWN)
    },
    Vertex {
        position: [-0.9, -0.9],
        tex_i: 0,
        coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.9, -0.1],
        tex_i: 0,
        coords: [0.0, 1.0],
    },
    Vertex {
        position: [-0.1, -0.9],
        tex_i: 0,
        coords: [1.0, 0.0],
    },
    Vertex {
        position: [-0.9, -0.1],
        tex_i: 0,
        coords: [0.0, 1.0],
    },
    Vertex {
        position: [-0.1, -0.1],
        tex_i: 0,
        coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.9, -0.9],
        tex_i: 1,
        coords: [1.0, 0.0],
    },
    Vertex {
        position: [0.1, -0.9],
        tex_i: 1,
        coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.1, -0.1],
        tex_i: 1,
        coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.9, -0.9],
        tex_i: 1,
        coords: [1.0, 0.0],
    },
    Vertex {
        position: [0.1, -0.1],
        tex_i: 1,
        coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.9, -0.1],
        tex_i: 1,
        coords: [1.0, 1.0],
    },
];
let vertex_buffer =
    CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, vertices)
        .unwrap();



let vs = vs::load(device.clone()).unwrap();
let fs = fs::load(device.clone()).unwrap();

// At this point, OpenGL initialization would be finished. However in Vulkan it is not. OpenGL
// implicitly does a lot of computation whenever you draw. In Vulkan, you have to do all this
// manually.

// The next step is to create a *render pass*, which is an object that describes where the
// output of the graphics pipeline will go. It describes the layout of the images
// where the colors, depth and/or stencil information will be written.
let render_pass = vulkano::single_pass_renderpass!(
    device.clone(),
    attachments: {
        // `color` is a custom name we give to the first and only attachment.
        color: {
            // `load: Clear` means that we ask the GPU to clear the content of this
            // attachment at the start of the drawing.
            load: Clear,
            // `store: Store` means that we ask the GPU to store the output of the draw
            // in the actual image. We could also ask it to discard the result.
            store: Store,
            // `format: <ty>` indicates the type of the format of the image. This has to
            // be one of the types of the `vulkano::format` module (or alternatively one
            // of your structs that implements the `FormatDesc` trait). Here we use the
            // same format as the swapchain.
            format: swapchain.image_format(),
            // TODO:
            samples: 1,
        }
    },
    pass: {
        // We use the attachment named `color` as the one and only color attachment.
        color: [color],
        // No depth-stencil attachment is indicated with empty brackets.
        depth_stencil: {}
    }
)
.unwrap();










let (pipeline, descriptor_set) = load_sprites();


return (device, queue, pipeline, images, render_pass, event_loopi, surface,  swapchain, descriptor_set, vertex_buffer, vertices)
}