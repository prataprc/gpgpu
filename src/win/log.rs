use winit::event::{Event, StartCause};

use std::{fmt, time};

use crate::{Error, Result};

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

