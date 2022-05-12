use log::{debug, error, trace};
use winit::dpi;

use std::{
    path,
    sync::{mpsc, Arc},
    thread,
};

use crate::{
    widg::load, ColorTarget, Context, Error, Result, SaveFile, Screen, Transforms,
    Viewport, Widget,
};

/// Rendering thread
pub struct Render {
    screen: Arc<Screen>,
    ssaa: f32,
    color_texture: Arc<wgpu::Texture>,
    color_format: wgpu::TextureFormat,
    save_file: Option<SaveFile>,
    handle: Option<thread::JoinHandle<Result<()>>>,
    tx: Option<mpsc::Sender<Request>>,
}

impl Drop for Render {
    fn drop(&mut self) {
        self.stop().ok();
        match self.save_file.as_mut() {
            Some(sf) => sf.save_to_file(),
            None => (),
        }
    }
}

impl Render {
    pub fn new_super_sampled(
        screen: Screen,
        ssaa: f32,
        color_format: wgpu::TextureFormat,
    ) -> Render {
        let size = {
            let mut size = screen.to_extent3d();
            size.width = size.width * ssaa as u32;
            size.height = size.height * ssaa as u32;
            size
        };
        let color_texture = Arc::new(screen.like_surface_texture(size, color_format));
        Render {
            screen: Arc::new(screen),
            ssaa,
            color_texture,
            color_format,
            save_file: None,
            handle: None,
            tx: None,
        }
    }

    pub fn new(screen: Screen, color_format: wgpu::TextureFormat) -> Render {
        Render::new_super_sampled(screen, crate::SCALE_FACTOR, color_format)
    }

    pub fn save_bmp<P>(&mut self, loc: P, format: wgpu::TextureFormat) -> &mut Self
    where
        P: AsRef<path::Path>,
    {
        let loc: path::PathBuf = {
            let loc: &path::Path = loc.as_ref();
            loc.into()
        };
        self.save_file = Some(SaveFile::new_bmp(
            loc,
            &self.screen.device,
            self.to_extent3d(),
            format,
        ));
        self
    }

    pub fn save_gif<P>(&mut self, loc: P, format: wgpu::TextureFormat) -> &mut Self
    where
        P: AsRef<path::Path>,
    {
        let loc: path::PathBuf = {
            let loc: &path::Path = loc.as_ref();
            loc.into()
        };
        self.save_file = Some(SaveFile::new_gif(
            loc,
            &self.screen.device,
            self.to_extent3d(),
            format,
        ));
        self
    }

    pub fn start(&mut self) {
        let screen = Arc::clone(&self.screen);
        let (tx, rx) = mpsc::channel();
        self.handle = Some(thread::spawn(|| render_loop(screen, rx)));
        self.tx = Some(tx)
    }

    pub fn stop(&mut self) -> Result<()> {
        match self.tx.take() {
            Some(tx) => std::mem::drop(tx),
            None => (),
        }

        match self.handle.take() {
            Some(handle) => handle.join().unwrap(),
            None => Ok(()),
        }
    }

    pub fn resize(
        &mut self,
        new_size: dpi::PhysicalSize<u32>,
        scale_factor: Option<f64>,
    ) {
        // first do this
        self.screen.resize(new_size, scale_factor);

        let size = self.to_extent3d();
        self.color_texture = {
            let texture = self.screen.like_surface_texture(size, self.color_format);
            Arc::new(texture)
        };
        self.save_file = match &self.save_file {
            Some(sf) => Some(sf.resize(&self.screen.device, size)),
            None => None,
        };
    }

    pub fn submit(&mut self, mut encoder: wgpu::CommandEncoder) -> Result<()> {
        match self.save_file.as_ref() {
            Some(sf) => sf.load_from_texture(
                &mut encoder,
                &self.screen.device,
                &self.color_texture,
            )?,
            None => (),
        }

        self.screen.queue.submit(vec![encoder.finish()]);

        match self.save_file.as_mut() {
            Some(sf) => sf.capture(&self.screen.device)?,
            None => (),
        }

        let frame = Arc::clone(&self.color_texture);
        match self.tx.as_ref() {
            Some(tx) => {
                let (resp_tx, rx) = mpsc::channel();
                let req = Request::Frame { frame, resp_tx };
                err_at!(IPCError, tx.send(req))?;
                err_at!(IPCError, rx.recv())?;
            }
            None => (),
        }

        Ok(())
    }
}

impl Render {
    pub fn as_screen(&self) -> Arc<Screen> {
        Arc::clone(&self.screen)
    }

    pub fn as_device(&self) -> &wgpu::Device {
        &self.screen.device
    }

    pub fn as_queue(&self) -> &wgpu::Queue {
        &self.screen.queue
    }

    pub fn to_scale_factor(&self) -> f32 {
        self.ssaa * (self.screen.to_scale_factor() as f32)
    }

    pub fn to_extent3d(&self) -> wgpu::Extent3d {
        let mut size = self.screen.to_extent3d();
        size.width = size.width * self.ssaa as u32;
        size.height = size.height * self.ssaa as u32;
        size
    }

    pub fn to_color_target(&self) -> ColorTarget {
        let view = {
            let desc = wgpu::TextureViewDescriptor::default();
            self.color_texture.create_view(&desc)
        };
        ColorTarget {
            format: self.color_format,
            view,
            view_port: Viewport::default(),
        }
    }
}

enum Request {
    Frame {
        frame: Arc<wgpu::Texture>,
        resp_tx: mpsc::Sender<bool>,
    },
}

fn render_loop(screen: Arc<Screen>, rx: mpsc::Receiver<Request>) -> Result<()> {
    let mut resp_txs: Vec<mpsc::Sender<bool>> = vec![];

    let mut surface_texture: Option<wgpu::SurfaceTexture> = None;
    let surface_format = screen.to_surface_config().format;
    let mut load = load::Load::new(&screen.device, surface_format)?;

    debug!("entering the render_loop ..");

    'outer: loop {
        surface_texture.map(|t| t.present());

        let (mut frame, disconnected) = 'inner: loop {
            let (frames, disconnected) = get_frames(&rx);
            trace!("frames:{} disconnected:{}", frames.len(), disconnected);
            match frames.into_iter().rev().next() {
                Some(frame) => break (frame, disconnected),
                None if disconnected => break 'outer,
                None => continue 'inner,
            }
        };

        surface_texture = match screen.get_current_texture() {
            Ok(texture) => Some(texture),
            Err(err) => {
                error!("error obtaning the surface texture{}", err);
                err_at!(Fatal, Err(err))?
            }
        };
        let surface_view = {
            let desc = wgpu::TextureViewDescriptor::default();
            surface_texture.as_ref().unwrap().texture.create_view(&desc)
        };

        frame.resp_txs.drain(..).for_each(|t| resp_txs.push(t));

        let mut encoder = {
            let desc = wgpu::CommandEncoderDescriptor {
                label: Some("render_loop:command-encoder"),
            };
            screen.device.create_command_encoder(&desc)
        };
        {
            let frame_view = frame
                .frame
                .create_view(&wgpu::TextureViewDescriptor::default());
            load.set_source(frame_view)
        };

        let context = Context {
            transforms: &Transforms::empty(),
            device: &screen.device,
            queue: &screen.queue,
        };
        let target = ColorTarget {
            format: surface_format,
            view: surface_view,
            // TODO let is be same as other dom elements, should we ?
            view_port: Viewport::default(),
        };
        load.render(&context, &mut encoder, &target)?;
        screen.queue.submit(vec![encoder.finish()]);

        //debug!("###########################");
        //let wait_queue = async { screen.queue.on_submitted_work_done().await };
        //pollster::block_on(wait_queue);
        //debug!("...........................");

        for tx in resp_txs.drain(..) {
            err_at!(IPCError, tx.send(true))?
        }

        if disconnected {
            break;
        }
    }

    debug!("exiting the render_loop ..");

    Ok(())
}

struct Frame {
    frame: Arc<wgpu::Texture>,
    resp_txs: Vec<mpsc::Sender<bool>>,
}

fn get_frames(rx: &mpsc::Receiver<Request>) -> (Vec<Frame>, bool) {
    let mut frames = vec![];
    loop {
        match rx.try_recv() {
            Ok(msg) => match msg {
                Request::Frame { frame, resp_tx } => {
                    let f = Frame {
                        frame,
                        resp_txs: vec![resp_tx],
                    };
                    frames.push(f);
                }
            },
            Err(mpsc::TryRecvError::Empty) => break (frames, false),
            Err(mpsc::TryRecvError::Disconnected) => break (frames, true),
        }
    }
}
