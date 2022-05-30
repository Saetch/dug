use bytemuck::{Pod, Zeroable};
use rand::Rng;
use std::{sync::{Arc, atomic::AtomicBool, RwLock}, thread::JoinHandle, time::SystemTime};
use vulkano::{
    buffer::{BufferUsage, CpuAccessibleBuffer, TypedBufferAccess},
    command_buffer::{
        AutoCommandBufferBuilder, CommandBufferUsage, RenderPassBeginInfo, SubpassContents,
    },
    image::{
        view::ImageView, ImageAccess,
        SwapchainImage,
    },
    pipeline::{
        graphics::{
            viewport::{Viewport},
        }, Pipeline, PipelineBindPoint,
    },
    render_pass::{Framebuffer, FramebufferCreateInfo, RenderPass},
    swapchain::{
        acquire_next_image, AcquireError, SwapchainCreateInfo, SwapchainCreationError,
    },
    sync::{self, FlushError, GpuFuture},
};

use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow},
    window::{Window},
};
use crate::view::renderer_init::*;


    // To create a buffer that will store the shape of our triangle.
    // We use #[repr(C)] here to force rustc to not do anything funky with our data, although for this
    // particular example, it doesn't actually change the in-memory representation. This can be understood as represent this data as it would be in C Code
    #[repr(C)]
    #[derive(Clone, Copy, Debug, Default, Zeroable, Pod)]
    pub(crate) struct Vertex {
        pub(crate) position: [f32; 2],
        pub(crate) tex_i: u32,
        pub(crate) coords: [f32; 2],
    }



    

pub(crate) fn vulkano_render(mut threads_vec : Vec<JoinHandle<()>>, running : Arc<AtomicBool>) {
    
    
    let (device, queue, pipeline, images, render_pass, event_loop
    , surface,mut swapchain, descriptor_set)
     = init();
    

    // Dynamic viewports allow us to recreate just the viewport when the window is resized
    // Otherwise we would have to recreate the whole pipeline.
    let mut viewport = Viewport {
        origin: [0.0, 0.0],
        dimensions: [0.0, 0.0],
        depth_range: 0.0..1.0,
    };

    // The render pass we created above only describes the layout of our framebuffers. Before we
    // can draw we also need to create the actual framebuffers.
    //
    // Since we need to draw to multiple images, we are going to create a different framebuffer for
    // each image.
    let mut framebuffers = window_size_dependent_setup(&images, render_pass.clone(), &mut viewport);

    // Initialization is finally finished!

    // In some situations, the swapchain will become invalid by itself. This includes for example
    // when the window is resized (as the images of the swapchain will no longer match the
    // window's) or, on Android, when the application went to the background and goes back to the
    // foreground.
    //
    // In this situation, acquiring a swapchain image or presenting it will return an error.
    // Rendering to an image of that swapchain will not produce any error, but may or may not work.
    // To continue rendering, we need to recreate the swapchain by creating a new swapchain.
    // Here, we remember that we need to do this for the next loop iteration.
    let mut recreate_swapchain = false;

    // In the loop below we are going to submit commands to the GPU. Submitting a command produces
    // an object that implements the `GpuFuture` trait, which holds the resources for as long as
    // they are in use by the GPU.
    //
    // Destroying the `GpuFuture` blocks until the GPU is finished executing it. In order to avoid
    // that, we store the submission of the previous frame here.
    let mut previous_frame_end = Some(sync::now(device.clone()).boxed());


    let mut last_change = SystemTime::now();
    let mut last_image_added = SystemTime::now();
    let vertices :Arc<RwLock<Vec<Vertex>>> = Arc::new(RwLock::new(Vec::new()));


    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {

                running.store(false, std::sync::atomic::Ordering::Relaxed);
                while let Some(cur_thread) = threads_vec.pop() {
                    cur_thread.join().unwrap();
                }
                *control_flow = ControlFlow::Exit;
            }
            Event::WindowEvent {
                event: WindowEvent::Resized(_),
                ..
            } => {
                recreate_swapchain = true;
            }
            Event::RedrawEventsCleared => {


                // Do not draw frame when screen dimensions are zero.
                // On Windows, this can occur from minimizing the application.
                let dimensions = surface.window().inner_size();
                if dimensions.width == 0 || dimensions.height == 0 {
                    return;
                }


                // It is important to call this function from time to time, otherwise resources will keep
                // accumulating and you will eventually reach an out of memory error.
                // Calling this function polls various fences in order to determine what the GPU has
                // already processed, and frees the resources that are no longer needed.
                previous_frame_end.as_mut().unwrap().cleanup_finished();

                // Whenever the window resizes we need to recreate everything dependent on the window size.
                // In this example that includes the swapchain, the framebuffers and the dynamic state viewport.
                if recreate_swapchain {
                    // Use the new dimensions of the window.

                    let (new_swapchain, new_images) =
                        match swapchain.recreate(SwapchainCreateInfo {
                            image_extent: dimensions.into(),
                            ..swapchain.create_info()
                        }) {
                            Ok(r) => r,
                            // This error tends to happen when the user is manually resizing the window.
                            // Simply restarting the loop is the easiest way to fix this issue.
                            Err(SwapchainCreationError::ImageExtentNotSupported { .. }) => return,
                            Err(e) => panic!("Failed to recreate swapchain: {:?}", e),
                        };

                    swapchain = new_swapchain;
                    // Because framebuffers contains an Arc on the old swapchain, we need to
                    // recreate framebuffers as well.
                    framebuffers = window_size_dependent_setup(
                        &new_images,
                        render_pass.clone(),
                        &mut viewport,
                    );
                    recreate_swapchain = false;
                }

                

                //safe the current state in the vertex_buffer for drawing
                let vertex_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, vertices.read().unwrap().clone())
                .unwrap();

                // Before we can draw on the output, we have to *acquire* an image from the swapchain. If
                // no image is available (which happens if you submit draw commands too quickly), then the
                // function will block.
                // This operation returns the index of the image that we are allowed to draw upon.
                //
                // This function can block if no image is available. The parameter is an optional timeout
                // after which the function call will return an error.
                let (image_num, suboptimal, acquire_future) =
                    match acquire_next_image(swapchain.clone(), None) {
                        Ok(r) => r,
                        Err(AcquireError::OutOfDate) => {
                            recreate_swapchain = true;
                            return;
                        }
                        Err(e) => panic!("Failed to acquire next image: {:?}", e),
                    };

                // acquire_next_image can be successful, but suboptimal. This means that the swapchain image
                // will still work, but it may not display correctly. With some drivers this can be when
                // the window resizes, but it may not cause the swapchain to become out of date.
                if suboptimal {
                    recreate_swapchain = true;
                }

                // In order to draw, we have to build a *command buffer*. The command buffer object holds
                // the list of commands that are going to be executed.
                //
                // Building a command buffer is an expensive operation (usually a few hundred
                // microseconds), but it is known to be a hot path in the driver and is expected to be
                // optimized.
                //
                // Note that we have to pass a queue family when we create the command buffer. The command
                // buffer will only be executable on that given queue family.
                let mut builder = AutoCommandBufferBuilder::primary(
                    device.clone(),
                    queue.family(),
                    CommandBufferUsage::MultipleSubmit,      //oneTimeSubmit is more optimized and applicable, since we create a new one every frame
                )
                .unwrap();

                builder
                    // Before we can draw, we have to *enter a render pass*.
                    .begin_render_pass(
                        RenderPassBeginInfo {
                            // A list of values to clear the attachments with. This list contains
                            // one item for each attachment in the render pass. In this case,
                            // there is only one attachment, and we clear it with a blue color.
                            //
                            // Only attachments that have `LoadOp::Clear` are provided with clear
                            // values, any others should use `ClearValue::None` as the clear value.
                            clear_values: vec![Some([0.4, 0.4, 0.4, 1.0].into())],
                            ..RenderPassBeginInfo::framebuffer(framebuffers[image_num].clone())
                        },
                        // The contents of the first (and only) subpass. This can be either
                        // `Inline` or `SecondaryCommandBuffers`. The latter is a bit more advanced
                        // and is not covered here.
                        SubpassContents::Inline,
                    )
                    .unwrap()
                    // We are now inside the first subpass of the render pass. We add a draw command.
                    //
                    // The last two parameters contain the list of resources to pass to the shaders.
                    // Since we used an `EmptyPipeline` object, the objects have to be `()`.
                    .set_viewport(0, [viewport.clone()])
                    .bind_pipeline_graphics(pipeline.clone())
                    .bind_descriptor_sets(
                        PipelineBindPoint::Graphics,
                        pipeline.layout().clone(),
                        0,
                        descriptor_set.clone(),
                    )
                    .bind_vertex_buffers(0, vertex_buffer.clone())
                    .draw(vertex_buffer.len() as u32, 1, 0, 0)
                    .unwrap()
                    // We leave the render pass by calling `draw_end`. Note that if we had multiple
                    // subpasses we could have called `next_inline` (or `next_secondary`) to jump to the
                    // next subpass.
                    .end_render_pass()
                    .unwrap();

                // Finish building the command buffer by calling `build`.
                let command_buffer = builder.build().unwrap();

                let future = previous_frame_end
                    .take()
                    .unwrap()
                    .join(acquire_future)
                    .then_execute(queue.clone(), command_buffer)
                    .unwrap()
                    // The color output is now expected to contain our triangle. But in order to show it on
                    // the screen, we have to *present* the image by calling `present`.
                    //
                    // This function does not actually present the image immediately. Instead it submits a
                    // present command at the end of the queue. This means that it will only be presented once
                    // the GPU has finished executing the command buffer that draws the triangle.
                    .then_swapchain_present(queue.clone(), swapchain.clone(), image_num)
                    .then_signal_fence_and_flush();

                match future {
                    Ok(future) => {
                        previous_frame_end = Some(future.boxed());
                    }
                    Err(FlushError::OutOfDate) => {
                        recreate_swapchain = true;
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
                    }
                    Err(e) => {
                        println!("Failed to flush future: {:?}",    e);
                        previous_frame_end = Some(sync::now(device.clone()).boxed());
                    }
                }
            }
            //this Event gets submitted 60 times per second, due to vulkan restricting rendering to once per ~16.4-16.5 ms.
            //this can be called way more if running on a different thread and thus (and because of hardware limitations aswell),
            //it is needed, to update game-logic with delta-time
            Event::MainEventsCleared => {
                
                
                    let now_time = SystemTime::now();
                    let time_diff = now_time.duration_since(last_change);
                    last_change = SystemTime::now();

                    println!("{:?}", time_diff);
                    for _ in 0..20000 {      //THIS IS JUST SOME JUNK TO SIMULATE SOME ACTUAL LOGIC TO GET THE CORRECT VERTICES
                        let k = 0;
                        let m = k+2;
                        let s = String::from(m.to_string());
                        assert!(m == s.parse::<i32>().unwrap());
                    }


                    let mut  vertices_lock = vertices.write().unwrap();
                    if vertices_lock.len() < 100{
                        *vertices_lock = Vec::new();
                        let max = 5000;
                        for i in 0..max{
                            let close_f = -0.1;
                            let far_f = -0.9;
                            let mut i2 = i as f32;
                            while i2 > max as f32 /10.0 {
                                i2 -=max as f32 /10.0;
                            }
                            vertices_lock.push(Vertex { position: [ i2 as f32 / ((max/10) as f32) + close_f, ((i/(max /10)) as f32 / 10.0) +far_f], tex_i: 0, coords: [1.0, 0.0] });
                            vertices_lock.push(Vertex { position: [ i2 as f32 / ((max/10) as f32) + far_f, ((i/(max /10)) as f32 / 10.0)  + far_f], tex_i: 0, coords: [0.0, 0.0] });
                            vertices_lock.push(Vertex { position: [ i2 as f32 / ((max/10) as f32) + far_f, ((i/(max /10)) as f32 / 10.0)  + close_f], tex_i: 0, coords: [0.0, 1.0] });
                            vertices_lock.push(Vertex { position: [ i2 as f32 / ((max/10) as f32) + close_f, ((i/(max /10)) as f32 / 10.0) + far_f], tex_i: 0, coords: [1.0, 0.0] });
                            vertices_lock.push(Vertex { position: [ i2 as f32 / ((max/10) as f32) + far_f, ((i/(max /10)) as f32 / 10.0)  + close_f], tex_i: 0, coords: [0.0, 1.0] });
                            vertices_lock.push(Vertex { position: [ i2 as f32 / ((max/10) as f32) + close_f, ((i/(max /10)) as f32 / 10.0)  + close_f], tex_i: 0, coords: [1.0, 1.0] });
    
                        }
                    }

                

                    let mut rng = rand::thread_rng();
                    //This puts even more strain on the current thread, later this should be handled by the model thread
                    if  time_diff.unwrap().as_millis() > 15 {
                        last_change = SystemTime::now();
                        
                        vertices_lock.iter_mut().for_each(|v| {v.position[1]+= 0.0005 });
                        vertices_lock.iter_mut().filter(|v| v.tex_i==1).for_each(|v| v.position[1]+=0.0005);
    
                    }
                    
                    if (now_time.duration_since(last_image_added)).unwrap().as_millis() > 7000 {
                        last_image_added = SystemTime::now();
                        let index = rng.gen::<u32>() % 2;
                        let sign = rng.gen_range(0.0..1.0);
    
                        let close_f = -0.1;
                        let far_f = -0.9;
                        vertices_lock.push(Vertex { position: [sign+close_f, far_f], tex_i: index, coords: [1.0, 0.0] });
                        vertices_lock.push(Vertex { position: [ sign+far_f,  far_f], tex_i: index, coords: [0.0, 0.0] });
                        vertices_lock.push(Vertex { position: [ sign+far_f, close_f], tex_i: index, coords: [0.0, 1.0] });
                        vertices_lock.push(Vertex { position: [sign+close_f,  far_f], tex_i: index, coords: [1.0, 0.0] });
                        vertices_lock.push(Vertex { position: [ sign+far_f, close_f], tex_i: index, coords: [0.0, 1.0] });
                        vertices_lock.push(Vertex { position: [sign+close_f, close_f], tex_i: index, coords: [1.0, 1.0] });
    
                    }
               
                    




            }
            _ => (),
        }
    });
}

/// This method is called once during initialization, then again whenever the window is resized
#[inline(always)]
fn window_size_dependent_setup(
    images: &[Arc<SwapchainImage<Window>>],
    render_pass: Arc<RenderPass>,
    viewport: &mut Viewport,
) -> Vec<Arc<Framebuffer>> {
    let dimensions = images[0].dimensions().width_height();
    viewport.dimensions = [dimensions[0] as f32, dimensions[1] as f32];

    images
        .iter()
        .map(|image| {
            let view = ImageView::new_default(image.clone()).unwrap();
            Framebuffer::new(
                render_pass.clone(),
                FramebufferCreateInfo {
                    attachments: vec![view],
                    ..Default::default()
                },
            )
            .unwrap()
        })
        .collect::<Vec<_>>()
}




    // The raw shader creation API provided by the vulkano library is unsafe, for various reasons.
    //
    // An overview of what the `shader!` macro generates can be found in the
    // `vulkano-shaders` crate docs. You can view them at https://docs.rs/vulkano-shaders/
    //
    // TODO: explain this in details

    //the vec4(position, 0.0, 1.0) puts the vertex at the specified index, while zooming out by a factor of 1. Changing the 1.0 will zoom inwards or outwards
    pub(crate) mod vs {
        vulkano_shaders::shader! {
            ty: "vertex",
            src: "
            #version 450
            
            layout(location = 0) in vec2 position;
            layout(location = 1) in uint tex_i;
            layout(location = 2) in vec2 coords;
            
            layout(location = 0) out flat uint out_tex_i;
            layout(location = 1) out vec2 out_coords;
            
            void main() {
                gl_Position = vec4(position, 0.0, 1.0);
                out_tex_i = tex_i;
                out_coords = coords;
            }"
        }
    }

    pub(crate) mod fs {
        vulkano_shaders::shader! {
            ty: "fragment",
            src: "
            #version 450
            
            #extension GL_EXT_nonuniform_qualifier : enable
            
            layout(location = 0) in flat uint tex_i;
            layout(location = 1) in vec2 coords;
            
            layout(location = 0) out vec4 f_color;
            
            layout(set = 0, binding = 0) uniform sampler2D tex[];
            
            void main() {
                f_color = texture(tex[tex_i], coords);
            }"
        }
    }





