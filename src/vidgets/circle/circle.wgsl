// Vertex shader

struct Transforms {
    model: mat4x4<f32>;
    mvp: mat4x4<f32>;
};

struct Parameters {
    bg: vec4<f32>;
    fg: vec4<f32>;
    fill: u32;
    radius: f32;
    center: vec2<f32>;
};

struct VertexInput {
    [[location(0)]] coord: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
};

[[binding(0), group(0)]] var<uniform> transforms: Transforms;
[[binding(1), group(0)]] var<uniform> params: Parameters;

[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = in.coord;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let x: f32 = in.clip_position.x - params.center.x;
    let y: f32 = params.center.y - in.clip_position.y;
    let s: f32 = sqrt((x*x) + (y*y));

    if (params.fill == u32(1)) {
        if (round(s) <= params.radius) {
            return params.fg;
        } else {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
    } else {
        if (ceil(s) == params.radius) {
            var fg = params.fg * (1.0 - (params.radius - s));
            fg.w = 1.0;
            return fg;
        } else if (floor(s) == params.radius) {
            var fg = params.fg * (1.0 - (s - params.radius));
            fg.w = 1.0;
            return fg;
        } else if (s < params.radius) {
            return params.bg;
        } else {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
    }
}
