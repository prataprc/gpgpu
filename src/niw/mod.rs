//! Package implement event-loop and window handling, uses [winit] as backend.
//!
//! Start with [SingleWindow] type. Constructing an `SingleWindow` shall create an
//! event_loop, and an associated window object, subsequently this can be used as
//! surface for `wgpu`. Internally `SingleWindow` uses [winit] for `event_loop` and
//! `window-handle`. This is suitable for applications required only one window.
//!
//! For an exhaustive list of all possible events, refer [Event]. Application can
//! subscribe handlers for individual events using the [SingleWindow] instance.
//! Following is the list of individual events and its corresponding handler types.
//!
//! |   Event                               |   Handler type
//! |---------------------------------------|-------------------------------------------
//! |  [Event::NewEvents]                   | on_new_events([Handler])
//! |  [Event::UserEvent]                   | on_user_event([Handler])
//! |  [Event::Suspended]                   | on_suspended([Handler])
//! |  [Event::Resumed]                     | on_resumed([Handler])
//! |  [Event::MainEventsCleared]           | on_main_events_cleared([Handler])
//! |  [Event::RedrawRequested]             | on_redraw_requested([Handler])
//! |  [Event::RedrawEventsCleared]         | on_redraw_events_cleared([Handler])
//! |  [Event::LoopDestroyed]               | on_loop_destroyed([Handler])
//! |  [WindowEvent::Resized]               | on_win_resized([Handler])
//! |  [WindowEvent::Moved]                 | on_win_moved([Handler])
//! |  [WindowEvent::CloseRequested]        | on_win_close_requested([Handler])
//! |  [WindowEvent::Destroyed]             | on_win_destroyed([Handler])
//! |  [WindowEvent::DroppedFile]           | on_win_dropped_file([Handler])
//! |  [WindowEvent::HoveredFile]           | on_win_hovered_file([Handler])
//! |  [WindowEvent::HoveredFileCancelled]  | on_win_hovered_file_cancelled([Handler])
//! |  [WindowEvent::ReceivedCharacter]     | on_win_received_character([Handler])
//! |  [WindowEvent::Focused]               | on_win_focused([Handler])
//! |  [WindowEvent::KeyboardInput]         | on_win_keyboard_input([Handler])
//! |  [WindowEvent::ModifiersChanged]      | on_win_modifiers_changed([Handler])
//! |  [WindowEvent::CursorMoved]           | on_win_cursor_moved([Handler])
//! |  [WindowEvent::CursorEntered]         | on_win_cursor_entered([Handler])
//! |  [WindowEvent::CursorLeft]            | on_win_cursor_left([Handler])
//! |  [WindowEvent::MouseWheel]            | on_win_mouse_wheel([Handler])
//! |  [WindowEvent::MouseInput]            | on_win_mouse_input([Handler])
//! |  [WindowEvent::TouchpadPressure]      | on_win_touchpad_pressure([Handler])
//! |  [WindowEvent::AxisMotion]            | on_win_axis_motion([Handler])
//! |  [WindowEvent::Touch]                 | on_win_touch([Handler])
//! |  [WindowEvent::ScaleFactorChanged]    | on_win_scale_factor_changed([Handler])
//! |  [WindowEvent::ThemeChanged]          | on_win_theme_changed([Handler])
//! |  [DeviceEvent::Added]                 | on_device_added([Handler])
//! |  [DeviceEvent::Removed]               | on_device_removed([Handler])
//! |  [DeviceEvent::MouseMotion]           | on_device_mouse_motion([Handler])
//! |  [DeviceEvent::MouseWheel]            | on_device_mouse_wheel([Handler])
//! |  [DeviceEvent::Motion]                | on_device_motion([Handler])
//! |  [DeviceEvent::Button]                | on_device_button([Handler])
//! |  [DeviceEvent::Key]                   | on_device_key([Handler])
//! |  [DeviceEvent::Text]                  | on_device_text([Handler])

mod events_log;
mod pretty;
mod single_window;

pub use events_log::{to_event_name, EventsLog};
pub use single_window::{Handler, SingleWindow};

#[allow(unused_imports)]
use winit::{
    dpi::{LogicalSize, PhysicalPosition, PhysicalSize},
    event::{
        DeviceEvent, Event, KeyboardInput, ModifiersState, StartCause, Touch, WindowEvent,
    },
    event_loop::EventLoop,
    window::Theme,
};

#[allow(unused_imports)]
use std::path::PathBuf;
use std::{fmt, result};

use crate::{Error, Result};

pub struct Monitor {
    pub name: String,
    pub size: PhysicalSize<u32>,
    pub scale_factor: f64,
}

impl fmt::Display for Monitor {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(
            f,
            "MONITOR: {:?}, physical_size:{}x{} scale_factor:{}",
            self.name, self.size.width, self.size.height, self.scale_factor
        )
    }
}

impl Monitor {
    pub fn to_logical_size(&self) -> LogicalSize<u32> {
        self.size.to_logical(self.scale_factor)
    }
}

pub fn get_monitor_info() -> Result<Monitor> {
    let ev = EventLoop::new();
    let mh = match ev.primary_monitor() {
        Some(mh) => mh,
        None => match ev.available_monitors().next() {
            Some(mh) => mh.clone(),
            None => err_at!(Invalid, msg: "Cannot find a monitor, check the cables")?,
        },
    };

    let val = Monitor {
        name: mh.name().unwrap_or("unnamed-monitor".to_string()),
        size: mh.size(),
        scale_factor: mh.scale_factor(),
    };

    Ok(val)
}
