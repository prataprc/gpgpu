//! Vulkan toolkit.

use prettytable::{cell, row};
use vulkano::instance::{InstanceExtensions, LayerProperties};

use crate::{Error, Result};

pub trait PrettyRow {
    fn to_format() -> prettytable::format::TableFormat;

    fn to_head() -> prettytable::Row;

    fn to_row(&self) -> prettytable::Row;
}

impl PrettyRow for LayerProperties {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Name", "Description", "Vulkan Version", "Layer Version"]
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

pub fn layers() -> Result<Vec<LayerProperties>> {
    use vulkano::instance::layers_list;

    Ok(err_at!(Vk, layers_list())?.collect())
}

pub fn extensions() -> Result<InstanceExtensions> {
    err_at!(Vk, InstanceExtensions::supported_by_core())
}

//pub fn make_instance() -> Result<Instance> {
//    let instance = match Instance::new(None, &InstanceExtensions::none(), None) {
//        Ok(i) => i,
//        Err(err) => panic!("Couldn't build instance: {:?}", err),
//    };
//}
