use log::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use gpgpu::{
    dom::{self, Domesticate},
    err_at, fonts, niw, util, Config, Context, Error, Render, Result, Screen, Transforms,
};

use crate::Opt;

const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Rgba8UnormSrgb;

struct State {
    font: fonts::FontFile,
    render: Render,
    frames: util::FrameRate,
    domr: dom::Dom,
}

impl AsRef<Render> for State {
    fn as_ref(&self) -> &Render {
        &self.render
    }
}

impl AsMut<Render> for State {
    fn as_mut(&mut self) -> &mut Render {
        &mut self.render
    }
}

impl State {
    fn redraw(&mut self) {
        if !self.frames.is_redraw() {
            return;
        }

        let screen = self.render.as_screen();

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("font-app:command-encoder"),
            };
            screen.device.create_command_encoder(&desc)
        };
        let context = Context {
            transforms: &Transforms::empty(),
            device: &screen.device,
            queue: &screen.queue,
        };
        let target = self.render.to_color_target();

        //self.render.submit(encoder).unwrap();

        self.frames.next_frame_after(10_000 /*micros*/);
    }
}

pub fn handle_raster(opts: Opt) -> Result<()> {
    let loc = opts.loc.clone().unwrap();

    let name = "font-app".to_string();
    let mut config = gpgpu::Config::default();
    {
        let mont = niw::get_monitor_info()?;
        let size = {
            let size = mont.to_logical_size();
            vec![(size.width as f64) * 0.8, (size.height as f64) * 0.9]
        };
        info!("{}", mont);
        info!("window size: {}x{}", size[0], size[1]);
        config.winit.inner_size = Some(size);
    };

    let mut swin = {
        let wattrs = config.to_window_attributes().unwrap();
        niw::SingleWindow::<State, ()>::from_config(wattrs).unwrap()
    };

    let font = fonts::FontFile::new(loc)?;

    let state = {
        let screen = pollster::block_on(Screen::new(
            name.clone(),
            swin.as_window(),
            Config::default(),
        ))
        .unwrap();

        let mut render = Render::new(screen, FORMAT);

        let domr = make_dom(&opts, &render, FORMAT)?;

        render.start();
        State {
            font,
            render,
            frames: util::FrameRate::new(),
            domr,
        }
    };

    swin.on_win_scale_factor_changed(Box::new(on_win_scale_factor_changed))
        .on_win_resized(Box::new(on_win_resized))
        .on_redraw_requested(Box::new(on_redraw_requested));

    info!("Press Esc to exit");
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

fn on_win_resized(
    _: &Window,
    state: &mut State,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    if let Event::WindowEvent { event, .. } = event {
        if let WindowEvent::Resized(size) = event {
            state.domr.resize((*size).into(), None);

            let extent = state.render.to_extent3d();
            info!("win_resize: extent:{}", extent);

            state.domr.compute_layout(extent).unwrap();
        }
    }

    None
}
fn on_win_scale_factor_changed(
    _: &Window,
    state: &mut State,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    if let Event::WindowEvent { event, .. } = event {
        if let WindowEvent::ScaleFactorChanged { .. } = event {
            state.domr.resize(
                state.render.to_extent3d().into(),
                Some(state.render.to_scale_factor()),
            );

            let extent = state.render.to_extent3d();
            println!("win_scale_factor_changed, extent:{}", extent);

            state.domr.compute_layout(extent).unwrap();
        }
    }

    None
}

fn make_dom(
    opts: &Opt,
    render: &Render,
    format: wgpu::TextureFormat,
) -> Result<dom::Dom> {
    use crate::SubCommand;

    let code_point = match opts.subcmd.clone() {
        SubCommand::Raster { code_point } => code_point,
        _ => unreachable!(),
    };

    let font = fonts::FontFile::new(opts.loc.as_ref().unwrap())?;
    let index = font.to_glyphs()?;
    let g = index
        .get(&code_point)
        .ok_or(err_at!(Invalid, error: "code_point {}", code_point))?
        .clone();

    let children = vec![dom::Node::from(dom::glyph::GlyphBox::new(g))];
    let mut win = dom::win::Win::new(children);
    win.resize(render.to_extent3d().into(), Some(render.to_scale_factor()));

    Ok(dom::Dom::new(win))
}
