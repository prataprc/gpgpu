use rand::random;
use structopt::StructOpt;
use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
    window::Window,
};

use std::sync::Arc;

use gpgpu::{niw, util, Config, Render, Screen};

mod render;

const SSAA: f32 = 2.0;

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
    color_texture: Arc<wgpu::Texture>,
    render: Render,
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

    swin.on_win_close_requested(Box::new(on_win_close_requested))
        .on_win_keyboard_input(Box::new(on_win_keyboard_input))
        .on_win_resized(Box::new(on_win_resized))
        .on_win_scale_factor_changed(Box::new(on_win_scale_factor_changed))
        .on_main_events_cleared(Box::new(on_main_events_cleared))
        .on_redraw_requested(Box::new(on_redraw_requested));

    let state = {
        let screen = pollster::block_on(Screen::new(
            name.clone(),
            swin.as_window(),
            Config::default(),
        ))
        .unwrap();

        let color_texture =
            Arc::new(screen.like_surface_texture(SSAA, Some(screen.to_texture_format())));

        let mut render = Render::new(screen);
        render.start();

        State {
            fg: util::html_to_color(&opts.fg.clone().unwrap_or("#123456".to_string()))
                .unwrap(),
            n_points: opts.n_points,
            render,
            color_texture,
        }
    };

    println!("Press Esc to exit");
    swin.run(state);
}

// RedrawRequested will only trigger once, unless we manually request it.
fn on_main_events_cleared(
    w: &Window,
    _state: &mut State,
    _event: &mut Event<()>,
) -> Option<ControlFlow> {
    w.request_redraw();
    None
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
    let pipeline = render::render_pipeline(&state.render.as_screen());

    let render_view = {
        let desc = wgpu::TextureViewDescriptor::default();
        state.color_texture.create_view(&desc)
    };

    let mut encoder = {
        let desc = wgpu::CommandEncoderDescriptor {
            label: Some("example-points"),
        };
        state
            .render
            .as_screen()
            .device
            .create_command_encoder(&desc)
    };
    {
        let mut render_pass = {
            let ops = wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: true,
            };
            let desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[
                    // This is what @location(0) in the fragment shader targets
                    wgpu::RenderPassColorAttachment {
                        view: &render_view,
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

    let cmd_buffers = vec![encoder.finish()];

    state
        .render
        .as_screen()
        .queue
        .submit(cmd_buffers.into_iter());

    state
        .render
        .post_frame(Arc::clone(&state.color_texture))
        .unwrap();

    None
}

fn on_win_resized(
    _: &Window,
    state: &mut State,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(size) => state.render.as_screen().resize(*size, None),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

fn on_win_scale_factor_changed(
    _: &Window,
    state: &mut State,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::ScaleFactorChanged {
                new_inner_size,
                scale_factor,
            } => {
                let screen = state.render.as_screen();
                screen.resize(**new_inner_size, Some(*scale_factor));
                state.color_texture = Arc::new(
                    screen.like_surface_texture(SSAA, Some(screen.to_texture_format())),
                );
            }
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }

    None
}

fn on_win_close_requested(
    _: &Window,
    state: &mut State,
    _: &mut Event<()>,
) -> Option<ControlFlow> {
    state.render.stop().ok();
    Some(ControlFlow::Exit)
}

fn on_win_keyboard_input(
    _: &Window,
    state: &mut State,
    event: &mut Event<()>,
) -> Option<ControlFlow> {
    match event {
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::KeyboardInput {
                input:
                    KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Escape),
                        ..
                    },
                ..
            } => {
                state.render.stop().ok();
                Some(ControlFlow::Exit)
            }
            _ => None,
        },
        _ => None,
    }
}
