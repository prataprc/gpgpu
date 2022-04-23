// Vertex shader

struct Transforms {
    model: mat4x4<f32>;
    mvp: mat4x4<f32>;
};

struct VertexInput {
    [[location(0)]] coord: vec4<f32>;
    [[location(1)]] color: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec4<f32>;
};

[[binding(0), group(0)]] var<uniform> transforms: Transforms;

[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;

    out.color = in.color;
    out.clip_position = transforms.mvp * in.coord;

    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return in.color;
}
