use serde::Deserialize;

use crate::{Error, Result};

#[derive(Clone)]
pub struct Config {
    pub web: bool,
    pub adapter_options: AdapterOptions,
    pub winit: WinitOptions,
}

#[derive(Clone)]
pub struct AdapterOptions {
    pub power_preference: wgpu::PowerPreference,
    pub force_fallback_adapter: bool,
}

#[derive(Clone)]
pub struct WinitOptions {
    title: String,
    visible: bool,
    alway_on_top: bool,
    maximised: bool,
    minimised: bool,
    resizeable: bool,
    cursor_position: Option<Vec<i32>>,
    cursor_visible: bool,
    window_decorations: bool,
    inner_size: Option<Vec<u32>>,
    max_inner_size: Option<Vec<u32>>,
    min_inner_size: Option<Vec<u32>>,
    outer_position: Option<Vec<i32>>,
    // TODO: ime_position
    // TODO: cursor_icon
    // TODO: fullscreen
    // TODO: window_icon: Option<ffi::OsString>,
}

//-----

#[derive(Clone, Deserialize)]
struct TomlConfig {
    web: Option<bool>,
    adapter_options: Option<TomlAdapterOptions>,
    winit: Option<TomlWinitOptions>,
}

#[derive(Clone, Deserialize)]
pub struct TomlAdapterOptions {
    power_preference: Option<String>,
    force_fallback_adapter: Option<bool>,
}

#[derive(Clone)]
pub struct WinitWindowOptions {
    //
}

impl Default for Config {
    fn default() -> Config {
        let adapter_options = AdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        };
        Config {
            web: false,
            adapter_options: adapter_options,
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
}

fn power_preference(s: &str) -> Result<wgpu::PowerPreference> {
    let val = match s {
        "low_power" => wgpu::PowerPreference::LowPower,
        "high_performance" => wgpu::PowerPreference::HighPerformance,
        _ => err_at!(Invalid, msg: "invalid config.adapter_options.power_preference")?,
    };

    Ok(val)
}
