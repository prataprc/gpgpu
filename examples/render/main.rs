use winit::{
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::ControlFlow,
};

use gpgpu::{niw, wg};

type Renderer = niw::Renderer<wg::Gpu, ()>;

fn main() {
    let name = "example-render".to_string();
    let config = wg::Config::default();

    env_logger::init();

    let mut swin = {
        let wattrs = config.to_window_attributes().unwrap();
        niw::SingleWindow::<wg::Gpu, (), ()>::from_config(wattrs).unwrap()
    };

    swin.on_win_close_requested(Box::new(on_win_close_requested))
        .on_win_keyboard_input(Box::new(on_win_keyboard_input))
        .on_win_resized(Box::new(on_win_resized));

    let r = {
        let gpu = pollster::block_on(wg::Gpu::new(
            name.clone(),
            swin.as_window(),
            wg::Config::default(),
        ))
        .unwrap();
        let state = ();
        Renderer { gpu, state }
    };

    println!("Press Esc to exit");
    swin.run(r);
}

fn on_win_resized(_r: &mut Renderer, _event: Event<()>) -> Option<ControlFlow> {
    None
}

fn on_win_close_requested(_r: &mut Renderer, _: Event<()>) -> Option<ControlFlow> {
    Some(ControlFlow::Exit)
}

fn on_win_keyboard_input(_r: &mut Renderer, event: Event<()>) -> Option<ControlFlow> {
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
            } => Some(ControlFlow::Exit),
            _ => None,
        },
        _ => None,
    }
}
