use log::{debug, info, trace, warn};
use winit::{
    event::{DeviceEvent, Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowAttributes, WindowBuilder},
};

use std::fmt;

#[allow(unused_imports)]
use crate::ConfigWinit;
use crate::{Error, Render, Result};

/// Type to handle events with an event-argument `E`, window-target and state `S`.
pub type Handler<S, E> =
    Box<dyn FnMut(&Window, &mut S, &mut Event<E>) -> Option<ControlFlow>>;

/// Type instantiates an event-loop and an associated window, useful for single window
/// applications.
///
/// Can be constructed from [ConfigWinit] configuration, refer
/// [SingleWindow::from_config]. This type parameterised over user-event `E` for
/// [EventLoop]
pub struct SingleWindow<S, E = ()>
where
    E: 'static,
{
    event_loop: Option<EventLoop<E>>,
    window: Option<Window>,
    on_event: Option<Handler<S, E>>,
    event_handlers: Option<EventHandlers<S, E>>,
    window_event_handlers: Option<WindowEventHandlers<S, E>>,
    device_event_handlers: Option<DeviceEventHandlers<S, E>>,
    exit_on_esc: bool,
}

struct EventHandlers<S, E>
where
    E: 'static,
{
    on_new_events: Handler<S, E>,
    on_user_event: Handler<S, E>,
    on_suspended: Handler<S, E>,
    on_resumed: Handler<S, E>,
    on_main_events_cleared: Handler<S, E>,
    on_redraw_requested: Handler<S, E>,
    on_redraw_events_cleared: Handler<S, E>,
    on_loop_destroyed: Handler<S, E>,
}

impl<S, E> Default for EventHandlers<S, E> {
    fn default() -> Self {
        EventHandlers {
            on_new_events: Box::new(|_, _, _| None),
            on_user_event: Box::new(|_, _, _| None),
            on_suspended: Box::new(|_, _, _| None),
            on_resumed: Box::new(|_, _, _| None),
            on_main_events_cleared: Box::new(|_, _, _| None),
            on_redraw_requested: Box::new(|_, _, _| None),
            on_redraw_events_cleared: Box::new(|_, _, _| None),
            on_loop_destroyed: Box::new(|_, _, _| None),
        }
    }
}

struct WindowEventHandlers<S, E>
where
    E: 'static,
{
    on_resized: Handler<S, E>,
    on_moved: Handler<S, E>,
    on_close_requested: Handler<S, E>,
    on_destroyed: Handler<S, E>,
    on_dropped_file: Handler<S, E>,
    on_hovered_file: Handler<S, E>,
    on_hovered_file_cancelled: Handler<S, E>,
    on_received_character: Handler<S, E>,
    on_focused: Handler<S, E>,
    on_keyboard_input: Handler<S, E>,
    on_modifiers_changed: Handler<S, E>,
    on_cursor_moved: Handler<S, E>,
    on_cursor_entered: Handler<S, E>,
    on_cursor_left: Handler<S, E>,
    on_mouse_wheel: Handler<S, E>,
    on_mouse_input: Handler<S, E>,
    on_touchpad_pressure: Handler<S, E>,
    on_axis_motion: Handler<S, E>,
    on_touch: Handler<S, E>,
    on_scale_factor_changed: Handler<S, E>,
    on_theme_changed: Handler<S, E>,
}

impl<S, E> Default for WindowEventHandlers<S, E> {
    fn default() -> Self {
        WindowEventHandlers {
            on_resized: Box::new(|_, _, _| None),
            on_moved: Box::new(|_, _, _| None),
            on_close_requested: Box::new(|_, _, _| None),
            on_destroyed: Box::new(|_, _, _| None),
            on_dropped_file: Box::new(|_, _, _| None),
            on_hovered_file: Box::new(|_, _, _| None),
            on_hovered_file_cancelled: Box::new(|_, _, _| None),
            on_received_character: Box::new(|_, _, _| None),
            on_focused: Box::new(|_, _, _| None),
            on_keyboard_input: Box::new(|_, _, _| None),
            on_modifiers_changed: Box::new(|_, _, _| None),
            on_cursor_moved: Box::new(|_, _, _| None),
            on_cursor_entered: Box::new(|_, _, _| None),
            on_cursor_left: Box::new(|_, _, _| None),
            on_mouse_wheel: Box::new(|_, _, _| None),
            on_mouse_input: Box::new(|_, _, _| None),
            on_touchpad_pressure: Box::new(|_, _, _| None),
            on_axis_motion: Box::new(|_, _, _| None),
            on_touch: Box::new(|_, _, _| None),
            on_scale_factor_changed: Box::new(|_, _, _| None),
            on_theme_changed: Box::new(|_, _, _| None),
        }
    }
}

struct DeviceEventHandlers<S, E>
where
    E: 'static,
{
    on_added: Handler<S, E>,
    on_removed: Handler<S, E>,
    on_mouse_motion: Handler<S, E>,
    on_mouse_wheel: Handler<S, E>,
    on_motion: Handler<S, E>,
    on_button: Handler<S, E>,
    on_key: Handler<S, E>,
    on_text: Handler<S, E>,
}

impl<S, E> Default for DeviceEventHandlers<S, E> {
    fn default() -> Self {
        DeviceEventHandlers {
            on_added: Box::new(|_, _, _| None),
            on_removed: Box::new(|_, _, _| None),
            on_mouse_motion: Box::new(|_, _, _| None),
            on_mouse_wheel: Box::new(|_, _, _| None),
            on_motion: Box::new(|_, _, _| None),
            on_button: Box::new(|_, _, _| None),
            on_key: Box::new(|_, _, _| None),
            on_text: Box::new(|_, _, _| None),
        }
    }
}

impl<S, E> SingleWindow<S, E>
where
    E: 'static,
{
    pub fn from_config(attrs: WindowAttributes) -> Result<Self>
    where
        E: Default,
    {
        let event_loop = EventLoop::<E>::with_user_event();

        info!("inner_size {:?}", attrs.inner_size);

        let window = {
            let mut wb = WindowBuilder::new();
            wb.window = attrs;
            err_at!(Fatal, wb.build(&event_loop))?
        };

        let val = SingleWindow {
            event_loop: Some(event_loop),
            window: Some(window),
            on_event: Some(Box::new(|_, _, _| None)),
            event_handlers: Some(EventHandlers::default()),
            window_event_handlers: Some(WindowEventHandlers::default()),
            device_event_handlers: Some(DeviceEventHandlers::default()),
            exit_on_esc: true,
        };

        Ok(val)
    }

    pub fn set_exit_on_esc(&mut self, val: bool) -> &mut Self {
        self.exit_on_esc = val;
        self
    }

    pub fn as_event_loop(&self) -> &EventLoop<E> {
        self.event_loop.as_ref().unwrap()
    }

    pub fn as_window(&self) -> &Window {
        self.window.as_ref().unwrap()
    }

    pub fn to_scale_factor(&self) -> f32 {
        self.window.as_ref().map(|w| w.scale_factor() as f32).unwrap_or(0.0)
    }

    pub fn run(mut self, mut state: S) -> !
    where
        S: 'static + AsMut<Render>,
        E: fmt::Debug + Clone,
    {
        let window = self.window.take().unwrap();
        let event_loop = self.event_loop.take().unwrap();
        let wid = window.id();
        let mut on_event = self.on_event.take().unwrap();
        let mut event_handlers = self.event_handlers.take().unwrap();
        let mut window_event_handlers = self.window_event_handlers.take().unwrap();
        let mut device_event_handlers = self.device_event_handlers.take().unwrap();

        debug!("starting the event_loop ...");

        event_loop.run(
            move |mut evnt: Event<E>,
                  _: &EventLoopWindowTarget<E>,
                  cf: &mut ControlFlow| {
                log_event(&evnt);
                let mut no_op: Handler<S, E> = Box::new(|_, _, _| None);

                (on_event)(&window, &mut state, &mut evnt);

                let handler = match &evnt {
                    Event::NewEvents(_) => &mut event_handlers.on_new_events,
                    Event::UserEvent(_) => &mut event_handlers.on_user_event,
                    Event::Suspended => &mut event_handlers.on_suspended,
                    Event::Resumed => &mut event_handlers.on_resumed,
                    Event::MainEventsCleared => {
                        // Locally handle few things here.
                        window.request_redraw();
                        &mut event_handlers.on_main_events_cleared
                    }
                    Event::RedrawRequested(window_id) if window_id == &wid => {
                        &mut event_handlers.on_redraw_requested
                    }
                    Event::RedrawRequested(window_id) => {
                        warn!(
                            "mismatch in window id {:?} != {:?} for {:?}",
                            window_id, wid, evnt
                        );
                        &mut no_op
                    }
                    Event::RedrawEventsCleared => {
                        &mut event_handlers.on_redraw_events_cleared
                    }
                    Event::LoopDestroyed => &mut event_handlers.on_loop_destroyed,
                    Event::WindowEvent { window_id, event } if window_id == &wid => {
                        match event {
                            WindowEvent::Resized(size) => {
                                // Locally handle few things here.
                                let render: &mut Render = state.as_mut();
                                render.resize(*size, None);
                                &mut window_event_handlers.on_resized
                            }
                            WindowEvent::Moved(_) => &mut window_event_handlers.on_moved,
                            WindowEvent::CloseRequested => {
                                // Locally handle few things here.
                                let render: &mut Render = state.as_mut();
                                render.stop().ok();
                                *cf = ControlFlow::Exit;
                                &mut window_event_handlers.on_close_requested
                            }
                            WindowEvent::Destroyed => {
                                &mut window_event_handlers.on_destroyed
                            }
                            WindowEvent::DroppedFile(_) => {
                                &mut window_event_handlers.on_dropped_file
                            }
                            WindowEvent::HoveredFile(_) => {
                                &mut window_event_handlers.on_hovered_file
                            }
                            WindowEvent::HoveredFileCancelled => {
                                &mut window_event_handlers.on_hovered_file_cancelled
                            }
                            WindowEvent::ReceivedCharacter(_) => {
                                &mut window_event_handlers.on_received_character
                            }
                            WindowEvent::Focused(_) => {
                                &mut window_event_handlers.on_focused
                            }
                            WindowEvent::KeyboardInput { input, .. }
                                if self.exit_on_esc =>
                            {
                                use winit::event::{
                                    ElementState, KeyboardInput, VirtualKeyCode,
                                };

                                match input {
                                    KeyboardInput {
                                        state: ElementState::Pressed,
                                        virtual_keycode: Some(VirtualKeyCode::Escape),
                                        ..
                                    } => {
                                        // Locally handle few things here.
                                        let render: &mut Render = state.as_mut();
                                        render.stop().ok();
                                        *cf = ControlFlow::Exit;
                                        &mut window_event_handlers.on_keyboard_input
                                    }
                                    _ => &mut window_event_handlers.on_keyboard_input,
                                }
                            }
                            WindowEvent::KeyboardInput { .. } => {
                                &mut window_event_handlers.on_keyboard_input
                            }
                            WindowEvent::ModifiersChanged(_) => {
                                &mut window_event_handlers.on_modifiers_changed
                            }
                            WindowEvent::CursorMoved { .. } => {
                                &mut window_event_handlers.on_cursor_moved
                            }
                            WindowEvent::CursorEntered { .. } => {
                                &mut window_event_handlers.on_cursor_entered
                            }
                            WindowEvent::CursorLeft { .. } => {
                                &mut window_event_handlers.on_cursor_left
                            }
                            WindowEvent::MouseWheel { .. } => {
                                &mut window_event_handlers.on_mouse_wheel
                            }
                            WindowEvent::MouseInput { .. } => {
                                &mut window_event_handlers.on_mouse_input
                            }
                            WindowEvent::TouchpadPressure { .. } => {
                                &mut window_event_handlers.on_touchpad_pressure
                            }
                            WindowEvent::AxisMotion { .. } => {
                                &mut window_event_handlers.on_axis_motion
                            }
                            WindowEvent::Touch(_) => &mut window_event_handlers.on_touch,
                            WindowEvent::ScaleFactorChanged {
                                new_inner_size,
                                scale_factor,
                            } => {
                                // Locally handle few things here.
                                let render: &mut Render = state.as_mut();
                                render.resize(**new_inner_size, Some(*scale_factor));
                                &mut window_event_handlers.on_scale_factor_changed
                            }
                            WindowEvent::ThemeChanged(_) => {
                                &mut window_event_handlers.on_theme_changed
                            }
                        }
                    }
                    Event::WindowEvent { window_id, .. } => {
                        warn!(
                            "mismatch in window id {:?} != {:?} for {:?}",
                            window_id, wid, evnt
                        );
                        &mut no_op
                    }
                    Event::DeviceEvent { event, .. } => match event {
                        DeviceEvent::Added => &mut device_event_handlers.on_added,
                        DeviceEvent::Removed => &mut device_event_handlers.on_removed,
                        DeviceEvent::MouseMotion { .. } => {
                            &mut device_event_handlers.on_mouse_motion
                        }
                        DeviceEvent::MouseWheel { .. } => {
                            &mut device_event_handlers.on_mouse_wheel
                        }
                        DeviceEvent::Motion { .. } => {
                            &mut device_event_handlers.on_motion
                        }
                        DeviceEvent::Button { .. } => {
                            &mut device_event_handlers.on_button
                        }
                        DeviceEvent::Key(_) => &mut device_event_handlers.on_key,
                        DeviceEvent::Text { .. } => &mut device_event_handlers.on_text,
                    },
                };

                match handler(&window, &mut state, &mut evnt) {
                    Some(val) => *cf = val,
                    None => (),
                }
            },
        );
    }
}

impl<S, E> SingleWindow<S, E>
where
    E: 'static,
{
    pub fn on_event(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_event ...");
        self.on_event = Some(handler);
        self
    }

    pub fn on_new_events(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_new_events ...");
        self.event_handlers.as_mut().unwrap().on_new_events = handler;
        self
    }

    pub fn on_user_event(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_user_event ...");
        self.event_handlers.as_mut().unwrap().on_user_event = handler;
        self
    }

    pub fn on_suspended(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_suspended ...");
        self.event_handlers.as_mut().unwrap().on_suspended = handler;
        self
    }

    pub fn on_resumed(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_resumed ...");
        self.event_handlers.as_mut().unwrap().on_resumed = handler;
        self
    }

    pub fn on_main_events_cleared(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_main_events_cleared ...");
        self.event_handlers.as_mut().unwrap().on_main_events_cleared = handler;
        self
    }

    pub fn on_redraw_requested(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_redraw_requested ...");
        self.event_handlers.as_mut().unwrap().on_redraw_requested = handler;
        self
    }

    pub fn on_redraw_events_cleared(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_redraw_events_cleared ...");
        self.event_handlers.as_mut().unwrap().on_redraw_events_cleared = handler;
        self
    }

    pub fn on_loop_destroyed(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_loop_destroyed ...");
        self.event_handlers.as_mut().unwrap().on_loop_destroyed = handler;
        self
    }

    pub fn on_win_resized(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_resized ...");
        self.window_event_handlers.as_mut().unwrap().on_resized = handler;
        self
    }

    pub fn on_win_moved(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_moved ...");
        self.window_event_handlers.as_mut().unwrap().on_moved = handler;
        self
    }

    pub fn on_win_close_requested(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_close_requested ...");
        self.window_event_handlers.as_mut().unwrap().on_close_requested = handler;
        self
    }

    pub fn on_win_destroyed(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_destroyed ...");
        self.window_event_handlers.as_mut().unwrap().on_destroyed = handler;
        self
    }

    pub fn on_win_dropped_file(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_dropped_file ...");
        self.window_event_handlers.as_mut().unwrap().on_dropped_file = handler;
        self
    }

    pub fn on_win_hovered_file(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_hovered_file ...");
        self.window_event_handlers.as_mut().unwrap().on_hovered_file = handler;
        self
    }

    pub fn on_win_hovered_file_cancelled(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_hovered_file_cancelled ...");
        self.window_event_handlers.as_mut().unwrap().on_hovered_file_cancelled = handler;
        self
    }

    pub fn on_win_received_character(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_received_character ...");
        self.window_event_handlers.as_mut().unwrap().on_received_character = handler;
        self
    }

    pub fn on_win_focused(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_focused ...");
        self.window_event_handlers.as_mut().unwrap().on_focused = handler;
        self
    }

    pub fn on_win_keyboard_input(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_keyboard_input ...");
        self.window_event_handlers.as_mut().unwrap().on_keyboard_input = handler;
        self
    }

    pub fn on_win_modifiers_changed(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_modifiers_changed ...");
        self.window_event_handlers.as_mut().unwrap().on_modifiers_changed = handler;
        self
    }

    pub fn on_win_cursor_moved(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_cursor_moved ...");
        self.window_event_handlers.as_mut().unwrap().on_cursor_moved = handler;
        self
    }

    pub fn on_win_cursor_entered(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_cursor_entered ...");
        self.window_event_handlers.as_mut().unwrap().on_cursor_entered = handler;
        self
    }

    pub fn on_win_cursor_left(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_cursor_left ...");
        self.window_event_handlers.as_mut().unwrap().on_cursor_left = handler;
        self
    }

    pub fn on_win_mouse_wheel(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_mouse_wheel ...");
        self.window_event_handlers.as_mut().unwrap().on_mouse_wheel = handler;
        self
    }

    pub fn on_win_mouse_input(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_mouse_input ...");
        self.window_event_handlers.as_mut().unwrap().on_mouse_input = handler;
        self
    }

    pub fn on_win_touchpad_pressure(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_touchpad_pressure ...");
        self.window_event_handlers.as_mut().unwrap().on_touchpad_pressure = handler;
        self
    }

    pub fn on_win_axis_motion(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_axis_motion ...");
        self.window_event_handlers.as_mut().unwrap().on_axis_motion = handler;
        self
    }

    pub fn on_win_touch(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_touch ...");
        self.window_event_handlers.as_mut().unwrap().on_touch = handler;
        self
    }

    pub fn on_win_scale_factor_changed(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_scale_factor_changed ...");
        self.window_event_handlers.as_mut().unwrap().on_scale_factor_changed = handler;
        self
    }

    pub fn on_win_theme_changed(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_win_theme_changed ...");
        self.window_event_handlers.as_mut().unwrap().on_theme_changed = handler;
        self
    }

    pub fn on_device_added(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_device_added ...");
        self.device_event_handlers.as_mut().unwrap().on_added = handler;
        self
    }

    pub fn on_device_removed(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_device_removed ...");
        self.device_event_handlers.as_mut().unwrap().on_removed = handler;
        self
    }

    pub fn on_device_mouse_motion(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_device_motion ...");
        self.device_event_handlers.as_mut().unwrap().on_mouse_motion = handler;
        self
    }

    pub fn on_device_mouse_wheel(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_device_mouse_wheel ...");
        self.device_event_handlers.as_mut().unwrap().on_mouse_wheel = handler;
        self
    }

    pub fn on_device_motion(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_device_motion ...");
        self.device_event_handlers.as_mut().unwrap().on_motion = handler;
        self
    }

    pub fn on_device_button(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_device_button ...");
        self.device_event_handlers.as_mut().unwrap().on_button = handler;
        self
    }

    pub fn on_device_key(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_device_key ...");
        self.device_event_handlers.as_mut().unwrap().on_key = handler;
        self
    }

    pub fn on_device_text(&mut self, handler: Handler<S, E>) -> &mut Self {
        debug!("subcribed to on_device_text ...");
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
            trace!("event {:?}", event)
        }
        _ => debug!("event {:?}", event),
    }
}
