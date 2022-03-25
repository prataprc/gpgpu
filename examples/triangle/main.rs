use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoopWindowTarget},
    window::WindowAttributes,
};

use gpgpu::niw;

fn main() {
    env_logger::init();

    let mut h = {
        let wattrs = WindowAttributes::default();
        niw::SingleWindow::<()>::from_config(wattrs).unwrap()
    };

    h.on_win_close_requested(Some(Box::new(on_win_close_requested)))
        .on_win_keyboard_input(Some(Box::new(on_win_keyboard_input)));

    h.run();
}

fn on_win_close_requested(_target: &EventLoopWindowTarget<()>) -> niw::HandlerRes<()> {
    niw::HandlerRes {
        control_flow: Some(ControlFlow::Exit),
        param: (),
    }
}

fn on_win_keyboard_input(
    input: niw::WinKeyboardInput,
    _target: &EventLoopWindowTarget<()>,
) -> niw::HandlerRes<()> {
    let control_flow = match input {
        niw::WinKeyboardInput {
            input:
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Escape),
                    ..
                },
            ..
        } => Some(ControlFlow::Exit),
        _ => None,
    };

    niw::HandlerRes {
        control_flow,
        param: (),
    }
}
