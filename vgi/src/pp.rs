use colored::Colorize;
use prettytable::{cell, row, Row, Table};
use vulkano::{
    descriptor::descriptor::ShaderStages,
    format::{Format, FormatProperties},
    image::{ImageAspects, SampleCounts},
    instance::{
        DriverId, LayerProperties, MemoryHeap, MemoryType, PhysicalDevice,
        PointClippingBehavior, QueueFamily, ShaderFloatControlsIndependence,
        SubgroupFeatures,
    },
};

use std::fmt;

#[macro_export]
macro_rules! format_unwrap_or {
    ($val:expr, $tos:ident, $def:expr) => {
        $val.as_ref().map(|x| $tos(x)).unwrap_or($def.to_string())
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
                match (
                    $val.linear_tiling_features.$field,
                    $val.optimal_tiling_features.$field,
                    $val.buffer_features.$field
                ) {
                    (false, false, false) => "-".to_string(),
                    (a, b, c) => format_cell_content(a, b, c).to_string(),
                },
            )*
        ]
    );
}

macro_rules! format_bool {
    ($val:expr) => {
        if $val {
            "✓".green()
        } else {
            "✗".red()
        }
    };
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

fn shader_stages_to_str(val: &ShaderStages) -> String {
    let mut outs = vec![];
    if val.vertex {
        outs.push("vs")
    }
    if val.tessellation_control {
        outs.push("tcs")
    }
    if val.tessellation_evaluation {
        outs.push("tes")
    }
    if val.geometry {
        outs.push("gs")
    }
    if val.fragment {
        outs.push("fs")
    }
    if val.compute {
        outs.push("cs")
    }
    outs.join("|")
}

fn shader_float_control_to_str(val: &ShaderFloatControlsIndependence) -> String {
    match val {
        ShaderFloatControlsIndependence::Float32Only => "float32_only",
        ShaderFloatControlsIndependence::All => "all",
        ShaderFloatControlsIndependence::None => "none",
    }
    .to_string()
}

fn driver_id_to_str(val: &DriverId) -> String {
    match val {
        DriverId::AMDProprietary => "AMDProprietary",
        DriverId::AMDOpenSource => "AMDOpenSource",
        DriverId::MesaRADV => "MesaRADV",
        DriverId::NvidiaProprietary => "NvidiaProprietary",
        DriverId::IntelProprietaryWindows => "IntelProprietaryWindows",
        DriverId::IntelOpenSourceMesa => "IntelOpenSourceMesa",
        DriverId::ImaginationProprietary => "ImaginationProprietary",
        DriverId::QualcommProprietary => "QualcommProprietary",
        DriverId::ARMProprietary => "ARMProprietary",
        DriverId::GoogleSwiftshader => "GoogleSwiftshader",
        DriverId::GGPProprietary => "GGPProprietary",
        DriverId::BroadcomProprietary => "BroadcomProprietary",
        DriverId::MesaLLVMpipe => "MesaLLVMpipe",
        DriverId::MoltenVK => "MoltenVK",
    }
    .to_string()
}

fn sample_counts_to_str(val: &SampleCounts) -> String {
    let mut outs = vec![];
    if val.sample1 {
        outs.push("1")
    }
    if val.sample2 {
        outs.push("2")
    }
    if val.sample4 {
        outs.push("4")
    }
    if val.sample8 {
        outs.push("8")
    }
    if val.sample16 {
        outs.push("16")
    }
    if val.sample32 {
        outs.push("32")
    }
    if val.sample64 {
        outs.push("64")
    }
    outs.join("|")
}

fn point_clipping_to_str(val: &PointClippingBehavior) -> String {
    match val {
        PointClippingBehavior::AllClipPlanes => "AllClipPlanes",
        PointClippingBehavior::UserClipPlanesOnly => "UserClipPlanesOnly",
    }
    .to_string()
}

fn subgroup_to_str(val: &SubgroupFeatures) -> String {
    let mut outs = vec![];

    if val.basic {
        outs.push("basic")
    }
    if val.vote {
        outs.push("vote")
    }
    if val.arithmetic {
        outs.push("arithmetic")
    }
    if val.ballot {
        outs.push("ballot")
    }
    if val.shuffle {
        outs.push("shuffle")
    }
    if val.shuffle_relative {
        outs.push("shuffle_relative")
    }
    if val.clustered {
        outs.push("clustered")
    }
    if val.quad {
        outs.push("quad")
    }
    outs.join("|")
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

impl PrettyRow for vk_parse::Platform {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "name", "protect", "comment"]
    }

    fn to_row(&self) -> prettytable::Row {
        let comment = match &self.comment {
            Some(val) => val.clone(),
            None => "None".to_string(),
        };
        row![self.name, self.protect, comment]
    }
}

impl PrettyRow for vk_parse::VendorId {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "name", "id", "comment"]
    }

    fn to_row(&self) -> prettytable::Row {
        let comment = match &self.comment {
            Some(val) => val.clone(),
            None => "None".to_string(),
        };
        row![self.name, self.id, comment]
    }
}

impl PrettyRow for vk_parse::Tag {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy => "name", "author", "contact"]
    }

    fn to_row(&self) -> prettytable::Row {
        row![self.name, self.author, self.contact]
    }
}

impl PrettyRow for vk_parse::TypesChild {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy =>
            "name", "alias", "api", "requires", "category", "comment", "parent",
            //"returnedonly", "structextends", "allowduplicate", "objtypeenum",
            //"bitvalues", "comment",
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        use vk_parse::TypesChild;

        match self {
            TypesChild::Type(ty) => row![
                format_unwrap_or!(ty.name, tos, "-"),
                format_unwrap_or!(ty.alias, tos, "-"),
                format_unwrap_or!(ty.api, tos, "-"),
                format_unwrap_or!(ty.requires, tos, "-"),
                format_unwrap_or!(ty.category, tos, "-"),
                format_unwrap_or!(ty.comment, tos, "-"),
                format_unwrap_or!(ty.parent, tos, "-"),
                format_unwrap_or!(ty.returnedonly, tos, "-"),
                format_unwrap_or!(ty.structextends, tos, "-"),
                format_unwrap_or!(ty.allowduplicate, tos, "-"),
                format_unwrap_or!(ty.objtypeenum, tos, "-"),
                format_unwrap_or!(ty.bitvalues, tos, "-"),
                // spec: TypeSpec TODO: implement a way to list type-spec.
                "-".to_string(),
            ],
            TypesChild::Comment(c) => row![
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                "-".to_string(),
                // spec: TypeSpec TODO: implement a way to list type-spec.
                c,
            ],
            val => panic!("non-exhaustive pattern for TypesChild {:?}", val),
        }
    }
}

impl PrettyRow for vk_parse::Extension {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![Fy =>
            "name", "number", "platform", "author", "requires", "requires_core",
            "deprecatedby", "children",
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.name,
            format_unwrap_or!(self.number, tos, "-"),
            format_unwrap_or!(self.platform, tos, "-"),
            format_unwrap_or!(self.author, tos, "-"),
            format_unwrap_or!(self.requires, tos, "-"),
            format_unwrap_or!(self.requires_core, tos, "-"),
            format_unwrap_or!(self.deprecatedby, tos, "-"),
            self.children
                .iter()
                .map(ext_child_to_string)
                .collect::<Vec<String>>()
                .join(",")
        ]
    }
}

#[inline]
pub fn tos<T: fmt::Display>(val: T) -> String {
    val.to_string()
}

#[inline]
fn tod<T: fmt::Debug>(val: T) -> String {
    format!("{:?}", val)
}

fn ext_child_to_string(c: &vk_parse::ExtensionChild) -> String {
    use vk_parse::ExtensionChild;

    match c {
        ExtensionChild::Require {
            api,
            feature,
            extension,
            ..
        } => format!("Req|{:?}|{:?}|{:?}", api, feature, extension),
        ExtensionChild::Remove { .. } => format!("Rem"),
        val => panic!("non-exhaustive pattern for ExtensionChild {:?}", val),
    }
}

pub fn transpose(mut table: Table) -> Table {
    let format = table.get_format().clone();

    let mut rows: Vec<Row> = vec![];
    for row in table.into_iter() {
        for (i, cell) in row.into_iter().enumerate() {
            if i < rows.len() {
                rows[i].add_cell(cell.clone())
            } else {
                rows.push(Row::new(vec![cell.clone()]));
            }
        }
    }

    let mut transpose = Table::init(rows);
    transpose.set_format(format);

    transpose
}
