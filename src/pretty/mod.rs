//! Package implement pretty printing for [wgpu] features, limits, formats, textures etc.

mod features;
mod limits;
mod texture_formats;

pub use features::{add_adapter_to_features, features, Feature};
pub use limits::{add_adapter_to_limits, limits, Limit};
pub use texture_formats::{
    texture_format_flags, texture_formats_info, texture_usages, TextureFormatInfo,
};

use prettytable::{cell, row};

use crate::util::PrettyRow;

#[derive(Clone)]
pub struct StorageReport {
    report: wgpu_core::hub::StorageReport,
    name: String,
}

impl<'a> From<(&'a str, wgpu_core::hub::StorageReport)> for StorageReport {
    fn from((name, report): (&str, wgpu_core::hub::StorageReport)) -> Self {
        StorageReport {
            name: name.to_string(),
            report,
        }
    }
}

impl PrettyRow for StorageReport {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Name", "NumOccupied", "NumVacant", "NumError", "ElementSize"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.name,
            self.report.num_occupied,
            self.report.num_vacant,
            self.report.num_error,
            self.report.element_size,
        ]
    }
}

impl PrettyRow for wgpu::AdapterInfo {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Name", "Vendor", "Device", "DeviceType", "Backend"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.name,
            self.vendor,
            self.device,
            format!("{:?}", self.device_type),
            format!("{:?}", self.backend),
        ]
    }
}
