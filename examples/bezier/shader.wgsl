// Vertex shader

struct VertexInput {
    [[location(0)]] coord: vec3<f32>;
    [[location(1)]] color: vec3<f32>;
    [[location(2)]] texcoord: vec3<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
    [[location(1)]] texcoord: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main( in: VertexInput ) -> VertexOutput {
    var out: VertexOutput;
    out.color = in.color;
    out.texcoord = in.texcoord;
    out.clip_position = vec4<f32>(in.coord, 1.0);
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let val: f32 = (in.texcoord.x * in.texcoord.x) - in.texcoord.y;
    if (val < 0.0) {
        return vec4<f32>(in.color, 0.0);
    } else {
        return vec4<f32>(0.0, 0.0, 0.0, 0.0);
    }
}
