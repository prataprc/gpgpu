//! Package implement graphics as library on top of [wgpu] library.

mod backends;
mod gpu;
pub mod pretty;

pub use backends::{backend, backend_to_string, string_to_backend};
pub use gpu::Gpu;
