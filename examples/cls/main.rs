use structopt::StructOpt;
use winit::{event::Event, event_loop::ControlFlow, window::Window};

use gpgpu::{
    niw, primv::clear, util, Config, Context, Render, Screen, Transforms, Viewport,
};

const SSAA: f32 = 2.0;
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(short = "c")]
    color: Option<String>,

    #[structopt(long = "position", use_delimiter = true)]
    position: Option<Vec<f32>>,

    #[structopt(long = "size", use_delimiter = true)]
    size: Option<Vec<f32>>,
}

struct State {
    color: wgpu::Color,
    position: [f32; 2],
    size: [f32; 2],
    render: Render,
}

impl State {
    fn redraw(&mut self) {
        let screen = self.render.as_screen();

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("examples/cls:command-encoder"),
            };
            screen.device.create_command_encoder(&desc)
        };

        let context = Context {
            transforms: &Transforms::empty(),
            device: &screen.device,
            queue: &screen.queue,
        };
        let mut target = self.render.to_color_target();
        target.view_port = Viewport {
            x: self.position[0],
            y: self.position[1],
            w: self.size[0],
            h: self.size[1],
            ..Viewport::default()
        };

        let mut clear = clear::Clear::new(self.color);
        clear.redraw(&context, &mut encoder, &mut target).unwrap();

        self.render.submit(encoder).unwrap();
    }
}

impl AsMut<Render> for State {
    fn as_mut(&mut self) -> &mut Render {
        &mut self.render
    }
}

fn main() {
    env_logger::init();

    let opts = Opt::from_args();

    let name = "example-cls".to_string();
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

    let mut render = Render::new_super_sampled(screen, SSAA, FORMAT);

    let state = {
        let wgpu::Extent3d { width, height, .. } = render.to_extent3d();

        let position = match opts.position.as_ref().map(|v| v.as_slice()) {
            Some([x]) => [*x, 0.0],
            Some([x, y]) => [*x, *y],
            _ => [0.0, 0.0],
        };
        let size = match opts.size.as_ref().map(|v| v.as_slice()) {
            Some([w]) => [*w, height as f32],
            Some([w, h]) => [*w, *h],
            _ => [width as f32, height as f32],
        };
        let color = opts.color.clone().unwrap_or("#FFFFFF".to_string());

        render.start();
        State {
            color: util::html_to_color(&color).unwrap(),
            position,
            size,
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
    state.redraw();
    None
}
