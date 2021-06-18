use colored::Colorize;
use prettytable::{cell, row, Row};
use vulkano::{
    format::{Format, FormatProperties},
    image::ImageAspects,
    instance::{
        LayerProperties, MemoryHeap, MemoryType, PhysicalDevice, PhysicalDeviceType,
        QueueFamily,
    },
};

#[macro_export]
macro_rules! format_unwrap_or {
    ($val:expr, $def:expr) => {
        $val.map(|x| x.to_string()).unwrap_or($def.clone())
    };
}

#[macro_export]
macro_rules! make_list {
    ($(($items:ident, $field:ident),)*) => (
        vec![
            $(
                match $items.$field {
                    true => stringify!($field),
                    false => "",
                },
            )*
        ]
    );
    ($(($items:ident, $field:ident, $val:expr),)*) => (
        vec![
            $(
                match $items.$field {
                    true => $val,
                    false => "",
                },
            )*
        ]
    );
}

macro_rules! format_props {
    ($val:ident, $($field:ident,)*) => (
        vec![
            $(
                match ($val.linear_tiling_features.$field, $val.optimal_tiling_features.$field, $val.buffer_features.$field) {
                    (false, false, false) => "-".to_string(),
                    (a, b, c) => format_cell_content(a, b, c).to_string(),
                },
            )*
        ]
    );
}

pub trait PrettyRow {
    fn to_format() -> prettytable::format::TableFormat;

    fn to_head() -> prettytable::Row;

    fn to_row(&self) -> prettytable::Row;
}

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

impl<'a> PrettyRow for MemoryHeap<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "MemoryHeap", "Size", "DEVICE_LOCAL", "MULTI_INSTANCE"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.id(),
            self.size(),
            self.is_device_local(),
            self.is_multi_instance()
        ]
    }
}

impl<'a> PrettyRow for QueueFamily<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "QueueId", "Count", "ImageTxGranularity", "Graphics", "Compute", "Sparse", "XTransfer"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.id(),
            self.queues_count(),
            format!("{:?}", self.min_image_transfer_granularity()),
            if self.supports_graphics() {
                "✓"
            } else {
                "✗"
            },
            if self.supports_compute() {
                "✓"
            } else {
                "✗"
            },
            if self.explicitly_supports_transfers() {
                "✓"
            } else {
                "✗"
            },
            if self.supports_sparse_binding() {
                "✓"
            } else {
                "✗"
            },
        ]
    }
}

impl<'a> PrettyRow for MemoryType<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "MemoryType", "Heap", "LOCAL", "VISIBLE", "CACHED", "COHERENT", "LAZY"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.id(),
            self.heap().id(),
            self.is_device_local(),
            self.is_host_visible(),
            self.is_host_cached(),
            self.is_host_coherent(),
            self.is_lazily_allocated(),
        ]
    }
}

// TODO: don't do this as table, there are way too many details.
impl<'a> PrettyRow for PhysicalDevice<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Index", "Name", "Type", "DeviceID", "VendorID", "API", "Driver"]
    }

    fn to_row(&self) -> prettytable::Row {
        let uuid_to_s = |id: &[u8; 16]| {
            uuid::Uuid::from_slice(&id[..])
                .unwrap()
                .to_hyphenated()
                .to_string()
        };

        let properties = self.properties();
        let none = "-none-".to_string();

        row![
            self.index(),
            format!(
                "{}\nDEVICE_UUID:{}\nDRIVER_UUID:{}",
                properties.device_name.as_ref().unwrap_or(&none),
                properties
                    .device_uuid
                    .as_ref()
                    .map(uuid_to_s)
                    .unwrap_or(none.clone()),
                properties
                    .driver_uuid
                    .as_ref()
                    .map(uuid_to_s)
                    .unwrap_or(none.clone())
            ),
            physical_device_type_to_str(properties.device_type.unwrap()),
            &format!("{:x}", properties.pci_device.unwrap()),
            &format!("{:x}", properties.vendor_id.unwrap()),
            properties.api_version.unwrap(),
            properties.driver_version.unwrap(),
        ]
    }
}

impl PrettyRow for FormatProperties {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy =>
            "Format", "sai", "sti", "sia", "utb", "stb", "stba",
            "vtb", "cra", "cab", "dsa", "bts", "btd", "sifl",
            "txs", "txd", "mcs", "ycl", "ycs", "ycc", "ycf",
            "disj", "ccs", "sifm", "sifc", "asvb", "fdm"
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        use std::iter::FromIterator;

        let cells = format_props![
            self,
            sampled_image,
            storage_image,
            storage_image_atomic,
            uniform_texel_buffer,
            storage_texel_buffer,
            storage_texel_buffer_atomic,
            vertex_buffer,
            color_attachment,
            color_attachment_blend,
            depth_stencil_attachment,
            blit_src,
            blit_dst,
            sampled_image_filter_linear,
            transfer_src,
            transfer_dst,
            midpoint_chroma_samples,
            sampled_image_ycbcr_conversion_linear_filter,
            sampled_image_ycbcr_conversion_separate_reconstruction_filter,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit,
            sampled_image_ycbcr_conversion_chroma_reconstruction_explicit_forceable,
            disjoint,
            cosited_chroma_samples,
            sampled_image_filter_minmax,
            img_sampled_image_filter_cubic,
            khr_acceleration_structure_vertex_buffer,
            ext_fragment_density_map,
        ];

        Row::from_iter(cells.into_iter())
    }
}

impl PrettyRow for Format {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "Format", "DataType", "Size(Bytes)", "BlockDimn", "Planes", "Aspects" ]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            format!("{:?}", self),
            format!("{:?}", self.ty()),
            self.size()
                .map(|a| a.to_string())
                .unwrap_or("-".to_string()),
            format!("{:?}", self.block_dimensions()),
            self.planes(),
            image_aspects(self.aspects()),
        ]
    }
}

fn physical_device_type_to_str(ty: PhysicalDeviceType) -> &'static str {
    match ty {
        PhysicalDeviceType::IntegratedGpu => "IntegratedGpu",
        PhysicalDeviceType::DiscreteGpu => "DiscreteGpu",
        PhysicalDeviceType::VirtualGpu => "VirtualGpu",
        PhysicalDeviceType::Cpu => "Cpu",
        PhysicalDeviceType::Other => "Other",
    }
}

fn format_cell_content(a: bool, b: bool, c: bool) -> String {
    let mut s = String::default();
    match a {
        true => s.push_str(&"✓".green().to_string()),
        false => s.push_str(&"✗".red().to_string()),
    }
    match b {
        true => s.push_str(&"✓".green().to_string()),
        false => s.push_str(&"✗".red().to_string()),
    }
    match c {
        true => s.push_str(&"✓".green().to_string()),
        false => s.push_str(&"✗".red().to_string()),
    }
    s
}

fn image_aspects(val: ImageAspects) -> String {
    let ss: Vec<&str> = make_list![
        (val, color),
        (val, depth),
        (val, stencil),
        (val, metadata),
        (val, plane0),
        (val, plane1),
        (val, plane2),
        (val, memory_plane0),
        (val, memory_plane1),
        (val, memory_plane2),
    ]
    .into_iter()
    .filter(|s| s.len() > 0)
    .collect();
    ss.join(", ")
}
