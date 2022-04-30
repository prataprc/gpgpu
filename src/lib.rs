//! General Purpose GPU.

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

mod backends;
mod config;
//mod render;
mod save;
mod screen;
mod spinlock;
mod transforms;

pub mod dom;
pub mod fonts;
pub mod niw;
pub mod pretty;
pub mod util;
// pub mod widg;

pub use backends::{backend, backend_to_string};
pub use config::{Config, ConfigAdapter, ConfigWinit};
//pub use render::Render;
pub use save::SaveFile;
pub use screen::Screen;
pub use spinlock::Spinlock;
pub use transforms::{Camera, Ortho, Perspective, Transforms};

pub struct ViewPort {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub min_depth: f32,
    pub max_depth: f32,
}
