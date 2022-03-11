use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        AxisId, ButtonId, DeviceId, ElementState, Event, KeyboardInput, ModifiersState,
        MouseButton, MouseScrollDelta, StartCause, Touch, TouchPhase,
    },
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Theme, Window, WindowBuilder},
};

use std::{marker::PhantomData, path::PathBuf};

pub struct WinKeyboardInput {
    device_id: DeviceId,
    input: KeyboardInput,
}

pub struct WinCursorMoved {
    device_id: DeviceId,
    position: PhysicalPosition<f64>,
    modifiers: ModifiersState,
}

pub struct WinCursorEntered {
    device_id: DeviceId,
}

pub struct WinCursorLeft {
    device_id: DeviceId,
}

pub struct WinMouseWheel {
    device_id: DeviceId,
    delta: MouseScrollDelta,
    phase: TouchPhase,
    modifiers: ModifiersState,
}

pub struct WinMouseInput {
    device_id: DeviceId,
    state: ElementState,
    button: MouseButton,
    modifiers: ModifiersState,
}

pub struct WinTouchpadPressure {
    device_id: DeviceId,
    pressure: f32,
    stage: i64,
}

pub struct WinAxisMotion {
    device_id: DeviceId,
    axis: AxisId,
    value: f64,
}

pub struct HandlerRes<T> {
    control_flow: ControlFlow,
    param: T,
}

type Handler<A, T, S> =
    Option<Box<dyn FnMut(A, &EventLoopWindowTarget<T>) -> HandlerRes<S>>>;
type HandlerNoArg<T, S> =
    Option<Box<dyn FnMut(&EventLoopWindowTarget<T>) -> HandlerRes<S>>>;

pub struct WinLoop<T>
where
    T: 'static,
{
    window: Window,
    on_new_events: Handler<StartCause, T, ()>,
    window_event_handlers: WindowEventHandlers<T>,
    device_event_handlers: DeviceEventHandlers<T>,
    on_user_event: Handler<T, T, ()>,
    on_suspended: HandlerNoArg<T, ()>,
    on_resumed: HandlerNoArg<T, ()>,
    on_main_events_cleared: HandlerNoArg<T, ()>,
    on_redraw_requested: HandlerNoArg<T, ()>,
    on_redraw_events_cleared: HandlerNoArg<T, ()>,
    on_loop_destroyed: HandlerNoArg<T, ()>,

    _t: PhantomData<T>,
}

#[derive(Default)]
struct WindowEventHandlers<T>
where
    T: 'static,
{
    on_resized: Handler<PhysicalSize<u32>, T, ()>,
    on_moved: Handler<PhysicalPosition<i32>, T, ()>,
    on_close_requested: HandlerNoArg<T, ()>,
    on_destroyed: HandlerNoArg<T, ()>,
    on_dropped_file: Handler<PathBuf, T, ()>,
    on_hovered_file: Handler<PathBuf, T, ()>,
    on_hovered_file_cancelled: Handler<PathBuf, T, ()>,
    on_received_character: Handler<char, T, ()>,
    on_focused: Handler<bool, T, ()>,
    on_keyboard_input: Handler<WinKeyboardInput, T, ()>,
    on_modifiers_changed: Handler<ModifiersState, T, ()>,
    on_cursor_moved: Handler<WinCursorMoved, T, ()>,
    on_cursor_entered: Handler<WinCursorEntered, T, ()>,
    on_cursor_left: Handler<WinCursorLeft, T, ()>,
    on_mouse_wheel: Handler<WinMouseWheel, T, ()>,
    on_mouse_input: Handler<WinMouseInput, T, ()>,
    on_touchpad_pressure: Handler<WinTouchpadPressure, T, ()>,
    on_axis_motion: Handler<WinAxisMotion, T, ()>,
    on_touch: Handler<Touch, T, ()>,
    on_scale_factor_changed: Handler<f64, T, PhysicalSize<u32>>,
    on_theme_changed: Handler<Theme, T, ()>,
}

pub struct DeviceMouseMotion {
    delta: (f64, f64),
}

pub struct DeviceMouseWheel {
    delta: MouseScrollDelta,
}

pub struct DeviceMotion {
    axis: AxisId,
    value: f64,
}

pub struct DeviceButton {
    button: ButtonId,
    state: ElementState,
}

#[derive(Default)]
struct DeviceEventHandlers<T>
where
    T: 'static,
{
    on_added: HandlerNoArg<T, ()>,
    on_removed: HandlerNoArg<T, ()>,
    on_mouse_motion: Handler<DeviceMouseMotion, T, ()>,
    on_mouse_wheel: Handler<DeviceMouseWheel, T, ()>,
    on_motion: Handler<DeviceMotion, T, ()>,
    on_button: Handler<DeviceButton, T, ()>,
    on_key: Handler<KeyboardInput, T, ()>,
    on_text: Handler<char, T, ()>,
}

impl<T> WinLoop<T>
where
    T: 'static,
{
    pub fn new() -> Self
    where
        T: Default,
    {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new().build(&event_loop).unwrap();

        WinLoop {
            window,
            on_new_events: None,
            window_event_handlers: WindowEventHandlers::default(),
            device_event_handlers: DeviceEventHandlers::default(),
            on_user_event: None,
            on_suspended: None,
            on_resumed: None,
            on_main_events_cleared: None,
            on_redraw_requested: None,
            on_redraw_events_cleared: None,
            on_loop_destroyed: None,

            _t: PhantomData,
        }
    }

    pub fn on_new_events(&mut self, handler: Handler<StartCause, T, ()>) -> &mut Self {
        self.on_new_events = handler;
        self
    }

    pub fn on_user_event(&mut self, handler: Handler<T, T, ()>) -> &mut Self {
        self.on_user_event = handler;
        self
    }

    pub fn on_suspended(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.on_suspended = handler;
        self
    }

    pub fn on_resumed(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.on_resumed = handler;
        self
    }

    pub fn on_main_events_cleared(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.on_main_events_cleared = handler;
        self
    }

    pub fn on_redraw_requested(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.on_redraw_requested = handler;
        self
    }

    pub fn on_redraw_events_cleared(
        &mut self,
        handler: HandlerNoArg<T, ()>,
    ) -> &mut Self {
        self.on_redraw_events_cleared = handler;
        self
    }

    pub fn on_loop_destroyed(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.on_loop_destroyed = handler;
        self
    }

    pub fn on_win_resized(
        &mut self,
        handler: Handler<PhysicalSize<u32>, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_resized = handler;
        self
    }

    pub fn on_win_moved(
        &mut self,
        handler: Handler<PhysicalPosition<i32>, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_moved = handler;
        self
    }

    pub fn on_win_close_requested(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.window_event_handlers.on_close_requested = handler;
        self
    }

    pub fn on_win_destroyed(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.window_event_handlers.on_destroyed = handler;
        self
    }

    pub fn on_win_dropped_file(&mut self, handler: Handler<PathBuf, T, ()>) -> &mut Self {
        self.window_event_handlers.on_dropped_file = handler;
        self
    }

    pub fn on_win_hovered_file(&mut self, handler: Handler<PathBuf, T, ()>) -> &mut Self {
        self.window_event_handlers.on_hovered_file = handler;
        self
    }

    pub fn on_win_hovered_file_cancelled(
        &mut self,
        handler: Handler<PathBuf, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_hovered_file_cancelled = handler;
        self
    }

    pub fn on_win_received_character(
        &mut self,
        handler: Handler<char, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_received_character = handler;
        self
    }

    pub fn on_win_focused(&mut self, handler: Handler<bool, T, ()>) -> &mut Self {
        self.window_event_handlers.on_focused = handler;
        self
    }

    pub fn on_win_keyboard_input(
        &mut self,
        handler: Handler<WinKeyboardInput, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_keyboard_input = handler;
        self
    }

    pub fn on_win_modifiers_changed(
        &mut self,
        handler: Handler<ModifiersState, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_modifiers_changed = handler;
        self
    }

    pub fn on_win_cursor_moved(
        &mut self,
        handler: Handler<WinCursorMoved, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_cursor_moved = handler;
        self
    }

    pub fn on_win_cursor_entered(
        &mut self,
        handler: Handler<WinCursorEntered, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_cursor_entered = handler;
        self
    }

    pub fn on_win_cursor_left(
        &mut self,
        handler: Handler<WinCursorLeft, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_cursor_left = handler;
        self
    }

    pub fn on_win_mouse_wheel(
        &mut self,
        handler: Handler<WinMouseWheel, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_mouse_wheel = handler;
        self
    }

    pub fn on_win_mouse_input(
        &mut self,
        handler: Handler<WinMouseInput, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_mouse_input = handler;
        self
    }

    pub fn on_win_touchpad_pressure(
        &mut self,
        handler: Handler<WinTouchpadPressure, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_touchpad_pressure = handler;
        self
    }

    pub fn on_win_axis_motion(
        &mut self,
        handler: Handler<WinAxisMotion, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.on_axis_motion = handler;
        self
    }

    pub fn on_win_touch(&mut self, handler: Handler<Touch, T, ()>) -> &mut Self {
        self.window_event_handlers.on_touch = handler;
        self
    }

    pub fn on_win_scale_factor_changed(
        &mut self,
        handler: Handler<f64, T, PhysicalSize<u32>>,
    ) -> &mut Self {
        self.window_event_handlers.on_scale_factor_changed = handler;
        self
    }

    pub fn on_win_theme_changed(&mut self, handler: Handler<Theme, T, ()>) -> &mut Self {
        self.window_event_handlers.on_theme_changed = handler;
        self
    }

    pub fn on_device_added(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.device_event_handlers.on_added = handler;
        self
    }

    pub fn on_device_removed(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.device_event_handlers.on_removed = handler;
        self
    }

    pub fn on_device_mouse_motion(
        &mut self,
        handler: Handler<DeviceMouseMotion, T, ()>,
    ) -> &mut Self {
        self.device_event_handlers.on_mouse_motion = handler;
        self
    }

    pub fn on_device_mouse_wheel(
        &mut self,
        handler: Handler<DeviceMouseWheel, T, ()>,
    ) -> &mut Self {
        self.device_event_handlers.on_mouse_wheel = handler;
        self
    }

    pub fn on_device_motion(
        &mut self,
        handler: Handler<DeviceMotion, T, ()>,
    ) -> &mut Self {
        self.device_event_handlers.on_motion = handler;
        self
    }

    pub fn on_device_button(
        &mut self,
        handler: Handler<DeviceButton, T, ()>,
    ) -> &mut Self {
        self.device_event_handlers.on_button = handler;
        self
    }

    pub fn on_device_key(&mut self, handler: Handler<KeyboardInput, T, ()>) -> &mut Self {
        self.device_event_handlers.on_key = handler;
        self
    }

    pub fn on_device_text(&mut self, handler: Handler<char, T, ()>) -> &mut Self {
        self.device_event_handlers.on_text = handler;
        self
    }
}
