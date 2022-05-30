use std::{sync::Arc, io::Cursor};

use vulkano::{pipeline::{GraphicsPipeline, PipelineLayout, layout::PipelineLayoutCreateInfo, graphics::{vertex_input::BuffersDefinition, viewport::ViewportState, color_blend::ColorBlendState}, Pipeline}, descriptor_set::{PersistentDescriptorSet, layout::{DescriptorSetLayoutCreateInfo, DescriptorSetLayout, DescriptorSetLayoutCreationError}, WriteDescriptorSet}, image::{ImageDimensions, ImmutableImage, MipmapsCount, view::ImageView}, format::Format, sampler::{Sampler, SamplerCreateInfo, Filter, SamplerAddressMode}, render_pass::{Subpass, RenderPass}, device::{Queue, Device}, shader::ShaderModule};

use super::renderer::Vertex;

//SEE vulkano examples -> image-self-copy-blit for an example on how to use StorageImages, to store imageInfo and on how to use ImageCopies in render passes

pub(crate) fn load_sprites(device: Arc<Device>, queue: Arc<Queue>, render_pass: Arc<RenderPass>, vs: Arc<ShaderModule>, fs: Arc<ShaderModule>)-> (Arc<GraphicsPipeline>, Arc<PersistentDescriptorSet>){
    let dwarf_base_house_texture = {
        let png_bytes = include_bytes!("../../Dwarf_BaseHouse_px9.png").to_vec();
        let cursor = Cursor::new(png_bytes);
        let decoder = png::Decoder::new(cursor);
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let dimensions = ImageDimensions::Dim2d {
            width: info.width,
            height: info.height,
            array_layers: 1,
        };
        let mut image_data = Vec::new();
        image_data.resize((info.width * info.height * 4) as usize, 0);
        reader.next_frame(&mut image_data).unwrap();        //putting the start into the image_data vector, which is also an iterator
    
        let image = ImmutableImage::from_iter(
            image_data,
            dimensions,
            MipmapsCount::One,
            Format::R8G8B8A8_SRGB,
            queue.clone(),
        )
        .unwrap()
        .0;
    
        ImageView::new_default(image).unwrap()
    };
    
    let rust_logo_texture = {
        let png_bytes = include_bytes!("../../image_img.png").to_vec();
        let cursor = Cursor::new(png_bytes);
        let decoder = png::Decoder::new(cursor);
        let mut reader = decoder.read_info().unwrap();
        let info = reader.info();
        let dimensions = ImageDimensions::Dim2d {
            width: info.width,
            height: info.height,
            array_layers: 1,
        };
        let mut image_data = Vec::new();
        image_data.resize((info.width * info.height * 4) as usize, 0);
        reader.next_frame(&mut image_data).unwrap();
    
        let image = ImmutableImage::from_iter(
            image_data,
            dimensions,
            MipmapsCount::One,
            Format::R8G8B8A8_SRGB,
            queue.clone(),
        )
        .unwrap()
        .0;
    
        ImageView::new_default(image).unwrap()
    };
    
    //A sampler is an object that gives the color value of an texture for the given coordinates
    let sampler = Sampler::new(
        device.clone(),
        SamplerCreateInfo {
            mag_filter: Filter::Linear,
            min_filter: Filter::Linear,
            address_mode: [SamplerAddressMode::Repeat; 3],  //this will determine what happens when the sampler is supposed to get the color of a pixel outside the image. Repeat: act as if the image repeats endlessly, thus get the color of the next hypothetical image. 
            ..Default::default()
        },
    )
    .unwrap();
    
    
    //create a pipeline layout that can take the bindings for the texture samplers
    let pipeline_layout = {
        let mut layout_create_infos: Vec<_> = DescriptorSetLayoutCreateInfo::from_requirements(
            fs.entry_point("main").unwrap().descriptor_requirements(),
        );
    
        // Set 0, Binding 0
        let binding = layout_create_infos[0].bindings.get_mut(&0).unwrap();
        binding.variable_descriptor_count = true;
        binding.descriptor_count = 2;
    
        let set_layouts = layout_create_infos
            .into_iter()
            .map(|desc| Ok(DescriptorSetLayout::new(device.clone(), desc.clone())?))
            .collect::<Result<Vec<_>, DescriptorSetLayoutCreationError>>()
            .unwrap();
    
        PipelineLayout::new(
            device.clone(),
            PipelineLayoutCreateInfo {
                set_layouts,
                push_constant_ranges: fs
                    .entry_point("main")
                    .unwrap()
                    .push_constant_requirements()
                    .cloned()
                    .into_iter()
                    .collect(),
                ..Default::default()
            },
        )
        .unwrap()
    };
    
    
    //the subpass the pipeline will be used for. This needs to be specified in the pipeline creation and the pipeline will only be usable here
    let subpass = Subpass::from(render_pass.clone(), 0).unwrap();
    // Before we draw we have to create what is called a pipeline. This is similar to an OpenGL
    // program, but much more specific.
    let pipeline = GraphicsPipeline::start()
        // We need to indicate the layout of the vertices.
        .vertex_input_state(BuffersDefinition::new().vertex::<Vertex>())
        //.input_assembly_state(InputAssemblyState::new().topology(PrimitiveTopology::TriangleStrip))   //this will make it possible to define a rectangle sprite with just 4 vertices. BUT it will ultimately force multiple Vertices to all be connected. No good for a lot of vertices that represent distinct sprites. This could be used for smaller actions
        // A Vulkan shader can in theory contain multiple entry points, so we have to specify
        // which one.
        .vertex_shader(vs.entry_point("main").unwrap(), ())
        // Use a resizable viewport set to draw over the entire window
        .viewport_state(ViewportState::viewport_dynamic_scissor_irrelevant())
        // See `vertex_shader`.
        .fragment_shader(fs.entry_point("main").unwrap(), ())
        .color_blend_state(ColorBlendState::new(subpass.num_color_attachments()).blend_alpha())
        .render_pass(subpass)
        .with_pipeline_layout(device.clone(), pipeline_layout)
        .unwrap();
    
    let layout = pipeline.layout().set_layouts().get(0).unwrap();
    
    
    let descriptor_array = [
        (dwarf_base_house_texture.clone() as _, sampler.clone()),
        (rust_logo_texture.clone() as _, sampler.clone()),
    ];
    
    let descriptor_set = PersistentDescriptorSet::new_variable(
        layout.clone(),
        2,
        [WriteDescriptorSet::image_view_sampler_array(
            0,
            0,
            descriptor_array,
        )],
    )
    .unwrap();

    (pipeline, descriptor_set)
}