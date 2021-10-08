// Vertex shader

[[block]]
struct Uniform {
    rotation: f32;
};
[[group(0), binding(0)]]
var<uniform> uniform: Uniform;

struct VertexOutput {
    [[location(0)]] color: vec3<f32>;
    [[builtin(position)]] clip_pos: vec4<f32>;
};

[[stage(vertex)]]
fn main(
    [[builtin(vertex_index)]] in_vertex_index: u32,
) -> VertexOutput {
    var angle = uniform.rotation;
    var vertices = array<vec2<f32>, 3>(
        vec2<f32>(cos(angle), sin(angle)),
        vec2<f32>(cos(angle + 2.094395102), sin(angle + 2.094395102)),
        vec2<f32>(cos(angle + 4.188790205), sin(angle + 4.188790205)),
    );
    var colors = array<vec3<f32>, 3>(
        vec3<f32>(1.0, 0.0, 0.0),
        vec3<f32>(0.0, 1.0, 0.0),
        vec3<f32>(0.0, 0.0, 1.0)
    );
    var out: VertexOutput;
    out.clip_pos = vec4<f32>(vertices[in_vertex_index], 0.0, 1.0);
    out.color = colors[in_vertex_index];
    return out;
}

// Fragment shader

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}
