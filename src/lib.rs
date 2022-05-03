//! General Purpose GPU.

use bytemuck::{Pod, Zeroable};

use std::{error, fmt, result};

/// Short form to compose Error values.
///
/// Here are few possible ways:
///
/// ```ignore
/// use crate::Error;
/// err_at!(ParseError, msg: format!("bad argument"));
/// ```
///
/// ```ignore
/// use crate::Error;
/// err_at!(ParseError, std::io::read(buf));
/// ```
///
/// ```ignore
/// use crate::Error;
/// err_at!(ParseError, std::fs::read(file_path), format!("read failed"));
/// ```
#[macro_export]
macro_rules! err_at {
    ($v:ident, msg: $($arg:expr),+) => {{
        let prefix = format!("{}:{}", file!(), line!());
        Err(Error::$v(prefix, format!($($arg),+)))
    }};
    ($v:ident, $e:expr) => {{
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                Err(Error::$v(prefix, format!("{}", err)))
            }
        }
    }};
    ($v:ident, $e:expr, $($arg:expr),+) => {{
        match $e {
            Ok(val) => Ok(val),
            Err(err) => {
                let prefix = format!("{}:{}", file!(), line!());
                let msg = format!($($arg),+);
                Err(Error::$v(prefix, format!("{} {}", err, msg)))
            }
        }
    }};
}

/// Error variants that are returned by this package's API.
///
/// Each variant carries a prefix, typically identifying the
/// error location.
pub enum Error {
    Fatal(String, String),
    Invalid(String, String),
    FailConvert(String, String),
    IOError(String, String),
    IPCError(String, String),
    Vk(String, String),
    Wgpu(String, String),
    SurfaceLost(String, String),
    SurfaceOutOfMemory(String, String),
    SurfaceOutdated(String, String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        use Error::*;

        match self {
            Fatal(p, msg) => write!(f, "{} Fatal: {}", p, msg),
            Invalid(p, msg) => write!(f, "{} Invalid: {}", p, msg),
            FailConvert(p, msg) => write!(f, "{} FailConvert: {}", p, msg),
            IOError(p, msg) => write!(f, "{} IOError: {}", p, msg),
            IPCError(p, msg) => write!(f, "{} IPCError: {}", p, msg),
            Vk(p, msg) => write!(f, "{} Vk: {}", p, msg),
            Wgpu(p, msg) => write!(f, "{} Wgpu: {}", p, msg),
            SurfaceLost(p, msg) => write!(f, "{} SurfaceLost: {}", p, msg),
            SurfaceOutOfMemory(p, msg) => write!(f, "{} SurfaceOutOfMemory: {}", p, msg),
            SurfaceOutdated(p, msg) => write!(f, "{} SurfaceOutdated: {}", p, msg),
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> result::Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}

impl error::Error for Error {}

/// Type alias for Result return type, used by this package.
pub type Result<T> = result::Result<T, Error>;

mod config;
mod render;
mod save;
mod screen;
mod transforms;

pub mod dom;
pub mod fonts;
pub mod niw;
pub mod pretty;
pub mod util;
pub mod widg;

pub use config::{Config, ConfigAdapter, ConfigWinit};
pub use render::Render;
pub use save::SaveFile;
pub use screen::Screen;
pub use transforms::{Camera, Ortho, Perspective, Transforms};
pub use util::*;

use wgpu::Backend;

#[cfg(target_os = "macos")]
pub fn backend() -> Backend {
    Backend::Metal
}

#[cfg(target_os = "linux")]
pub fn backend() -> Backend {
    Backend::Vulkan
}

pub fn backend_to_string(backend: Backend) -> String {
    let s = match backend {
        Backend::Empty => "empty",
        Backend::Vulkan => "vulkan",
        Backend::Metal => "metal",
        Backend::Dx12 => "directx12",
        Backend::Dx11 => "directx11",
        Backend::Gl => "opengl",
        Backend::BrowserWebGpu => "web",
    };

    s.to_string()
}

#[derive(Clone, Copy, Default)]
pub struct BoxLayout {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl From<stretch::result::Layout> for BoxLayout {
    fn from(val: stretch::result::Layout) -> BoxLayout {
        let stretch::result::Layout {
            size: stretch::geometry::Size { width, height },
            location: stretch::geometry::Point { x, y },
            ..
        } = val;

        BoxLayout {
            x,
            y,
            w: width,
            h: height,
        }
    }
}

impl BoxLayout {
    pub fn to_vertices(&self, size: wgpu::Extent3d) -> Vec<BoxVertex> {
        let tl = [
            ((self.x / (size.width as f32)) * 2.0) - 1.0,
            1.0 - ((self.y / (size.height as f32)) * 2.0),
            0.0,
            1.0,
        ];
        let tr = [
            (((self.x + self.w) / (size.width as f32)) * 2.0) - 1.0,
            1.0 - ((self.y / (size.height as f32)) * 2.0),
            0.0,
            1.0,
        ];
        let br = [
            (((self.x + self.w) / (size.width as f32)) * 2.0) - 1.0,
            1.0 - (((self.y + self.h) / (size.height as f32)) * 2.0),
            0.0,
            1.0,
        ];
        let bl = [
            ((self.x / (size.width as f32)) * 2.0) - 1.0,
            1.0 - (((self.y + self.h) / (size.height as f32)) * 2.0),
            0.0,
            1.0,
        ];
        vec![
            BoxVertex { position: tl },
            BoxVertex { position: bl },
            BoxVertex { position: tr },
            BoxVertex { position: tr },
            BoxVertex { position: bl },
            BoxVertex { position: br },
        ]
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct BoxVertex {
    position: [f32; 4],
}

impl BoxVertex {
    const ATTRIBUTES: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
        0 => Float32x4,
    ];
}

impl BoxVertex {
    pub fn to_vertex_buffer_layout<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<BoxVertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBUTES,
        }
    }
}

// of screen coordinate, in pixels
#[derive(Clone, Copy, Debug)]
pub struct Location {
    pub x: f32,
    pub y: f32,
}

// of screen coordinate, in pixels
#[derive(Clone, Copy, Debug)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
