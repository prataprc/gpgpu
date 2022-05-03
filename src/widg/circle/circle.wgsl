// Vertex shader

struct Transforms {
    model: mat4x4<f32>;
    mvp: mat4x4<f32>;
};

struct Style {
    fg: vec4<f32>;
    bg: vec4<f32>;
};

struct Params {
    center: vec2<f32>;
    radius: f32;
    fill: u32;
};


struct VertexInput {
    [[location(0)]] coord: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
};

[[binding(0), group(0)]] var<uniform> transforms: Transforms;
[[binding(1), group(0)]] var<uniform> style: Style;
[[binding(2), group(0)]] var<uniform> params: Params;

[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = in.coord;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let x: f32 = params.center.x - in.clip_position.x;
    let y: f32 = in.clip_position.y - params.center.y;
    let s: f32 = sqrt((x*x) + (y*y));

    if (params.fill == u32(1)) {
        if (s == params.radius) {
            return style.fg;
        } else if (ceil(s) == params.radius) {
            var fg = style.fg * (1.0 - (params.radius - s));
            fg.w = 1.0;
            return fg;
        } else if (floor(s) == params.radius) {
            var fg = style.fg * (1.0 - (s - params.radius));
            fg.w = 1.0;
            return fg;
        } else if (s < params.radius) {
            return style.fg;
        } else {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
    } else {
        if (s == params.radius) {
            return style.fg;
        } else if (ceil(s) == params.radius) {
            var fg = style.fg * (1.0 - (params.radius - s));
            fg.w = 1.0;
            return fg;
        } else if (floor(s) == params.radius) {
            var fg = style.fg * (1.0 - (s - params.radius));
            fg.w = 1.0;
            return fg;
        } else if (s < params.radius) {
            return style.bg;
        } else {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
    }
}
