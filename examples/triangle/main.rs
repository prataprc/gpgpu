use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoopWindowTarget},
};

fn main() {
    env_logger::init();

    let mut wloop = cgi::win::WinLoop::<()>::new();

    wloop
        .on_win_close_requested(Some(Box::new(on_win_close_requested)))
        .on_win_keyboard_input(Some(Box::new(on_win_keyboard_input)));

    wloop.run();
}

fn on_win_close_requested(
    _target: &EventLoopWindowTarget<()>,
) -> cgi::win::HandlerRes<()> {
    cgi::win::HandlerRes {
        control_flow: Some(ControlFlow::Exit),
        param: (),
    }
}

fn on_win_keyboard_input(
    input: cgi::win::WinKeyboardInput,
    _target: &EventLoopWindowTarget<()>,
) -> cgi::win::HandlerRes<()> {
    let control_flow = match input {
        cgi::win::WinKeyboardInput {
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

    cgi::win::HandlerRes {
        control_flow,
        param: (),
    }
}
