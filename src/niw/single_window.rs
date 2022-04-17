use log::{debug, info, trace, warn};
use winit::{
    dpi,
    event::{DeviceEvent, Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoop, EventLoopWindowTarget},
    window::{Window, WindowAttributes, WindowBuilder},
};

use std::fmt;

#[allow(unused_imports)]
use crate::ConfigWinit;
use crate::{Error, Result};

/// Type to handle events with an event-argument `E`, window-target and context `C`.
pub type Handler<C, E> =
    Box<dyn FnMut(&Window, &mut C, &mut Event<E>) -> Option<ControlFlow>>;

/// Type instantiates an event-loop and an associated window, useful for single window
/// applications.
///
/// Can be constructed from [ConfigWinit] configuration, refer
/// [SingleWindow::from_config]. This type parameterised over user-event `E` for
/// [EventLoop]
pub struct SingleWindow<C, E = ()>
where
    E: 'static,
{
    event_loop: Option<EventLoop<E>>,
    window: Option<Window>,
    on_event: Option<Handler<C, E>>,
    event_handlers: Option<EventHandlers<C, E>>,
    window_event_handlers: Option<WindowEventHandlers<C, E>>,
    device_event_handlers: Option<DeviceEventHandlers<C, E>>,
}

struct EventHandlers<C, E>
where
    E: 'static,
{
    on_new_events: Handler<C, E>,
    on_user_event: Handler<C, E>,
    on_suspended: Handler<C, E>,
    on_resumed: Handler<C, E>,
    on_main_events_cleared: Handler<C, E>,
    on_redraw_requested: Handler<C, E>,
    on_redraw_events_cleared: Handler<C, E>,
    on_loop_destroyed: Handler<C, E>,
}

impl<C, E> Default for EventHandlers<C, E> {
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

struct WindowEventHandlers<C, E>
where
    E: 'static,
{
    on_resized: Handler<C, E>,
    on_moved: Handler<C, E>,
    on_close_requested: Handler<C, E>,
    on_destroyed: Handler<C, E>,
    on_dropped_file: Handler<C, E>,
    on_hovered_file: Handler<C, E>,
    on_hovered_file_cancelled: Handler<C, E>,
    on_received_character: Handler<C, E>,
    on_focused: Handler<C, E>,
    on_keyboard_input: Handler<C, E>,
    on_modifiers_changed: Handler<C, E>,
    on_cursor_moved: Handler<C, E>,
    on_cursor_entered: Handler<C, E>,
    on_cursor_left: Handler<C, E>,
    on_mouse_wheel: Handler<C, E>,
    on_mouse_input: Handler<C, E>,
    on_touchpad_pressure: Handler<C, E>,
    on_axis_motion: Handler<C, E>,
    on_touch: Handler<C, E>,
    on_scale_factor_changed: Handler<C, E>,
    on_theme_changed: Handler<C, E>,
}

impl<C, E> Default for WindowEventHandlers<C, E> {
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

struct DeviceEventHandlers<C, E>
where
    E: 'static,
{
    on_added: Handler<C, E>,
    on_removed: Handler<C, E>,
    on_mouse_motion: Handler<C, E>,
    on_mouse_wheel: Handler<C, E>,
    on_motion: Handler<C, E>,
    on_button: Handler<C, E>,
    on_key: Handler<C, E>,
    on_text: Handler<C, E>,
}

impl<C, E> Default for DeviceEventHandlers<C, E> {
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

impl<C, E> SingleWindow<C, E>
where
    E: 'static,
{
    pub fn from_config(mut attrs: WindowAttributes) -> Result<Self>
    where
        E: Default,
    {
        let event_loop = EventLoop::<E>::with_user_event();
        let scale_factor = match event_loop.primary_monitor() {
            Some(mont) => mont.scale_factor() as f32,
            None => match event_loop.available_monitors().next() {
                Some(mont) => mont.scale_factor() as f32,
                None => 1.0_f32,
            },
        };
        attrs.inner_size = match attrs.inner_size {
            Some(dpi::Size::Physical(dpi::PhysicalSize { width, height })) => {
                let size = dpi::PhysicalSize {
                    width: ((width as f32) * scale_factor).floor() as u32,
                    height: ((height as f32) * scale_factor).floor() as u32,
                };
                Some(dpi::Size::Physical(size))
            }
            val => val,
        };
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
        };

        Ok(val)
    }

    pub fn as_event_loop(&self) -> &EventLoop<E> {
        self.event_loop.as_ref().unwrap()
    }

    pub fn as_window(&self) -> &Window {
        self.window.as_ref().unwrap()
    }

    pub fn run(mut self, mut r: C) -> !
    where
        C: 'static,
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
                let mut no_op: Handler<C, E> = Box::new(|_, _, _| None);

                (on_event)(&window, &mut r, &mut evnt);

                let handler = match &evnt {
                    Event::NewEvents(_) => &mut event_handlers.on_new_events,
                    Event::UserEvent(_) => &mut event_handlers.on_user_event,
                    Event::Suspended => &mut event_handlers.on_suspended,
                    Event::Resumed => &mut event_handlers.on_resumed,
                    Event::MainEventsCleared => {
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
                            WindowEvent::Resized(_) => {
                                &mut window_event_handlers.on_resized
                            }
                            WindowEvent::Moved(_) => &mut window_event_handlers.on_moved,
                            WindowEvent::CloseRequested => {
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
                            WindowEvent::ScaleFactorChanged { .. } => {
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

                match handler(&window, &mut r, &mut evnt) {
                    Some(val) => *cf = val,
                    None => (),
                }
            },
        );
    }
}

impl<C, E> SingleWindow<C, E>
where
    E: 'static,
{
    pub fn on_event(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_event ...");
        self.on_event = Some(handler);
        self
    }

    pub fn on_new_events(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_new_events ...");
        self.event_handlers.as_mut().unwrap().on_new_events = handler;
        self
    }

    pub fn on_user_event(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_user_event ...");
        self.event_handlers.as_mut().unwrap().on_user_event = handler;
        self
    }

    pub fn on_suspended(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_suspended ...");
        self.event_handlers.as_mut().unwrap().on_suspended = handler;
        self
    }

    pub fn on_resumed(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_resumed ...");
        self.event_handlers.as_mut().unwrap().on_resumed = handler;
        self
    }

    pub fn on_main_events_cleared(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_main_events_cleared ...");
        self.event_handlers.as_mut().unwrap().on_main_events_cleared = handler;
        self
    }

    pub fn on_redraw_requested(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_redraw_requested ...");
        self.event_handlers.as_mut().unwrap().on_redraw_requested = handler;
        self
    }

    pub fn on_redraw_events_cleared(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_redraw_events_cleared ...");
        self.event_handlers
            .as_mut()
            .unwrap()
            .on_redraw_events_cleared = handler;
        self
    }

    pub fn on_loop_destroyed(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_loop_destroyed ...");
        self.event_handlers.as_mut().unwrap().on_loop_destroyed = handler;
        self
    }

    pub fn on_win_resized(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_resized ...");
        self.window_event_handlers.as_mut().unwrap().on_resized = handler;
        self
    }

    pub fn on_win_moved(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_moved ...");
        self.window_event_handlers.as_mut().unwrap().on_moved = handler;
        self
    }

    pub fn on_win_close_requested(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_close_requested ...");
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_close_requested = handler;
        self
    }

    pub fn on_win_destroyed(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_destroyed ...");
        self.window_event_handlers.as_mut().unwrap().on_destroyed = handler;
        self
    }

    pub fn on_win_dropped_file(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_dropped_file ...");
        self.window_event_handlers.as_mut().unwrap().on_dropped_file = handler;
        self
    }

    pub fn on_win_hovered_file(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_hovered_file ...");
        self.window_event_handlers.as_mut().unwrap().on_hovered_file = handler;
        self
    }

    pub fn on_win_hovered_file_cancelled(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_hovered_file_cancelled ...");
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_hovered_file_cancelled = handler;
        self
    }

    pub fn on_win_received_character(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_received_character ...");
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_received_character = handler;
        self
    }

    pub fn on_win_focused(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_focused ...");
        self.window_event_handlers.as_mut().unwrap().on_focused = handler;
        self
    }

    pub fn on_win_keyboard_input(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_keyboard_input ...");
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_keyboard_input = handler;
        self
    }

    pub fn on_win_modifiers_changed(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_modifiers_changed ...");
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_modifiers_changed = handler;
        self
    }

    pub fn on_win_cursor_moved(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_cursor_moved ...");
        self.window_event_handlers.as_mut().unwrap().on_cursor_moved = handler;
        self
    }

    pub fn on_win_cursor_entered(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_cursor_entered ...");
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_cursor_entered = handler;
        self
    }

    pub fn on_win_cursor_left(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_cursor_left ...");
        self.window_event_handlers.as_mut().unwrap().on_cursor_left = handler;
        self
    }

    pub fn on_win_mouse_wheel(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_mouse_wheel ...");
        self.window_event_handlers.as_mut().unwrap().on_mouse_wheel = handler;
        self
    }

    pub fn on_win_mouse_input(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_mouse_input ...");
        self.window_event_handlers.as_mut().unwrap().on_mouse_input = handler;
        self
    }

    pub fn on_win_touchpad_pressure(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_touchpad_pressure ...");
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_touchpad_pressure = handler;
        self
    }

    pub fn on_win_axis_motion(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_axis_motion ...");
        self.window_event_handlers.as_mut().unwrap().on_axis_motion = handler;
        self
    }

    pub fn on_win_touch(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_touch ...");
        self.window_event_handlers.as_mut().unwrap().on_touch = handler;
        self
    }

    pub fn on_win_scale_factor_changed(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_scale_factor_changed ...");
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_scale_factor_changed = handler;
        self
    }

    pub fn on_win_theme_changed(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_win_theme_changed ...");
        self.window_event_handlers
            .as_mut()
            .unwrap()
            .on_theme_changed = handler;
        self
    }

    pub fn on_device_added(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_device_added ...");
        self.device_event_handlers.as_mut().unwrap().on_added = handler;
        self
    }

    pub fn on_device_removed(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_device_removed ...");
        self.device_event_handlers.as_mut().unwrap().on_removed = handler;
        self
    }

    pub fn on_device_mouse_motion(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_device_motion ...");
        self.device_event_handlers.as_mut().unwrap().on_mouse_motion = handler;
        self
    }

    pub fn on_device_mouse_wheel(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_device_mouse_wheel ...");
        self.device_event_handlers.as_mut().unwrap().on_mouse_wheel = handler;
        self
    }

    pub fn on_device_motion(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_device_motion ...");
        self.device_event_handlers.as_mut().unwrap().on_motion = handler;
        self
    }

    pub fn on_device_button(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_device_button ...");
        self.device_event_handlers.as_mut().unwrap().on_button = handler;
        self
    }

    pub fn on_device_key(&mut self, handler: Handler<C, E>) -> &mut Self {
        debug!("subcribed to on_device_key ...");
        self.device_event_handlers.as_mut().unwrap().on_key = handler;
        self
    }

    pub fn on_device_text(&mut self, handler: Handler<C, E>) -> &mut Self {
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
