use std::sync::Arc;

use crate::{util, Result};

pub struct SaveConfig {
    pub texture: Arc<wgpu::Texture>,
    pub dimension: wgpu::Extent3d,
    pub format: wgpu::TextureFormat,
}

pub struct Save {
    buffer: wgpu::Buffer,
}

impl Save {
    pub fn new(device: &wgpu::Device, size: usize) -> Save {
        use wgpu::BufferUsages;

        let desc = wgpu::BufferDescriptor {
            label: Some("save-buffer"),
            size: size as wgpu::BufferAddress,
            usage: BufferUsages::MAP_READ,
            mapped_at_creation: false,
        };
        let buffer = device.create_buffer(&desc);

        Save { buffer }
    }

    pub fn save(
        &self,
        source: SaveConfig,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Result<Vec<u8>> {
        use std::num::NonZeroU32;

        let src = source.texture.as_image_copy();
        let (dimn, formt) = (source.dimension, source.format);
        let dst = wgpu::ImageCopyBuffer {
            buffer: &self.buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(util::bytes_per_row(dimn, formt)),
                rows_per_image: NonZeroU32::new(source.dimension.height),
            },
        };

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("widgets/save:command-encoder"),
            };
            device.create_command_encoder(&desc)
        };
        encoder.copy_texture_to_buffer(src, dst, dimn);

        let cmd_buffers = vec![encoder.finish()];
        queue.submit(cmd_buffers.into_iter());

        let data = self.buffer.slice(..).get_mapped_range().to_vec();

        Ok(data)
    }
}
