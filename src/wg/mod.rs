mod backends;
pub mod config;
mod features;
mod limits;
mod pretty;
mod texture_formats;

pub use backends::{backend, backend_to_string, string_to_backend};
pub use features::{add_adapter_to_features, features, Feature};
pub use limits::{add_adapter_to_limits, limits, Limit};
pub use pretty::StorageReport;
pub use texture_formats::{
    texture_format_flags, texture_formats_info, texture_usages, TextureFormatInfo,
};
