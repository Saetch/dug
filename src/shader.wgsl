// Vertex shader


//store output, we declare a struct, builtin position means that these are coordinates
struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
};


//mark this as valid entryPoint for vertexShader
[[stage(vertex)]]
fn vs_main(  [[builtin(vertex_index)]] in_vertex_index: u32) -> VertexOutput {

    var out: VertexOutput;                                      //var means mutable and needs its type specified, let means immutable and does not change
    let x = f32(1 - i32(in_vertex_index)) * 0.5;                //these will just resolve to the correct X and Y coordinates when fed with index 0, 1, 2
    let y = f32(i32(in_vertex_index & 1u) * 2 - 1) * 0.5;
    out.clip_position = vec4<f32>(x, y, 0.0, 1.0);              //we specified the VertexOutput struct above and its clip_position field
    return out;

}


// Fragment shader

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.clip_position[0], in.clip_position[1], 0.1, 1.0);
}

 

 
 

 