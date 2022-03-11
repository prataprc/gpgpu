use prettytable::{cell, row};
use winit::monitor::{MonitorHandle, VideoMode};

use crate::util::PrettyRow;

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
