// Vertex shader

struct VertexInput {
    @location(0) position: vec3<f32>,
    @location(1) tex_i: u32,
    @location(2) tex_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_i: u32,
    @location(1) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = model.tex_coords;
    out.clip_position = vec4<f32>(model.position, 1.0);
    out.tex_i = model.tex_i;
    return out;
}

// Fragment shader

@group(0) @binding(0)
var t_diffuse: binding_array<texture_2d<f32>>;
@group(0) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let image = t_diffuse[in.tex_i];
    return textureSample(image, s_diffuse, in.tex_coords);
}