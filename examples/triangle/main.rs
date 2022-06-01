use structopt::StructOpt;
use winit::{event::Event, event_loop::ControlFlow, window::Window};

use gpgpu::{niw, util, Config, Render, Screen};

mod render;

const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(short = "bg")]
    bg: Option<String>,
}

struct State {
    bg: wgpu::Color,
    render: Render,
}

impl AsMut<Render> for State {
    fn as_mut(&mut self) -> &mut Render {
        &mut self.render
    }
}

const VERTICES: &[render::Vertex] = &[
    render::Vertex { position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0] },
    render::Vertex {
        position: [-0.5, -0.5, 0.0],
        color: [0.0, 1.0, 0.0],
    },
    render::Vertex { position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0] },
];

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let name = "example-triangle".to_string();
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
            bg: util::html_to_color(&opts.bg.clone().unwrap_or("#123456".to_string()))
                .unwrap(),
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
    let vertex_buffer =
        render::Vertex::to_buffer(&state.render.as_screen().device, VERTICES);
    let pipeline = render::render_pipeline(&state.render.as_screen().device, FORMAT);

    let target = state.render.to_color_target();

    let mut encoder = {
        let desc = wgpu::CommandEncoderDescriptor { label: Some("clear_screen") };
        state.render.as_screen().device.create_command_encoder(&desc)
    };
    {
        let mut render_pass = {
            let ops = wgpu::Operations {
                load: wgpu::LoadOp::Clear(state.bg.clone()),
                store: true,
            };
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
        render_pass.draw(0..3, 0..1);
    }

    state.render.submit(encoder).unwrap();

    None
}
