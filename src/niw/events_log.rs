use winit::event::{DeviceEvent, Event, StartCause, WindowEvent};

use std::{collections::BTreeMap, time};

pub struct EventsLog {
    event_names: BTreeMap<String, u64>,
    genesis: time::Instant,
}

impl Default for EventsLog {
    fn default() -> EventsLog {
        EventsLog {
            event_names: BTreeMap::new(),
            genesis: time::Instant::now(),
        }
    }
}

impl EventsLog {
    pub fn append<T>(&mut self, event: &Event<T>) {
        let name = to_event_name(event).to_string();

        let n = self.event_names.get(&name).map(|v| *v).unwrap_or(0);
        self.event_names.insert(name, n + 1);
    }

    pub fn pretty_print(&self) {
        let duration = self.genesis.elapsed();
        for (key, value) in self.event_names.iter() {
            let rate = match duration.as_secs() {
                secs if secs > 0 => value / secs,
                _ => *value,
            };
            println!("{:030} : {:10}/{}", key, value, rate)
        }
    }
}

// TODO: Should this be moved into a common module like niw/mod.rs
pub fn to_event_name<T>(event: &Event<T>) -> &'static str {
    match event {
        Event::NewEvents(cause) => match cause {
            StartCause::ResumeTimeReached { .. } => "Event::NewEvents(ResumeTimeReached)",
            StartCause::WaitCancelled { .. } => "Event::NewEvents(WaitCancelled)",
            StartCause::Poll => "Event::NewEvents(Pool)",
            StartCause::Init => "Event::NewEvents(Init)",
        },
        Event::WindowEvent { event, .. } => match event {
            WindowEvent::Resized(_) => "WindowEvent::Resized",
            WindowEvent::Moved(_) => "WindowEvent::Moved",
            WindowEvent::CloseRequested => "WindowEvent::CloseRequested",
            WindowEvent::Destroyed => "WindowEvent::Destroyed",
            WindowEvent::DroppedFile(_) => "WindowEvent::DroppedFile",
            WindowEvent::HoveredFile(_) => "WindowEvent::HoveredFile",
            WindowEvent::HoveredFileCancelled => "WindowEvent::HoveredFileCancelled",
            WindowEvent::ReceivedCharacter(_) => "WindowEvent::ReceivedCharacter",
            WindowEvent::Focused(_) => "WindowEvent::Focused",
            WindowEvent::KeyboardInput { .. } => "WindowEvent::KeyboardInput",
            WindowEvent::ModifiersChanged(_) => "WindowEvent::ModifiersChanged",
            WindowEvent::CursorMoved { .. } => "WindowEvent::CursorMoved",
            WindowEvent::CursorEntered { .. } => "WindowEvent::CursorEntered",
            WindowEvent::CursorLeft { .. } => "WindowEvent::CursorLeft",
            WindowEvent::MouseWheel { .. } => "WindowEvent::MouseWheel",
            WindowEvent::MouseInput { .. } => "WindowEvent::MouseInput",
            WindowEvent::TouchpadPressure { .. } => "WindowEvent::TouchpadPressure",
            WindowEvent::AxisMotion { .. } => "WindowEvent::AxisMotion",
            WindowEvent::Touch(_) => "WindowEvent::Touch",
            WindowEvent::ScaleFactorChanged { .. } => "WindowEvent::ScaleFactorChanged",
            WindowEvent::ThemeChanged(_) => "WindowEvent::ThemeChanged",
        },
        Event::DeviceEvent { event, .. } => match event {
            DeviceEvent::Added => "DeviceEvent::Added",
            DeviceEvent::Removed => "DeviceEvent::Removed",
            DeviceEvent::MouseMotion { .. } => "DeviceEvent::MouseMotion",
            DeviceEvent::MouseWheel { .. } => "DeviceEvent::MouseWheel",
            DeviceEvent::Motion { .. } => "DeviceEvent::Motion",
            DeviceEvent::Button { .. } => "DeviceEvent::Button",
            DeviceEvent::Key(_) => "DeviceEvent::Key",
            DeviceEvent::Text { .. } => "DeviceEvent::Text",
        },
        Event::UserEvent(_) => "Event::UserEvent",
        Event::Suspended => "Event::Suspended",
        Event::Resumed => "Event::Resumed",
        Event::MainEventsCleared => "Event::MainEventsCleared",
        Event::RedrawRequested(_) => "Event::RedrawEventsCleared",
        Event::RedrawEventsCleared => "Event::RedrawEventsCleared",
        Event::LoopDestroyed => "Event::LoopDestroyed",
    }
}
