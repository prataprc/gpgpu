use structopt::StructOpt;
use winit::{event::Event, event_loop::ControlFlow, window::Window};

use gpgpu::{
    niw, util, widg::clear, Config, Context, Render, Screen, Transforms, Widget,
};

const SSAA: f32 = 2.0;

#[derive(Clone, StructOpt)]
pub struct Opt {
    #[structopt(short = "c")]
    color: Option<String>,
}

struct State {
    color: wgpu::Color,
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
        let target = self.render.to_color_target();

        let clear = clear::Clear::new(self.color);
        clear.render(&context, &mut encoder, &target).unwrap();

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

    let mut render = Render::new_super_sampled(screen, SSAA);

    let state = {
        render.start();
        State {
            color: util::html_to_color(
                &opts.color.clone().unwrap_or("#FFFFFF".to_string()),
            )
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
    state.redraw();
    None
}
