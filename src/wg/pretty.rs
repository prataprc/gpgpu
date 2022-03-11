use prettytable::{cell, row};

use crate::util::PrettyRow;

impl PrettyRow for wgpu_core::hub::StorageReport {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "NumOccupied", "NumVacant", "NumError", "ElementSize"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.num_occupied,
            self.num_vacant,
            self.num_error,
            self.element_size,
        ]
    }
}
