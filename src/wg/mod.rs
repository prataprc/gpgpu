//! Package implement graphics as library on top of [wgpu] library.

mod backends;
pub mod config;
pub mod pretty;
mod state;

pub use backends::{backend, backend_to_string, string_to_backend};
pub use config::{AdapterConfig, Config};
pub use state::State;
