use log::{debug, trace, warn};
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        AxisId, ButtonId, DeviceEvent, DeviceId, ElementState, Event, KeyboardInput,
        ModifiersState, MouseButton, MouseScrollDelta, StartCause, Touch, TouchPhase,
        WindowEvent,
    },
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Theme, Window, WindowAttributes, WindowBuilder},
};

use std::{fmt, marker::PhantomData, path::PathBuf};

#[allow(unused_imports)]
use crate::{niw, niw::WinitConfig};
use crate::{Error, Result};

/// Result returned by application handlers subscribed for [winit-events].
#[derive(Default)]
pub struct HandlerRes<S> {
    /// Refer to [ControlFlow] for details.
    pub control_flow: Option<ControlFlow>,
    /// Handler can return values parametrised over each event-handler.
    pub param: S,
}

/// Type to handle events with an event-argument and window-target
///
/// Event argument `A` must be one of the types defined by the [niw] package, refer to
/// package documentation for details.
pub type Handler<A, T, S> =
    Option<Box<dyn FnMut(A, &EventLoopWindowTarget<T>) -> HandlerRes<S>>>;

/// Type to handle events with no event-argument and only a window-target.
pub type HandlerNoArg<T, S> =
    Option<Box<dyn FnMut(&EventLoopWindowTarget<T>) -> HandlerRes<S>>>;

/// Type handles event-loop and window object, can be instantiated from [WinitConfig]
/// configuration.
pub struct Handle<T>
where
    T: 'static,
{
    event_loop: EventLoop<T>,
    window: Window,
    event_handlers: Option<EventHandlers<T>>,
    window_event_handlers: Option<WindowEventHandlers<T>>,
    device_event_handlers: Option<DeviceEventHandlers<T>>,

    _t: PhantomData<T>,
}

#[derive(Default)]
struct EventHandlers<T>
where
    T: 'static,
{
    on_new_events: Handler<StartCause, T, ()>,
    on_user_event: Handler<T, T, ()>,
    on_suspended: HandlerNoArg<T, ()>,
    on_resumed: HandlerNoArg<T, ()>,
    on_main_events_cleared: HandlerNoArg<T, ()>,
    on_redraw_requested: HandlerNoArg<T, ()>,
    on_redraw_events_cleared: HandlerNoArg<T, ()>,
    on_loop_destroyed: HandlerNoArg<T, ()>,
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
    on_hovered_file_cancelled: HandlerNoArg<T, ()>,
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

macro_rules! handle_event {
    (arg: $arg:ident, $target:ident, $($handler:tt)*) => {{
        match $($handler)* {
            Some(handler) => {
                let HandlerRes { control_flow, param } = handler($arg, $target);
                (control_flow, Some(param))
            },
            None => (None, None),
        }
    }};
    ($target:ident, $($handler:tt)*) => {{
        match $($handler)* {
            Some(handler) => {
                let HandlerRes { control_flow, param } = handler($target);
                (control_flow, Some(param))
            }
            None => (None, None)
        }
    }};
}

impl<T> Handle<T>
where
    T: 'static,
{
    pub fn from_config(attrs: WindowAttributes) -> Result<Self>
    where
        T: Default,
    {
        let event_loop = EventLoop::<T>::with_user_event();

        let window = {
            let mut wb = WindowBuilder::new();
            wb.window = attrs;
            err_at!(Fatal, wb.build(&event_loop))?
        };

        let val = Handle {
            event_loop,
            window,
            event_handlers: Some(EventHandlers::default()),
            window_event_handlers: Some(WindowEventHandlers::default()),
            device_event_handlers: Some(DeviceEventHandlers::default()),

            _t: PhantomData,
        };

        Ok(val)
    }

    pub fn as_event_loop(&self) -> &EventLoop<T> {
        &self.event_loop
    }

    pub fn as_window(&self) -> &Window {
        &self.window
    }

    pub fn run(mut self) -> !
    where
        T: fmt::Debug,
    {
        let wid = self.window.id();
        let mut event_handlers = self.event_handlers.take().unwrap();
        let mut window_event_handlers = self.window_event_handlers.take().unwrap();
        let mut device_event_handlers = self.device_event_handlers.take().unwrap();

        self.event_loop.run(
            move |event: Event<T>,
                  target: &EventLoopWindowTarget<T>,
                  control_flow: &mut ControlFlow| {
                log_event(&event);

                let (new_control_flow, _) = match event {
                    Event::NewEvents(start_clause) => handle_event!(
                        arg: start_clause,
                        target,
                        &mut event_handlers.on_new_events
                    ),
                    Event::UserEvent(ev) => {
                        handle_event!(arg: ev, target, &mut event_handlers.on_user_event)
                    }
                    Event::Suspended => {
                        handle_event!(target, &mut event_handlers.on_suspended)
                    }
                    Event::Resumed => {
                        handle_event!(target, &mut event_handlers.on_resumed)
                    }
                    Event::MainEventsCleared => {
                        handle_event!(target, &mut event_handlers.on_main_events_cleared)
                    }
                    Event::RedrawRequested(window_id) if window_id == wid => {
                        handle_event!(target, &mut event_handlers.on_redraw_requested)
                    }
                    Event::RedrawEventsCleared => handle_event!(
                        target,
                        &mut event_handlers.on_redraw_events_cleared
                    ),
                    Event::LoopDestroyed => {
                        handle_event!(target, &mut event_handlers.on_loop_destroyed)
                    }
                    Event::WindowEvent { window_id, event } if window_id == wid => {
                        match event {
                            WindowEvent::Resized(size) => handle_event!(
                                arg: size,
                                target,
                                &mut window_event_handlers.on_resized
                            ),
                            WindowEvent::Moved(pos) => handle_event!(
                                arg: pos,
                                target,
                                &mut window_event_handlers.on_moved
                            ),
                            WindowEvent::CloseRequested => handle_event!(
                                target,
                                &mut window_event_handlers.on_close_requested
                            ),
                            WindowEvent::Destroyed => handle_event!(
                                target,
                                &mut window_event_handlers.on_destroyed
                            ),
                            WindowEvent::DroppedFile(file_loc) => handle_event!(
                                arg: file_loc,
                                target,
                                &mut window_event_handlers.on_dropped_file
                            ),
                            WindowEvent::HoveredFile(file_loc) => handle_event!(
                                arg: file_loc,
                                target,
                                &mut window_event_handlers.on_hovered_file
                            ),
                            WindowEvent::HoveredFileCancelled => handle_event!(
                                target,
                                &mut window_event_handlers.on_hovered_file_cancelled
                            ),
                            WindowEvent::ReceivedCharacter(ch) => handle_event!(
                                arg: ch,
                                target,
                                &mut window_event_handlers.on_received_character
                            ),
                            WindowEvent::Focused(focused) => handle_event!(
                                arg: focused,
                                target,
                                &mut window_event_handlers.on_focused
                            ),
                            WindowEvent::KeyboardInput {
                                device_id,
                                input,
                                is_synthetic,
                            } => {
                                let arg = WinKeyboardInput {
                                    device_id,
                                    input,
                                    is_synthetic,
                                };
                                handle_event!(
                                    arg: arg,
                                    target,
                                    &mut window_event_handlers.on_keyboard_input
                                )
                            }
                            WindowEvent::ModifiersChanged(state) => handle_event!(
                                arg: state,
                                target,
                                &mut window_event_handlers.on_modifiers_changed
                            ),
                            WindowEvent::CursorMoved {
                                device_id,
                                position,
                                ..
                            } => {
                                let arg = WinCursorMoved {
                                    device_id,
                                    position,
                                };
                                handle_event!(
                                    arg: arg,
                                    target,
                                    &mut window_event_handlers.on_cursor_moved
                                )
                            }
                            WindowEvent::CursorEntered { device_id } => {
                                let arg = WinCursorEntered { device_id };
                                handle_event!(
                                    arg: arg,
                                    target,
                                    &mut window_event_handlers.on_cursor_entered
                                )
                            }
                            WindowEvent::CursorLeft { device_id } => {
                                let arg = WinCursorLeft { device_id };
                                handle_event!(
                                    arg: arg,
                                    target,
                                    &mut window_event_handlers.on_cursor_left
                                )
                            }
                            WindowEvent::MouseWheel {
                                device_id,
                                delta,
                                phase,
                                ..
                            } => {
                                let arg = WinMouseWheel {
                                    device_id,
                                    delta,
                                    phase,
                                };
                                handle_event!(
                                    arg: arg,
                                    target,
                                    &mut window_event_handlers.on_mouse_wheel
                                )
                            }
                            WindowEvent::MouseInput {
                                device_id,
                                state,
                                button,
                                ..
                            } => {
                                let arg = WinMouseInput {
                                    device_id,
                                    state,
                                    button,
                                };
                                handle_event!(
                                    arg: arg,
                                    target,
                                    &mut window_event_handlers.on_mouse_input
                                )
                            }
                            WindowEvent::TouchpadPressure {
                                device_id,
                                pressure,
                                stage,
                            } => {
                                let arg = WinTouchpadPressure {
                                    device_id,
                                    pressure,
                                    stage,
                                };
                                handle_event!(
                                    arg: arg,
                                    target,
                                    &mut window_event_handlers.on_touchpad_pressure
                                )
                            }
                            WindowEvent::AxisMotion {
                                device_id,
                                axis,
                                value,
                            } => {
                                let arg = WinAxisMotion {
                                    device_id,
                                    axis,
                                    value,
                                };
                                handle_event!(
                                    arg: arg,
                                    target,
                                    &mut window_event_handlers.on_axis_motion
                                )
                            }
                            WindowEvent::Touch(touch) => handle_event!(
                                arg: touch,
                                target,
                                &mut window_event_handlers.on_touch
                            ),
                            WindowEvent::ScaleFactorChanged {
                                scale_factor,
                                new_inner_size,
                            } => {
                                let (control_flow, size) = handle_event!(
                                    arg: scale_factor,
                                    target,
                                    &mut window_event_handlers.on_scale_factor_changed
                                );
                                size.map(|size| *new_inner_size = size);
                                (control_flow, None)
                            }
                            WindowEvent::ThemeChanged(theme) => handle_event!(
                                arg: theme,
                                target,
                                &mut window_event_handlers.on_theme_changed
                            ),
                        }
                    }
                    Event::WindowEvent { window_id, .. } => {
                        warn!("mismatch in window id {:?} != {:?}", window_id, wid);
                        (None, None)
                    }
                    Event::DeviceEvent { event, .. } => match event {
                        DeviceEvent::Added => {
                            handle_event!(target, &mut device_event_handlers.on_added)
                        }
                        DeviceEvent::Removed => {
                            handle_event!(target, &mut device_event_handlers.on_removed)
                        }
                        DeviceEvent::MouseMotion { delta } => {
                            let arg = DeviceMouseMotion { delta };
                            handle_event!(
                                arg: arg,
                                target,
                                &mut device_event_handlers.on_mouse_motion
                            )
                        }
                        DeviceEvent::MouseWheel { delta } => {
                            let arg = DeviceMouseWheel { delta };
                            handle_event!(
                                arg: arg,
                                target,
                                &mut device_event_handlers.on_mouse_wheel
                            )
                        }
                        DeviceEvent::Motion { axis, value } => {
                            let arg = DeviceMotion { axis, value };
                            handle_event!(
                                arg: arg,
                                target,
                                &mut device_event_handlers.on_motion
                            )
                        }
                        DeviceEvent::Button { button, state } => {
                            let arg = DeviceButton { button, state };
                            handle_event!(
                                arg: arg,
                                target,
                                &mut device_event_handlers.on_button
                            )
                        }
                        DeviceEvent::Key(input) => handle_event!(
                            arg: input,
                            target,
                            &mut device_event_handlers.on_key
                        ),
                        DeviceEvent::Text { codepoint } => handle_event!(
                            arg: codepoint,
                            target,
                            &mut device_event_handlers.on_text
                        ),
                    },
                    _ => unreachable!(),
                };

                if let Some(val) = new_control_flow {
                    *control_flow = val
                }
            },
        );
    }
}

impl<T> Handle<T>
where
    T: 'static,
{
    pub fn on_new_events(&mut self, handler: Handler<StartCause, T, ()>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_new_events = handler;
        self
    }

    pub fn on_user_event(&mut self, handler: Handler<T, T, ()>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_user_event = handler;
        self
    }

    pub fn on_suspended(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_suspended = handler;
        self
    }

    pub fn on_resumed(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_resumed = handler;
        self
    }

    pub fn on_main_events_cleared(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_main_events_cleared = handler;
        self
    }

    pub fn on_redraw_requested(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_redraw_requested = handler;
        self
    }

    pub fn on_redraw_events_cleared(
        &mut self,
        handler: HandlerNoArg<T, ()>,
    ) -> &mut Self {
        self.event_handlers
            .as_mut()
            .unwrap()
            .on_redraw_events_cleared = handler;
        self
    }

    pub fn on_loop_destroyed(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_loop_destroyed = handler;
        self
    }

    pub fn on_win_resized(
        &mut self,
        handler: Handler<PhysicalSize<u32>, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_resized = handler;
        self
    }

    pub fn on_win_moved(
        &mut self,
        handler: Handler<PhysicalPosition<i32>, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_moved = handler;
        self
    }

    pub fn on_win_close_requested(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_close_requested = handler;
        self
    }

    pub fn on_win_destroyed(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_destroyed = handler;
        self
    }

    pub fn on_win_dropped_file(&mut self, handler: Handler<PathBuf, T, ()>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_dropped_file = handler;
        self
    }

    pub fn on_win_hovered_file(&mut self, handler: Handler<PathBuf, T, ()>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_hovered_file = handler;
        self
    }

    pub fn on_win_hovered_file_cancelled(
        &mut self,
        handler: HandlerNoArg<T, ()>,
    ) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_hovered_file_cancelled = handler;
        self
    }

    pub fn on_win_received_character(
        &mut self,
        handler: Handler<char, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_received_character = handler;
        self
    }

    pub fn on_win_focused(&mut self, handler: Handler<bool, T, ()>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_focused = handler;
        self
    }

    pub fn on_win_keyboard_input(
        &mut self,
        handler: Handler<WinKeyboardInput, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_keyboard_input = handler;
        self
    }

    pub fn on_win_modifiers_changed(
        &mut self,
        handler: Handler<ModifiersState, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_modifiers_changed = handler;
        self
    }

    pub fn on_win_cursor_moved(
        &mut self,
        handler: Handler<WinCursorMoved, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_cursor_moved = handler;
        self
    }

    pub fn on_win_cursor_entered(
        &mut self,
        handler: Handler<WinCursorEntered, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_cursor_entered = handler;
        self
    }

    pub fn on_win_cursor_left(
        &mut self,
        handler: Handler<WinCursorLeft, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_cursor_left = handler;
        self
    }

    pub fn on_win_mouse_wheel(
        &mut self,
        handler: Handler<WinMouseWheel, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_mouse_wheel = handler;
        self
    }

    pub fn on_win_mouse_input(
        &mut self,
        handler: Handler<WinMouseInput, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_mouse_input = handler;
        self
    }

    pub fn on_win_touchpad_pressure(
        &mut self,
        handler: Handler<WinTouchpadPressure, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_touchpad_pressure = handler;
        self
    }

    pub fn on_win_axis_motion(
        &mut self,
        handler: Handler<WinAxisMotion, T, ()>,
    ) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_axis_motion = handler;
        self
    }

    pub fn on_win_touch(&mut self, handler: Handler<Touch, T, ()>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_touch = handler;
        self
    }

    pub fn on_win_scale_factor_changed(
        &mut self,
        handler: Handler<f64, T, PhysicalSize<u32>>,
    ) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_scale_factor_changed = handler;
        self
    }

    pub fn on_win_theme_changed(&mut self, handler: Handler<Theme, T, ()>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_theme_changed = handler;
        self
    }

    pub fn on_device_added(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_added = handler;
        self
    }

    pub fn on_device_removed(&mut self, handler: HandlerNoArg<T, ()>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_removed = handler;
        self
    }

    pub fn on_device_mouse_motion(
        &mut self,
        handler: Handler<DeviceMouseMotion, T, ()>,
    ) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_mouse_motion = handler;
        self
    }

    pub fn on_device_mouse_wheel(
        &mut self,
        handler: Handler<DeviceMouseWheel, T, ()>,
    ) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_mouse_wheel = handler;
        self
    }

    pub fn on_device_motion(
        &mut self,
        handler: Handler<DeviceMotion, T, ()>,
    ) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_motion = handler;
        self
    }

    pub fn on_device_button(
        &mut self,
        handler: Handler<DeviceButton, T, ()>,
    ) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_button = handler;
        self
    }

    pub fn on_device_key(&mut self, handler: Handler<KeyboardInput, T, ()>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_key = handler;
        self
    }

    pub fn on_device_text(&mut self, handler: Handler<char, T, ()>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_text = handler;
        self
    }
}

fn log_event<T>(event: &Event<T>)
where
    T: fmt::Debug,
{
    match event {
        Event::NewEvents(StartCause::Poll)
        | Event::RedrawEventsCleared
        | Event::MainEventsCleared => {
            trace!(target: "niw::event_loop", "event {:?}", event)
        }
        _ => debug!(target: "niw::event_loop", "event {:?}", event),
    }
}

pub struct WinKeyboardInput {
    pub device_id: DeviceId,
    pub input: KeyboardInput,
    pub is_synthetic: bool,
}

pub struct WinCursorMoved {
    pub device_id: DeviceId,
    pub position: PhysicalPosition<f64>,
}

pub struct WinCursorEntered {
    pub device_id: DeviceId,
}

pub struct WinCursorLeft {
    pub device_id: DeviceId,
}

pub struct WinMouseWheel {
    pub device_id: DeviceId,
    pub delta: MouseScrollDelta,
    pub phase: TouchPhase,
}

pub struct WinMouseInput {
    pub device_id: DeviceId,
    pub state: ElementState,
    pub button: MouseButton,
}

pub struct WinTouchpadPressure {
    pub device_id: DeviceId,
    pub pressure: f32,
    pub stage: i64,
}

pub struct WinAxisMotion {
    pub device_id: DeviceId,
    pub axis: AxisId,
    pub value: f64,
}

pub struct DeviceMouseMotion {
    pub delta: (f64, f64),
}

pub struct DeviceMouseWheel {
    pub delta: MouseScrollDelta,
}

pub struct DeviceMotion {
    pub axis: AxisId,
    pub value: f64,
}

pub struct DeviceButton {
    pub button: ButtonId,
    pub state: ElementState,
}
