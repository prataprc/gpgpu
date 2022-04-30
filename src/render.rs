use log::{debug, error, trace};

use std::{
    sync::{mpsc, Arc},
    thread,
};

use crate::{
    widg::{self, Load, Widget},
    Error, Result, Screen, Transforms,
};

/// Rendering thread
pub struct Render {
    screen: Arc<Screen>,
    handle: Option<thread::JoinHandle<Result<()>>>,
    tx: Option<mpsc::Sender<Request>>,
}

impl Drop for Render {
    fn drop(&mut self) {
        self.stop().ok();
    }
}

impl Render {
    pub fn new(screen: Screen) -> Render {
        Render {
            screen: Arc::new(screen),
            handle: None,
            tx: None,
        }
    }

    pub fn as_screen(&self) -> Arc<Screen> {
        Arc::clone(&self.screen)
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

    pub fn post_frame(&self, frame: Arc<wgpu::Texture>) -> Result<()> {
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
    let mut load = Load::new(&screen.device, surface_format)?;

    debug!("starting the render_loop ..");

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

        let context = widg::Context {
            transforms: &Transforms::empty(),
            device: &screen.device,
            queue: &screen.queue,
        };
        let target = widg::ColorTarget {
            size: screen.to_extent3d(1),
            format: screen.to_texture_format(),
            view: &surface_view,
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

    debug!("stopping the render_loop ..");

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
