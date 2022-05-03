use winit::{window::Window, dpi};
use log::{error, warn, info};

use std::{sync::Arc};

use crate::{Config, Error, Result, util::Spinlock};

pub struct Screen {
    pub name: String,
    pub surface: wgpu::Surface,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    state: Spinlock<Arc<State>>,
}

struct State {
    surface_config: wgpu::SurfaceConfiguration,
    scale_factor: f64,
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
        let surface_format = surface.get_preferred_format(&adapter).unwrap();

        info!(
            "Surface created with size {}x{} format {:?}",
            size.width, size.height, surface_format,
        );

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
            format: surface_format,
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
            state: Spinlock::new(Arc::new(State {
                surface_config,
                scale_factor: win.scale_factor(),
            })),
        };

        Ok(val)
    }

    pub fn resize(&self, new_size: dpi::PhysicalSize<u32>, scale_factor: Option<f64>) {
        if new_size.width <= 0 && new_size.height <= 0 {
            warn!("screen-resize {:?}", new_size);
            return
        }
        info!("screen-resize {:?}", new_size);
        match scale_factor {
            Some(scale) => info!("scale_factor: {}", scale),
            None => (),
        }

        let (sc, scale_factor) = {
            let s = self.state.read();
            (s.surface_config.clone(), scale_factor.unwrap_or(s.scale_factor))
        };


        let surface_config = wgpu::SurfaceConfiguration {
            width: new_size.width,
            height: new_size.height,
            ..sc
        };
        self.surface.configure(&self.device, &surface_config);

        let state = State {
            surface_config,
            scale_factor,
        };

        *self.state.write() = Arc::new(state);
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

    pub fn to_surface_config(&self) -> wgpu::SurfaceConfiguration {
        self.state.read().surface_config.clone()
    }

    pub fn to_scale_factor(&self) -> f64 {
        self.state.read().scale_factor
    }

    pub fn to_extent3d(&self, ssaa: u32) -> wgpu::Extent3d {
        let sc = self.to_surface_config();
        let width = sc.width * ssaa;
        let height = sc.height * ssaa;
        let depth_or_array_layers = 1;
        wgpu::Extent3d { width, height, depth_or_array_layers }
    }

    pub fn to_center(&self, ssaa: u32) -> wgpu::Origin3d {
        let sc = self.to_surface_config();
        wgpu::Origin3d {
            x: (sc.width / 2) * ssaa ,
            y: (sc.height / 2) * ssaa ,
            z: 0,
        }
    }

    // width / height of the surface
    pub fn to_aspect_ratio(&self) -> f32 {
        let sc = self.to_surface_config();
        (sc.width as f32) / (sc.height as f32)
    }

    pub fn to_texture_format(&self) -> wgpu::TextureFormat {
        self.to_surface_config().format
    }

    pub fn to_physical_size(&self) -> dpi::PhysicalSize<u32> {
        let sc = self.to_surface_config();
        dpi::PhysicalSize {
            width: sc.width,
            height: sc.height,
        }
    }

    pub fn like_surface_texture(
        &self,
        ssaa: f32,
        format: Option<wgpu::TextureFormat>,
    ) -> wgpu::Texture {
        use wgpu::TextureUsages;

        let format = format.unwrap_or_else(|| self.to_texture_format());
        let desc = wgpu::TextureDescriptor {
            label: Some("like-surface-texture"),
            size: self.to_extent3d(ssaa as u32),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage: {
                TextureUsages::COPY_SRC
                | TextureUsages::TEXTURE_BINDING
                | TextureUsages::RENDER_ATTACHMENT
            },
        };
        self.device.create_texture(&desc)
    }
}

fn uncaptured_error_handler(err: wgpu::Error) {
    error!("uncaptured error: {}", err)
}
