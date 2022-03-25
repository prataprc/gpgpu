//! Package implement event-loop and window handling, uses [winit] as backend.
//!
//! Start with [Handle] type. Instantiating an `Handle` shall create an event_loop,
//! and an associated window object, subsequently this can be used as surface for `wgpu`.
//! Internally `Handle` uses [winit] for `event_loop` and `window-handle`.
//!
//! For an exhaustive list of all possible events, refer [Event]. Application can
//! subscribe handlers for individual events using the [Handle]. Following is the list
//! of individual events and its corresponding handler types.
//!
//! |   Event                               |   Handler type
//! |---------------------------------------|-------------------------------------------
//! |  [Event::NewEvents]                   | on_new_events([Handler]<[StartCause], T, ()>)
//! |  [Event::UserEvent]                   | on_user_event([Handler]<T, T, ()>)
//! |  [Event::Suspended]                   | on_suspended([HandlerNoArg]<T, ()>)
//! |  [Event::Resumed]                     | on_resumed([HandlerNoArg]<T, ()>)
//! |  [Event::MainEventsCleared]           | on_main_events_cleared([HandlerNoArg]<T, ()>)
//! |  [Event::RedrawRequested]             | on_redraw_requested([HandlerNoArg]<T, ()>)
//! |  [Event::RedrawEventsCleared]         | on_redraw_events_cleared([HandlerNoArg]<T, ()>)
//! |  [Event::LoopDestroyed]               | on_loop_destroyed([HandlerNoArg]<T, ()>)
//! |  [WindowEvent::Resized]               | on_win_resized([Handler]<[PhysicalSize<u32>], T, ()>)
//! |  [WindowEvent::Moved]                 | on_win_moved([Handler]<[PhysicalPosition<i32>], T, ()>)
//! |  [WindowEvent::CloseRequested]        | on_win_close_requested([HandlerNoArg]<T, ()>)
//! |  [WindowEvent::Destroyed]             | on_win_destroyed([HandlerNoArg]<T, ()>)
//! |  [WindowEvent::DroppedFile]           | on_win_dropped_file([Handler]<[PathBuf], T, ()>)
//! |  [WindowEvent::HoveredFile]           | on_win_hovered_file([Handler]<[PathBuf], T, ()>)
//! |  [WindowEvent::HoveredFileCancelled]  | on_win_hovered_file_cancelled([HandlerNoArg]<T, ()>)
//! |  [WindowEvent::ReceivedCharacter]     | on_win_received_character([Handler]<char, T, ()>)
//! |  [WindowEvent::Focused]               | on_win_focused([Handler]<bool, T, ()>)
//! |  [WindowEvent::KeyboardInput]         | on_win_keyboard_input([Handler]<[WinKeyboardInput], T, ()>)
//! |  [WindowEvent::ModifiersChanged]      | on_win_modifiers_changed([Handler]<[ModifiersState], T, ()>)
//! |  [WindowEvent::CursorMoved]           | on_win_cursor_moved([Handler]<[WinCursorMoved], T, ()>)
//! |  [WindowEvent::CursorEntered]         | on_win_cursor_entered([Handler]<[WinCursorEntered], T, ()>)
//! |  [WindowEvent::CursorLeft]            | on_win_cursor_left([Handler]<[WinCursorLeft], T, ()>)
//! |  [WindowEvent::MouseWheel]            | on_win_mouse_wheel([Handler]<[WinMouseWheel], T, ()>)
//! |  [WindowEvent::MouseInput]            | on_win_mouse_input([Handler]<[WinMouseInput], T, ()>)
//! |  [WindowEvent::TouchpadPressure]      | on_win_touchpad_pressure([Handler]<[WinTouchpadPressure], T, ()>)
//! |  [WindowEvent::AxisMotion]            | on_win_axis_motion([Handler]<[WinAxisMotion], T, ()>)
//! |  [WindowEvent::Touch]                 | on_win_touch([Handler]<[Touch], T, ()>)
//! |  [WindowEvent::ScaleFactorChanged]    | on_win_scale_factor_changed([Handler]<f64, T, [PhysicalSize<u32>]>)
//! |  [WindowEvent::ThemeChanged]          | on_win_theme_changed([Handler]<[Theme], T, ()>)
//! |  [DeviceEvent::Added]                 | on_device_added([HandlerNoArg]<T, ()>)
//! |  [DeviceEvent::Removed]               | on_device_removed([HandlerNoArg]<T, ()>)
//! |  [DeviceEvent::MouseMotion]           | on_device_mouse_motion([Handler]<[DeviceMouseMotion], T, ()>)
//! |  [DeviceEvent::MouseWheel]            | on_device_mouse_wheel([Handler]<[DeviceMouseWheel], T, ()>)
//! |  [DeviceEvent::Motion]                | on_device_motion([Handler]<[DeviceMotion], T, ()>)
//! |  [DeviceEvent::Button]                | on_device_button([Handler]<[DeviceButton], T, ()>)
//! |  [DeviceEvent::Key]                   | on_device_key([Handler]<[KeyboardInput], T, ()>)
//! |  [DeviceEvent::Text]                  | on_device_text([Handler]<char, T, ()>)

mod config;
mod event_loop;
mod pretty;

#[allow(unused_imports)]
use std::path::PathBuf;
#[allow(unused_imports)]
use winit::{
    dpi::{PhysicalPosition, PhysicalSize},
    event::{
        DeviceEvent, Event, KeyboardInput, ModifiersState, StartCause, Touch, WindowEvent,
    },
    window::Theme,
};

pub use config::WinitConfig;
pub use event_loop::{
    DeviceButton, DeviceMotion, DeviceMouseMotion, DeviceMouseWheel, WinAxisMotion,
    WinCursorEntered, WinCursorLeft, WinCursorMoved, WinKeyboardInput, WinMouseInput,
    WinMouseWheel, WinTouchpadPressure,
};
pub use event_loop::{Handle, Handler, HandlerNoArg, HandlerRes};
