use prettytable::{cell, row};
use winit::{
    event::{Event, StartCause},
    monitor::{MonitorHandle, VideoMode},
};

use std::{fmt, time};

use crate::{util::PrettyRow, Error, Result};

pub struct LogEvent<'a, T>
where
    T: 'static,
{
    pub elapsed: time::Duration,
    pub event: Event<'a, T>,
}

#[derive(Default)]
pub struct Events<'a, T>
where
    T: 'static,
{
    pub events: Vec<LogEvent<'a, T>>,
    pub n_new_events_poll: usize,
    pub n_main_events_cleared: usize,
    pub n_redraw_events_cleared: usize,
}

impl<'a, T> Events<'a, T>
where
    T: 'static,
{
    pub fn append(&mut self, event: Event<'a, T>) -> Result<()>
    where
        T: fmt::Debug,
    {
        let elapsed = err_at!(Fatal, time::UNIX_EPOCH.elapsed())?;
        match event {
            Event::NewEvents(StartCause::Poll) => self.n_new_events_poll += 1,
            Event::MainEventsCleared => self.n_main_events_cleared += 1,
            Event::RedrawEventsCleared => self.n_redraw_events_cleared += 1,
            event => {
                println!("{:?}", event);
                self.events.push(LogEvent { elapsed, event })
            }
        }

        Ok(())
    }
}

impl PrettyRow for MonitorHandle {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Name", "Position", "Size", "Scan-factor", "Video-modes"]
    }

    fn to_row(&self) -> prettytable::Row {
        let name = self.name().unwrap_or("-no-name-".to_string());
        let size = self.size();
        let post = self.position();
        let scale_factor = self.scale_factor();
        let modes: Vec<VideoMode> = self.video_modes().collect();

        row![
            name,
            format!("{},{}", post.x, post.y),
            format!("{}x{}", size.width, size.height),
            scale_factor,
            modes.len()
        ]
    }
}

impl PrettyRow for VideoMode {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Size", "Bit-depth", "Refresh-rate"]
    }

    fn to_row(&self) -> prettytable::Row {
        let size = self.size();

        row![
            format!("{}x{}", size.width, size.height),
            self.bit_depth(),
            self.refresh_rate(),
        ]
    }
}
