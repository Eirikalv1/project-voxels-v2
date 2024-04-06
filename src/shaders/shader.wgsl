struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) color: vec2<f32>,    
}

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec2<f32>
}

@vertex
fn vs_main(model: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(model.position, 0.0, 1.0);
    out.color = model.color;
    return out;
}

@group(0) @binding(0) var<uniform> frame_data: vec2<f32>;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let coord = vec3<f32>(in.color, 0.0) * 2.0 - 1.0; // -1 -> 1


    return vec4<f32>(coord.x, coord.y, 0.0, 1.0);
} 