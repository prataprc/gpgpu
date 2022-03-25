use log::{debug, trace, warn};
use winit::{
    event::{DeviceEvent, Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowAttributes, WindowBuilder},
};

use std::fmt;

#[allow(unused_imports)]
use crate::{niw, niw::WinitConfig};
use crate::{AppWindow, Error, Result};

/// Type to handle events with an event-argument and window-target
pub type Handler<E> = Option<Box<dyn FnMut(Event<E>) -> Option<ControlFlow>>>;

/// Type instantiates an event-loop and an associated window, useful for single window
/// applications.
///
/// Can be constructed from [WinitConfig] configuration, refer
/// [SingleWindow::from_config]. This type parameterised over user-event `E` for
/// [EventLoop]
pub struct SingleWindow<E = ()>
where
    E: 'static,
{
    event_loop: EventLoop<E>,
    window: Window,
    event_handlers: Option<EventHandlers<E>>,
    window_event_handlers: Option<WindowEventHandlers<E>>,
    device_event_handlers: Option<DeviceEventHandlers<E>>,
}

#[derive(Default)]
struct EventHandlers<E>
where
    E: 'static,
{
    on_new_events: Handler<E>,
    on_user_event: Handler<E>,
    on_suspended: Handler<E>,
    on_resumed: Handler<E>,
    on_main_events_cleared: Handler<E>,
    on_redraw_requested: Handler<E>,
    on_redraw_events_cleared: Handler<E>,
    on_loop_destroyed: Handler<E>,
}

#[derive(Default)]
struct WindowEventHandlers<E>
where
    E: 'static,
{
    on_resized: Handler<E>,
    on_moved: Handler<E>,
    on_close_requested: Handler<E>,
    on_destroyed: Handler<E>,
    on_dropped_file: Handler<E>,
    on_hovered_file: Handler<E>,
    on_hovered_file_cancelled: Handler<E>,
    on_received_character: Handler<E>,
    on_focused: Handler<E>,
    on_keyboard_input: Handler<E>,
    on_modifiers_changed: Handler<E>,
    on_cursor_moved: Handler<E>,
    on_cursor_entered: Handler<E>,
    on_cursor_left: Handler<E>,
    on_mouse_wheel: Handler<E>,
    on_mouse_input: Handler<E>,
    on_touchpad_pressure: Handler<E>,
    on_axis_motion: Handler<E>,
    on_touch: Handler<E>,
    on_scale_factor_changed: Handler<E>,
    on_theme_changed: Handler<E>,
}

#[derive(Default)]
struct DeviceEventHandlers<E>
where
    E: 'static,
{
    on_added: Handler<E>,
    on_removed: Handler<E>,
    on_mouse_motion: Handler<E>,
    on_mouse_wheel: Handler<E>,
    on_motion: Handler<E>,
    on_button: Handler<E>,
    on_key: Handler<E>,
    on_text: Handler<E>,
}

impl<E> AppWindow<winit::window::Window> for SingleWindow<E> {
    fn as_window(&self) -> &winit::window::Window {
        &self.window
    }
}

macro_rules! handle_event {
    ($event:ident, $($handler:tt)*) => {{
        match $($handler)* {
            Some(handler) => handler($event),
            None => None,
        }
    }};
}

impl<E> SingleWindow<E>
where
    E: 'static,
{
    pub fn from_config(attrs: WindowAttributes) -> Result<Self>
    where
        E: Default,
    {
        let event_loop = EventLoop::<E>::with_user_event();

        let window = {
            let mut wb = WindowBuilder::new();
            wb.window = attrs;
            err_at!(Fatal, wb.build(&event_loop))?
        };

        let val = SingleWindow {
            event_loop,
            window,
            event_handlers: Some(EventHandlers::default()),
            window_event_handlers: Some(WindowEventHandlers::default()),
            device_event_handlers: Some(DeviceEventHandlers::default()),
        };

        Ok(val)
    }

    pub fn as_event_loop(&self) -> &EventLoop<E> {
        &self.event_loop
    }

    pub fn as_window(&self) -> &Window {
        &self.window
    }

    pub fn run(mut self) -> !
    where
        E: fmt::Debug + Clone,
    {
        let wid = self.window.id();
        let mut event_handlers = self.event_handlers.take().unwrap();
        let mut window_event_handlers = self.window_event_handlers.take().unwrap();
        let mut device_event_handlers = self.device_event_handlers.take().unwrap();

        self.event_loop.run(
            move |evnt: Event<E>, _: &EventLoopWindowTarget<E>, cf: &mut ControlFlow| {
                log_event(&evnt);

                let control_flow = match &evnt {
                    Event::NewEvents(_) => {
                        handle_event!(evnt, &mut event_handlers.on_new_events)
                    }
                    Event::UserEvent(_) => {
                        handle_event!(evnt, &mut event_handlers.on_user_event)
                    }
                    Event::Suspended => {
                        handle_event!(evnt, &mut event_handlers.on_suspended)
                    }
                    Event::Resumed => {
                        handle_event!(evnt, &mut event_handlers.on_resumed)
                    }
                    Event::MainEventsCleared => {
                        handle_event!(evnt, &mut event_handlers.on_main_events_cleared)
                    }
                    Event::RedrawRequested(window_id) if window_id == &wid => {
                        handle_event!(evnt, &mut event_handlers.on_redraw_requested)
                    }
                    Event::RedrawRequested(window_id) => {
                        warn!(
                            "mismatch in window id {:?} != {:?} for {:?}",
                            window_id, wid, evnt
                        );
                        None
                    }
                    Event::RedrawEventsCleared => {
                        handle_event!(evnt, &mut event_handlers.on_redraw_events_cleared)
                    }
                    Event::LoopDestroyed => {
                        handle_event!(evnt, &mut event_handlers.on_loop_destroyed)
                    }
                    Event::WindowEvent { window_id, event } if window_id == &wid => {
                        match event {
                            WindowEvent::Resized(_) => {
                                handle_event!(evnt, &mut window_event_handlers.on_resized)
                            }
                            WindowEvent::Moved(_) => {
                                handle_event!(evnt, &mut window_event_handlers.on_moved)
                            }
                            WindowEvent::CloseRequested => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_close_requested
                            ),
                            WindowEvent::Destroyed => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_destroyed
                            ),
                            WindowEvent::DroppedFile(_) => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_dropped_file
                            ),
                            WindowEvent::HoveredFile(_) => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_hovered_file
                            ),
                            WindowEvent::HoveredFileCancelled => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_hovered_file_cancelled
                            ),
                            WindowEvent::ReceivedCharacter(_) => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_received_character
                            ),
                            WindowEvent::Focused(_) => {
                                handle_event!(evnt, &mut window_event_handlers.on_focused)
                            }
                            WindowEvent::KeyboardInput { .. } => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_keyboard_input
                            ),
                            WindowEvent::ModifiersChanged(_) => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_modifiers_changed
                            ),
                            WindowEvent::CursorMoved { .. } => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_cursor_moved
                            ),
                            WindowEvent::CursorEntered { .. } => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_cursor_entered
                            ),
                            WindowEvent::CursorLeft { .. } => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_cursor_left
                            ),
                            WindowEvent::MouseWheel { .. } => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_mouse_wheel
                            ),
                            WindowEvent::MouseInput { .. } => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_mouse_input
                            ),
                            WindowEvent::TouchpadPressure { .. } => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_touchpad_pressure
                            ),
                            WindowEvent::AxisMotion { .. } => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_axis_motion
                            ),
                            WindowEvent::Touch(_) => {
                                handle_event!(evnt, &mut window_event_handlers.on_touch)
                            }
                            WindowEvent::ScaleFactorChanged { .. } => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_scale_factor_changed
                            ),
                            WindowEvent::ThemeChanged(_) => handle_event!(
                                evnt,
                                &mut window_event_handlers.on_theme_changed
                            ),
                        }
                    }
                    Event::WindowEvent { window_id, .. } => {
                        warn!(
                            "mismatch in window id {:?} != {:?} for {:?}",
                            window_id, wid, evnt
                        );
                        None
                    }
                    Event::DeviceEvent { event, .. } => match event {
                        DeviceEvent::Added => {
                            handle_event!(evnt, &mut device_event_handlers.on_added)
                        }
                        DeviceEvent::Removed => {
                            handle_event!(evnt, &mut device_event_handlers.on_removed)
                        }
                        DeviceEvent::MouseMotion { .. } => {
                            handle_event!(
                                evnt,
                                &mut device_event_handlers.on_mouse_motion
                            )
                        }
                        DeviceEvent::MouseWheel { .. } => {
                            handle_event!(evnt, &mut device_event_handlers.on_mouse_wheel)
                        }
                        DeviceEvent::Motion { .. } => {
                            handle_event!(evnt, &mut device_event_handlers.on_motion)
                        }
                        DeviceEvent::Button { .. } => {
                            handle_event!(evnt, &mut device_event_handlers.on_button)
                        }
                        DeviceEvent::Key(_) => {
                            handle_event!(evnt, &mut device_event_handlers.on_key)
                        }
                        DeviceEvent::Text { .. } => {
                            handle_event!(evnt, &mut device_event_handlers.on_text)
                        }
                    },
                };

                match control_flow {
                    Some(val) => *cf = val,
                    None => (),
                }
            },
        );
    }
}

impl<E> SingleWindow<E>
where
    E: 'static,
{
    pub fn on_new_events(&mut self, handler: Handler<E>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_new_events = handler;
        self
    }

    pub fn on_user_event(&mut self, handler: Handler<E>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_user_event = handler;
        self
    }

    pub fn on_suspended(&mut self, handler: Handler<E>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_suspended = handler;
        self
    }

    pub fn on_resumed(&mut self, handler: Handler<E>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_resumed = handler;
        self
    }

    pub fn on_main_events_cleared(&mut self, handler: Handler<E>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_main_events_cleared = handler;
        self
    }

    pub fn on_redraw_requested(&mut self, handler: Handler<E>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_redraw_requested = handler;
        self
    }

    pub fn on_redraw_events_cleared(&mut self, handler: Handler<E>) -> &mut Self {
        self.event_handlers
            .as_mut()
            .unwrap()
            .on_redraw_events_cleared = handler;
        self
    }

    pub fn on_loop_destroyed(&mut self, handler: Handler<E>) -> &mut Self {
        self.event_handlers.as_mut().unwrap().on_loop_destroyed = handler;
        self
    }

    pub fn on_win_resized(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_resized = handler;
        self
    }

    pub fn on_win_moved(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_moved = handler;
        self
    }

    pub fn on_win_close_requested(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_close_requested = handler;
        self
    }

    pub fn on_win_destroyed(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_destroyed = handler;
        self
    }

    pub fn on_win_dropped_file(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_dropped_file = handler;
        self
    }

    pub fn on_win_hovered_file(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_hovered_file = handler;
        self
    }

    pub fn on_win_hovered_file_cancelled(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_hovered_file_cancelled = handler;
        self
    }

    pub fn on_win_received_character(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_received_character = handler;
        self
    }

    pub fn on_win_focused(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_focused = handler;
        self
    }

    pub fn on_win_keyboard_input(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_keyboard_input = handler;
        self
    }

    pub fn on_win_modifiers_changed(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_modifiers_changed = handler;
        self
    }

    pub fn on_win_cursor_moved(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_cursor_moved = handler;
        self
    }

    pub fn on_win_cursor_entered(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_cursor_entered = handler;
        self
    }

    pub fn on_win_cursor_left(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_cursor_left = handler;
        self
    }

    pub fn on_win_mouse_wheel(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_mouse_wheel = handler;
        self
    }

    pub fn on_win_mouse_input(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_mouse_input = handler;
        self
    }

    pub fn on_win_touchpad_pressure(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_touchpad_pressure = handler;
        self
    }

    pub fn on_win_axis_motion(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_axis_motion = handler;
        self
    }

    pub fn on_win_touch(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers.as_mut().unwrap().on_touch = handler;
        self
    }

    pub fn on_win_scale_factor_changed(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_scale_factor_changed = handler;
        self
    }

    pub fn on_win_theme_changed(&mut self, handler: Handler<E>) -> &mut Self {
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_theme_changed = handler;
        self
    }

    pub fn on_device_added(&mut self, handler: Handler<E>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_added = handler;
        self
    }

    pub fn on_device_removed(&mut self, handler: Handler<E>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_removed = handler;
        self
    }

    pub fn on_device_mouse_motion(&mut self, handler: Handler<E>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_mouse_motion = handler;
        self
    }

    pub fn on_device_mouse_wheel(&mut self, handler: Handler<E>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_mouse_wheel = handler;
        self
    }

    pub fn on_device_motion(&mut self, handler: Handler<E>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_motion = handler;
        self
    }

    pub fn on_device_button(&mut self, handler: Handler<E>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_button = handler;
        self
    }

    pub fn on_device_key(&mut self, handler: Handler<E>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_key = handler;
        self
    }

    pub fn on_device_text(&mut self, handler: Handler<E>) -> &mut Self {
        self.device_event_handlers.as_mut().unwrap().on_text = handler;
        self
    }
}

fn log_event<E>(event: &Event<E>)
where
    E: fmt::Debug,
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
