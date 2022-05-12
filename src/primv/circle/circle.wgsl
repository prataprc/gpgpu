struct Transforms {
    model: mat4x4<f32>;
    mvp: mat4x4<f32>;
};

struct Attributes {
    fg: vec4<f32>;
    bg: vec4<f32>;
    center: vec2<f32>;
    radius: f32;
    width: f32;
    fill: u32;
};


struct VertexInput {
    [[location(0)]] coord: vec4<f32>;
};

struct VertexOutput {
    [[builtin(position)]] clip_position: vec4<f32>;
};

[[binding(0), group(0)]] var<uniform> transforms: Transforms;
[[binding(1), group(0)]] var<uniform> attrs: Attributes;

[[stage(vertex)]]
fn vs_main(in: VertexInput) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = in.coord;
    return out;
}

[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    let width = attrs.width / 2.0;
    let x: f32 = attrs.center.x - in.clip_position.x;
    let y: f32 = in.clip_position.y - attrs.center.y;
    let s: f32 = sqrt((x*x) + (y*y));
    let d = abs((attrs.radius - width) - s);

    if (attrs.fill == u32(1)) {
        if (d < width) {
            return attrs.fg;
        } else if (s < (attrs.radius - width)) {
            return attrs.fg;
        } else {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
    } else {
        if (d < width) {
            return attrs.fg;
        } else if (s < (attrs.radius - width)) {
            return attrs.bg;
        } else {
            return vec4<f32>(0.0, 0.0, 0.0, 0.0);
        }
    }
}
