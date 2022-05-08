use log::{error, info};

use std::path;

use crate::{Error, Result};

enum Type {
    Bmp,
    Gif,
}

pub struct SaveFile {
    loc: path::PathBuf,
    typ: Type,
    size: wgpu::Extent3d,
    format: wgpu::TextureFormat,
    buffer: wgpu::Buffer,
    unpadded_bytes_per_row: u32,
    padded_bytes_per_row: u32,
    frames: Vec<Vec<u8>>,
}

impl SaveFile {
    pub fn new_gif(
        loc: path::PathBuf,
        device: &wgpu::Device,
        size: wgpu::Extent3d,
        format: wgpu::TextureFormat,
    ) -> SaveFile {
        let mut val = Self::new_bmp(loc, device, size, format);
        val.typ = Type::Gif;
        val.frames = Vec::with_capacity(8);
        val
    }

    pub fn new_bmp(
        loc: path::PathBuf,
        device: &wgpu::Device,
        size: wgpu::Extent3d,
        format: wgpu::TextureFormat,
    ) -> SaveFile {
        use wgpu::BufferUsages;

        let texel_size = Self::texel_size(format);
        let align = wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
        let unpadded_bytes_per_row = texel_size * size.width;
        let padding = (align - (unpadded_bytes_per_row % align)) % align;
        let padded_bytes_per_row = unpadded_bytes_per_row + padding;
        let size_bytes = padded_bytes_per_row * size.height;

        let desc = wgpu::BufferDescriptor {
            label: Some("save-buffer"),
            size: size_bytes as wgpu::BufferAddress,
            usage: BufferUsages::COPY_DST | BufferUsages::MAP_READ,
            mapped_at_creation: false,
        };
        let buffer = device.create_buffer(&desc);

        SaveFile {
            loc,
            typ: Type::Bmp,
            size,
            format,
            buffer,
            unpadded_bytes_per_row,
            padded_bytes_per_row,
            frames: Vec::with_capacity(1),
        }
    }

    pub fn resize(&self, device: &wgpu::Device, size: wgpu::Extent3d) -> SaveFile {
        let loc = self.loc.clone();
        match self.typ {
            Type::Bmp => SaveFile::new_bmp(loc, device, size, self.format),
            Type::Gif => SaveFile::new_gif(loc, device, size, self.format),
        }
    }

    pub fn load_from_texture(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        _device: &wgpu::Device,
        texture: &wgpu::Texture,
    ) -> Result<()> {
        use std::num::NonZeroU32;

        let src = texture.as_image_copy();
        let dst = wgpu::ImageCopyBuffer {
            buffer: &self.buffer,
            layout: wgpu::ImageDataLayout {
                offset: 0,
                bytes_per_row: NonZeroU32::new(self.padded_bytes_per_row),
                rows_per_image: NonZeroU32::new(self.size.height),
            },
        };
        encoder.copy_texture_to_buffer(src, dst, self.size);

        Ok(())
    }

    pub fn capture(&mut self, device: &wgpu::Device) -> Result<()> {
        let slice = self.buffer.slice(..);

        let request = slice.map_async(wgpu::MapMode::Read);
        device.poll(wgpu::Maintain::Wait); // wait for the GPU to finish
        err_at!(Fatal, pollster::block_on(request))?;

        let data = {
            let texel_size = Self::texel_size(self.format);
            let size = texel_size * self.size.width * self.size.height;

            let mut data = Vec::with_capacity(size as usize);
            let padded_data = slice.get_mapped_range().to_vec();
            padded_data
                .chunks(self.padded_bytes_per_row as _)
                .for_each(|chunk| {
                    data.extend_from_slice(&chunk[..self.unpadded_bytes_per_row as _])
                });
            data
        };

        self.buffer.unmap();
        self.frames.push(data);
        if self.frames.capacity() < 4 && self.frames.len() > 1 {
            self.frames.remove(0);
        }

        Ok(())
    }

    fn texel_size(format: wgpu::TextureFormat) -> u32 {
        match format {
            wgpu::TextureFormat::Rgba8Unorm => 4,
            wgpu::TextureFormat::Rgba8Uint => 4,
            wgpu::TextureFormat::Rgba8UnormSrgb => 4,
            wgpu::TextureFormat::Bgra8UnormSrgb => 4,
            val => panic!("format {:?} can't be handled for widg/save", val),
        }
    }
}

impl SaveFile {
    pub fn save_to_file(&mut self) {
        match self.typ {
            Type::Bmp => self.save_to_bmp(),
            Type::Gif => self.save_to_gif(),
        }
    }

    fn save_to_bmp(&mut self) {
        match self.frames.pop() {
            Some(frame) => {
                let imgbuf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> =
                    image::ImageBuffer::from_vec(
                        self.size.width,
                        self.size.height,
                        frame,
                    )
                    .unwrap();

                match imgbuf.save(&self.loc) {
                    Ok(_) => info!("Saving to bitmap file, {:?}", self.loc),
                    Err(err) => {
                        error!("Fail saving to bitmap file, {:?}, {}", self.loc, err)
                    }
                }
            }
            None => error!("No frames to save to bitmap"),
        }
    }

    fn save_to_gif(&mut self) {
        use gif::{Encoder, Frame, Repeat};

        let wgpu::Extent3d { width, height, .. } = self.size.clone();
        let (width, height) = (width as u16, height as u16);

        let mut image = match std::fs::File::create(&self.loc) {
            Ok(f) => f,
            Err(err) => {
                error!("Fail creating Gif file {:?}, {}", self.loc, err);
                return;
            }
        };
        let mut encoder = match Encoder::new(&mut image, width, height, &[]) {
            Ok(encoder) => encoder,
            Err(err) => {
                error!("Fail creating Gif encoder {:?} {}", self.loc, err);
                return;
            }
        };
        match encoder.set_repeat(Repeat::Infinite) {
            Ok(()) => (),
            Err(err) => {
                error!("Fail Gif encoder::set_repeat {}", err);
                return;
            }
        }

        for mut frame in self.frames.drain(..) {
            let frame = Frame::from_rgba_speed(width, height, &mut frame, 30);
            match encoder.write_frame(&frame) {
                Ok(()) => (),
                Err(err) => {
                    error!("Fail writing to Gif frame {}", err);
                    return;
                }
            }
        }

        info!("Saving to gif file, {:?}", self.loc);
    }
}
