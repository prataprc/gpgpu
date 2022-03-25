//! Package define configuration parameters for [wgpu] and [winit] libraries

use serde::Deserialize;
#[allow(unused_imports)]
use wgpu::{Instance, RequestAdapterOptions};

use std::{convert::TryInto, ffi, path};

#[allow(unused_imports)]
use crate::wg;
use crate::{niw, util, Error, Result};

// Local type that is friendly for converting from toml Value.
#[derive(Clone, Deserialize)]
struct TomlConfig {
    web: Option<bool>,
    trace_path: Option<ffi::OsString>,
    present_mode: Option<String>,
    adapter_options: Option<TomlAdapterConfig>,
}

// Local type that is friendly for converting from toml Value.
#[derive(Clone, Deserialize)]
struct TomlAdapterConfig {
    power_preference: Option<String>,
    force_fallback_adapter: Option<bool>,
}

//-----

/// Configuration type for everything under [wg] package.
#[derive(Clone)]
pub struct Config {
    /// Web only features are enabled in wgpu.
    pub web: bool,
    /// Path can be used for API call tracing, if that feature is enabled in wgpu-core.
    pub trace_path: Option<path::PathBuf>,
    /// Present mode configuration for window surface.
    pub present_mode: wgpu::PresentMode,
    /// Refer to [Config::to_request_adapter_options] for details.
    pub adapter_options: AdapterConfig,
    /// Refer to [niw::WinitConfig] for details
    pub winit: niw::WinitConfig,
}

/// Configuration for [RequestAdapterOptions].
#[derive(Clone)]
pub struct AdapterConfig {
    pub power_preference: wgpu::PowerPreference,
    pub force_fallback_adapter: bool,
}

impl Default for Config {
    fn default() -> Config {
        let adapter_options = AdapterConfig {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        };
        Config {
            web: false,
            trace_path: None,
            present_mode: wgpu::PresentMode::Fifo,
            adapter_options: adapter_options,
            winit: niw::WinitConfig::default(),
        }
    }
}

impl TryFrom<TomlConfig> for Config {
    type Error = Error;

    fn try_from(toml_config: TomlConfig) -> Result<Config> {
        let mut c = Config::default();

        if let Some(val) = toml_config.web.clone() {
            c.web = val;
        }
        c.trace_path = match toml_config.trace_path {
            Some(val) if val.len() > 0 => Some(val.into()),
            _ => c.trace_path,
        };
        c.present_mode = match toml_config.present_mode.as_ref().map(|s| s.as_str()) {
            Some("immediate") => wgpu::PresentMode::Immediate,
            Some("mailbox") => wgpu::PresentMode::Mailbox,
            Some("fifo") => wgpu::PresentMode::Fifo,
            Some(s) => err_at!(Invalid, msg: "present_mode {}", s)?,
            None => c.present_mode,
        };

        let adapter_options = toml_config.adapter_options.as_ref();
        if let Some(val) = (|| adapter_options?.power_preference.clone())() {
            c.adapter_options.power_preference = power_preference(&val)?;
        }
        if let Some(val) = (|| adapter_options?.force_fallback_adapter)() {
            c.adapter_options.force_fallback_adapter = val;
        }

        Ok(c)
    }
}

impl Config {
    /// Construct a new configuration from a file located by `loc`.
    pub fn from_file<P>(loc: P) -> Result<Config>
    where
        P: AsRef<path::Path>,
    {
        let mut value: toml::Value = util::load_toml(loc)?;
        let winit_config = match value.as_table_mut().unwrap().remove("winit") {
            Some(winit_value) => niw::WinitConfig::from_toml(winit_value)?,
            None => {
                let c = Config::default();
                c.winit
            }
        };

        let tc: TomlConfig = err_at!(FailConvert, toml::from_str(&value.to_string()))?;
        let mut conf: Config = tc.try_into()?;
        conf.winit = winit_config;

        Ok(conf)
    }

    /// Return [RequestAdapterOptions] that can be used to fetch a new compatible adapter,
    /// for `surface`, from wgpu [Instance].
    pub fn to_request_adapter_options<'a>(
        &self,
        surface: &'a wgpu::Surface,
    ) -> wgpu::RequestAdapterOptions<'a> {
        wgpu::RequestAdapterOptions {
            power_preference: self.adapter_options.power_preference,
            force_fallback_adapter: self.adapter_options.force_fallback_adapter,
            compatible_surface: Some(surface),
        }
    }

    /// Return window-attributes to instantiate a window instance, like [niw::SingleWindow].
    pub fn to_window_attributes(&self) -> Result<winit::window::WindowAttributes> {
        self.winit.clone().try_into()
    }

    /// Return the trace path for API call tracing, if that feature is enabled in wgpu-core.
    pub fn to_trace_path(&self) -> Option<&path::Path> {
        self.trace_path.as_ref().map(|x| x.as_path())
    }
}

fn power_preference(s: &str) -> Result<wgpu::PowerPreference> {
    let val = match s {
        "low_power" => wgpu::PowerPreference::LowPower,
        "high_performance" => wgpu::PowerPreference::HighPerformance,
        _ => err_at!(Invalid, msg: "invalid config.adapter_options.power_preference")?,
    };

    Ok(val)
}
