use colored::Colorize;
use prettytable::{cell, row};
use vulkano::instance::LayerProperties;

use crate::{Error, Result};

macro_rules! format_bool {
    ($val:expr) => {
        if $val {
            "✓".green()
        } else {
            "✗".red()
        }
    };
}

pub struct QueuePipelineStage {
    stage: vulkano::sync::PipelineStage,
    supported: Vec<bool>,
}

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

pub fn check_layer_names(layers: Vec<String>) -> Result<Vec<String>> {
    let available = layer_names()?;
    Ok(layers
        .into_iter()
        .filter(|l| available.contains(l))
        .collect())
}

pub fn queue_pipeline_stages(
    phydev: &vulkano::device::physical::PhysicalDevice,
) -> Vec<QueuePipelineStage> {
    use vulkano::sync::PipelineStage;

    let stages = vec![
        PipelineStage::TopOfPipe,
        PipelineStage::DrawIndirect,
        PipelineStage::VertexInput,
        PipelineStage::VertexShader,
        PipelineStage::TessellationControlShader,
        PipelineStage::TessellationEvaluationShader,
        PipelineStage::GeometryShader,
        PipelineStage::FragmentShader,
        PipelineStage::EarlyFragmentTests,
        PipelineStage::LateFragmentTests,
        PipelineStage::ColorAttachmentOutput,
        PipelineStage::ComputeShader,
        PipelineStage::Transfer,
        PipelineStage::BottomOfPipe,
        PipelineStage::Host,
        PipelineStage::AllGraphics,
        PipelineStage::AllCommands,
        PipelineStage::RayTracingShader,
    ];

    stages
        .into_iter()
        .map(|stage| QueuePipelineStage {
            stage: stage.clone(),
            supported: phydev
                .queue_families()
                .collect::<Vec<vulkano::device::physical::QueueFamily>>()
                .iter()
                .map(|qf| qf.supports_stage(stage))
                .collect(),
        })
        .collect()
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

impl<'a> PrettyRow for vulkano::device::physical::MemoryHeap<'a> {
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
            format_bool!(self.is_device_local()),
            format_bool!(self.is_multi_instance())
        ]
    }
}

impl<'a> PrettyRow for vulkano::device::physical::MemoryType<'a> {
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
            format_bool!(self.is_device_local()),
            format_bool!(self.is_host_visible()),
            format_bool!(self.is_host_cached()),
            format_bool!(self.is_host_coherent()),
            format_bool!(self.is_lazily_allocated()),
        ]
    }
}

impl<'a> PrettyRow for vulkano::device::physical::QueueFamily<'a> {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![
            Fy => "QueueId", "Count", "ImageTxGranularity", "Graphics", "Compute",
             "XTransfer", "Sparse",
        ]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            self.id(),
            self.queues_count(),
            format!("{:?}", self.min_image_transfer_granularity()),
            format_bool!(self.supports_graphics()),
            format_bool!(self.supports_compute()),
            format_bool!(self.explicitly_supports_transfers()),
            format_bool!(self.supports_sparse_binding()),
        ]
    }
}

impl<'a> PrettyRow for QueuePipelineStage {
    fn to_format() -> prettytable::format::TableFormat {
        *prettytable::format::consts::FORMAT_CLEAN
    }

    fn to_head() -> prettytable::Row {
        row![ Fy => "Stage", "QueueFamily" ]
    }

    fn to_row(&self) -> prettytable::Row {
        row![
            format!("{:?}", self.stage),
            self.supported
                .iter()
                .map(|s| format!("{}", format_bool!(*s)))
                .collect::<Vec<String>>()
                .join("")
        ]
    }
}
