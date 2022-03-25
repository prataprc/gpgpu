//! Package define configuration parameters for [wgpu] and [winit] libraries

use serde::Deserialize;

use std::{convert::TryInto, path};

use crate::{niw, util, Error, Result};

#[derive(Clone, Deserialize)]
struct TomlConfig {
    web: Option<bool>,
    adapter_options: Option<TomlAdapterConfig>,
}

#[derive(Clone, Deserialize)]
struct TomlAdapterConfig {
    power_preference: Option<String>,
    force_fallback_adapter: Option<bool>,
}

//-----

#[derive(Clone)]
pub struct Config {
    pub web: bool,
    pub adapter_options: AdapterConfig,
    pub winit: niw::WinitConfig,
}

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

    pub fn to_window_attributes(&self) -> Result<winit::window::WindowAttributes> {
        self.winit.to_window_attributes()
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
