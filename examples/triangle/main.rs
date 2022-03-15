use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoopWindowTarget},
};

fn main() {
    env_logger::init();

    let mut wloop = gpgpu::win::WinLoop::<()>::new();

    wloop
        .on_win_close_requested(Some(Box::new(on_win_close_requested)))
        .on_win_keyboard_input(Some(Box::new(on_win_keyboard_input)));

    wloop.run();
}

fn on_win_close_requested(
    _target: &EventLoopWindowTarget<()>,
) -> gpgpu::win::HandlerRes<()> {
    gpgpu::win::HandlerRes {
        control_flow: Some(ControlFlow::Exit),
        param: (),
    }
}

fn on_win_keyboard_input(
    input: gpgpu::win::WinKeyboardInput,
    _target: &EventLoopWindowTarget<()>,
) -> gpgpu::win::HandlerRes<()> {
    let control_flow = match input {
        gpgpu::win::WinKeyboardInput {
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

    gpgpu::win::HandlerRes {
        control_flow,
        param: (),
    }
}
