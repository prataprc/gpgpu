use raw_window_handle::HasRawWindowHandle;
use winit::dpi;

use crate::{wg, AppWindow, Windowing, Result, Error};

pub struct Gpu {
    name: String,
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    surface_config: wgpu::SurfaceConfiguration,
    size: dpi::PhysicalSize<u32>,
}

impl Gpu {
    /// * `win` abstracts one or more application window and optionally an event-loop
    /// * `config` is configuration parameter for working with [wg] package.
    pub async fn new<N, W>(
        name: String,
        win: N,
        config: wg::Config
    ) -> Result<Gpu>
    where
        N: AppWindow<W>,
        W: Windowing + HasRawWindowHandle,
    {
        let window = win.as_window();
        let size: dpi::PhysicalSize<u32>  = window.inner_size().into();

        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = {
            let adapter_options = config.to_request_adapter_options(&surface);
            match instance.request_adapter(&adapter_options).await {
                Some(adapter) => adapter,
                None => err_at!(Wgpu, msg: "can't find matching adapter")?,
            }
        };

        let desc =wgpu::DeviceDescriptor {
            label: Some(&name),
            features: wgpu::Features::empty(), // TODO: fetch from configuration
            limits: wgpu::Limits::default(), // TODO: fetch from configuration
        };
        let (device, queue) = {
            let res = adapter.request_device(&desc, config.to_trace_path()).await;
            err_at!(Wgpu, res)?
        };

        let surface_config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
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
}
