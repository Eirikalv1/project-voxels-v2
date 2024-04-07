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
    let coord = vec2<f32>(in.color) * 2.0 - 1.0; // -1 -> 1

    let ray_origin = vec3<f32>(0.0, 0.0, -1.0);
    let ray_direction = vec3<f32>(coord.x, coord.y, -1.0);
    let sphere_radius = 0.5;

    let a = dot(ray_direction, ray_direction);
    let b = 2.0 * dot(ray_origin, ray_direction);
    let c = dot(ray_origin, ray_origin) - pow(sphere_radius, 2.0);

    let discriminant = b * b - 4.0 * a * c;
    if discriminant > 0.0 {
        return vec4<f32>(0.0, 0.4, 1.0, 1.0);
    }

    return vec4<f32>(0.0, 0.0, 0.0, 1.0);
} 