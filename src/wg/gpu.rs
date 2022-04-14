use raw_window_handle::HasRawWindowHandle;
use winit::dpi;
use log::{error};

#[allow(unused_imports)]
use crate::wg;
use crate::{Config, Error, Result, Windowing};

pub struct Gpu {
    pub name: String,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: wgpu::SurfaceConfiguration,
    pub size: dpi::PhysicalSize<u32>,
}

impl Gpu {
    /// * `win` abstracts a window instance.
    /// * `config` is configuration parameter for working with [wg] package.
    pub async fn new<W>(name: String, window: &W, config: Config) -> Result<Gpu>
    where
        W: Windowing + HasRawWindowHandle,
    {
        let size: dpi::PhysicalSize<u32> = window.inner_size().into();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&window) };
        let adapter = {
            let adapter_options = config.to_request_adapter_options(&surface);
            match instance.request_adapter(&adapter_options).await {
                Some(adapter) => adapter,
                None => err_at!(Wgpu, msg: "can't find matching adapter")?,
            }
        };

        let desc = wgpu::DeviceDescriptor {
            label: Some(&name),
            features: adapter.features(),
            limits: wgpu::Limits::default(),   // TODO: fetch from configuration
        };
        let (device, queue) = {
            let res = adapter.request_device(&desc, config.to_trace_path()).await;
            err_at!(Wgpu, res)?
        };
        device.on_uncaptured_error(uncaptured_error_handler);

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_DST,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: config.present_mode,
        };
        surface.configure(&device, &surface_config);

        let val = Gpu {
            name,
            surface,
            device,
            queue,
            surface_config,
            size,
        };

        Ok(val)
    }

    pub fn resize(&mut self, new_size: dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.surface_config.width = new_size.width;
            self.surface_config.height = new_size.height;
            self.surface.configure(&self.device, &self.surface_config);
        }
    }

    pub fn get_current_texture(&self) -> Result<wgpu::SurfaceTexture> {
        match self.surface.get_current_texture() {
            Ok(val) => Ok(val),
            // Reconfigure the surface if lost
            Err(wgpu::SurfaceError::Lost) => err_at!(SurfaceLost, msg: ""),
            // The system is out of memory, we should probably quit
            Err(wgpu::SurfaceError::OutOfMemory) => err_at!(SurfaceOutOfMemory, msg: ""),
            Err(wgpu::SurfaceError::Outdated) => err_at!(SurfaceOutdated, msg: ""),
            // TODO handle Timeout error in updating the frame-buffer.
            Err(err) => err_at!(Wgpu, Err(err)),
        }
    }

    pub fn clear_view<C>(
        &self,
        view: &wgpu::TextureView,
        color: C,
    ) -> wgpu::CommandBuffer
    where
        C: Into<wgpu::Color>,
    {
        let color: wgpu::Color = color.into();
        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("clear_screen"),
            };
            self.device.create_command_encoder(&desc)
        };
        {
            let ops = wgpu::Operations {
                load: wgpu::LoadOp::Clear(color),
                store: true,
            };
            let desc = wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: view,
                    resolve_target: None,
                    ops,
                }],
                depth_stencil_attachment: None,
            };
            encoder.begin_render_pass(&desc)
        };

        encoder.finish()
    }

    pub fn render(
        &self,
        cmd_buffers: Vec<wgpu::CommandBuffer>,
        surface_texture: wgpu::SurfaceTexture,
    ) -> Result<()> {
        self.queue.submit(cmd_buffers.into_iter());
        surface_texture.present();

        Ok(())
    }

    pub fn to_extent3d(&self) -> wgpu::Extent3d {
        let width = self.surface_config.width;
        let height = self.surface_config.height;
        let depth_or_array_layers = 1;
        wgpu::Extent3d { width, height, depth_or_array_layers }
    }
}

fn uncaptured_error_handler(err: wgpu::Error) {
    error!(target: "wg::Gpu", "uncaptured error: {}", err)
}
