use rand::random;
use structopt::StructOpt;
use winit::{event::Event, event_loop::ControlFlow, window::Window};

use gpgpu::{niw, util, Config, Render, Screen};

mod render;

const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(short = "fg")]
    fg: Option<String>,

    #[structopt(short = "n", default_value = "100")]
    n_points: u32,
}

struct State {
    fg: wgpu::Color,
    n_points: u32,
    render: Render,
}

impl AsMut<Render> for State {
    fn as_mut(&mut self) -> &mut Render {
        &mut self.render
    }
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let name = "example-points".to_string();
    let config = Config::default();

    let mut swin = {
        let wattrs = config.to_window_attributes().unwrap();
        niw::SingleWindow::<State, ()>::from_config(wattrs).unwrap()
    };

    let screen = pollster::block_on(Screen::new(
        name.clone(),
        swin.as_window(),
        Config::default(),
    ))
    .unwrap();

    let state = {
        let mut render = Render::new(screen, FORMAT);

        render.start();
        State {
            fg: util::html_to_color(&opts.fg.clone().unwrap_or("#123456".to_string()))
                .unwrap(),
            n_points: opts.n_points,
            render,
        }
    };

    swin.on_redraw_requested(Box::new(on_redraw_requested));

    println!("Press Esc to exit");
    swin.run(state);
}

fn on_redraw_requested(
    _: &Window,
    state: &mut State,
    _event: &mut Event<()>,
) -> Option<ControlFlow> {
    let vertices: Vec<render::Vertex> = (0..state.n_points)
        .map(|_| {
            let wgpu::Color { r, g, b, .. } = state.fg;
            let x = ((random::<i32>() as f64) / (i32::MAX as f64)) as f32;
            let y = ((random::<i32>() as f64) / (i32::MAX as f64)) as f32;
            // println!("{} {}", x, y);
            render::Vertex {
                position: [x, y, 0.0],
                color: [r as f32, g as f32, b as f32],
            }
        })
        .collect();
    let vertex_buffer =
        render::Vertex::to_buffer(&state.render.as_screen().device, vertices.as_slice());
    let pipeline = render::render_pipeline(&state.render.as_screen(), FORMAT);

    let target = state.render.to_color_target();

    let mut encoder = {
        let desc = wgpu::CommandEncoderDescriptor { label: Some("example-points") };
        state.render.as_screen().device.create_command_encoder(&desc)
    };
    {
        let mut render_pass = {
            let ops = wgpu::Operations { load: wgpu::LoadOp::Load, store: true };
            let desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    wgpu::RenderPassColorAttachment {
                        view: &target.view,
                        resolve_target: None,
                        ops,
                    },
                ],
                depth_stencil_attachment: None,
            };
            encoder.begin_render_pass(&desc)
        };
        render_pass.set_pipeline(&pipeline);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..state.n_points, 0..1);
    }

    state.render.submit(encoder).unwrap();

    None
}
