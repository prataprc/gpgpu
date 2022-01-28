use prettytable::{cell, row};
use vulkano::instance::LayerProperties;

use crate::{Error, Result};

pub fn layer_properties() -> Result<Vec<vulkano::instance::LayerProperties>> {
    use vulkano::instance::layers_list;

    let mut layers = vec![];
    for layer in err_at!(Fatal, layers_list())? {
        layers.push(layer)
    }

    Ok(layers)
}

pub fn layer_names() -> Result<Vec<String>> {
    Ok(layer_properties()?
        .into_iter()
        .map(|l| l.name().to_string())
        .collect())
}

#[cfg(feature = "prettytable-rs")]
pub trait PrettyRow {
    fn to_format() -> prettytable::format::TableFormat;

    fn to_head() -> prettytable::Row;

    fn to_row(&self) -> prettytable::Row;
}

#[cfg(feature = "prettytable-rs")]
pub fn make_table<R>(rows: &[R]) -> prettytable::Table
where
    R: PrettyRow,
{
    let mut table = prettytable::Table::new();

    match rows.len() {
        0 => table,
        _ => {
            table.set_titles(R::to_head());
            rows.iter().for_each(|r| {
                table.add_row(r.to_row());
            });
            table.set_format(R::to_format());
            table
        }
    }
}

impl PrettyRow for LayerProperties {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Layer Name", "Description", "Vulkan Version", "Layer Version"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.name(),
            self.description(),
            self.vulkan_version(),
            self.implementation_version()
        ]
    }
}

impl<'a> PrettyRow for vulkano::device::physical::PhysicalDevice<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row!["index",]
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.index()]
    }
}
