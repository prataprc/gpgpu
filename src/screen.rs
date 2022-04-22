use winit::{window::Window, dpi};
use log::{error, debug, warn};

use std::sync::Arc;

use crate::{Config, Error, Result, spinlock::Spinlock};

pub struct Screen {
    pub name: String,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface_config: Spinlock<Arc<wgpu::SurfaceConfiguration>>,
}

impl Screen {
    /// * `win` abstracts a window instance.
    /// * `config` is configuration parameter for working with this crate.
    pub async fn new(name: String, win: &Window, config: Config) -> Result<Screen> {
        let size: dpi::PhysicalSize<u32> = win.inner_size().into();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&win) };
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
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: config.present_mode,
        };
        surface.configure(&device, &surface_config);

        let val = Screen {
            name,
            surface,
            device,
            queue,
            surface_config: Spinlock::new(Arc::new(surface_config)),
        };

        Ok(val)
    }

    pub fn as_surface_config(&self) -> Arc<wgpu::SurfaceConfiguration> {
        Arc::clone(&self.surface_config.read())
    }

    pub fn resize(&self, new_size: dpi::PhysicalSize<u32>) {
        if new_size.width <= 0 && new_size.height <= 0 {
            warn!("screen-resize {:?}", new_size);
            return
        }

        debug!("screen-resize {:?}", new_size);

        let surface_config = wgpu::SurfaceConfiguration {
                width: new_size.width,
                height: new_size.height,
                ..Arc::clone(&self.surface_config.read()).as_ref().clone()
        };
        self.surface.configure(&self.device, &surface_config);
        *self.surface_config.write() = Arc::new(surface_config);
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

    pub fn to_extent3d(&self) -> wgpu::Extent3d {
        let width = self.as_surface_config().width;
        let height = self.as_surface_config().height;
        let depth_or_array_layers = 1;
        wgpu::Extent3d { width, height, depth_or_array_layers }
    }

    pub fn like_surface_texture(&self) -> wgpu::Texture {
        let desc = wgpu::TextureDescriptor {
            label: Some("like-surface-texture"),
            size: self.to_extent3d(),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: self.to_texture_format(),
            usage: wgpu::TextureUsages::TEXTURE_BINDING | wgpu::TextureUsages::RENDER_ATTACHMENT,
        };
        self.device.create_texture(&desc)
    }

    // width / height of the surface
    pub fn to_aspect_ratio(&self) -> f32 {
        let sc = self.as_surface_config();
        (sc.width as f32) / (sc.height as f32)
    }

    pub fn to_texture_format(&self) -> wgpu::TextureFormat {
        self.as_surface_config().format
    }

    pub fn to_physical_size(&self) -> dpi::PhysicalSize<u32> {
        let sc = self.as_surface_config();
        dpi::PhysicalSize {
            width: sc.width,
            height: sc.height,
        }
    }
}

fn uncaptured_error_handler(err: wgpu::Error) {
    error!("uncaptured error: {}", err)
}
