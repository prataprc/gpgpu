//! Package define configuration parameters for this crate.
//!
//! Also refer to [wgpu] and [winit] libraries on how the configraion parameters map.

#[allow(unused_imports)]
use wgpu::{Instance, RequestAdapterOptions};
#[allow(unused_imports)]
use winit::window::{Window, WindowAttributes};

use serde::Deserialize;
use winit::{dpi, window};

use std::{convert::TryInto, ffi, path};

#[allow(unused_imports)]
use crate::niw;
use crate::{util, Error, Result};

// Local type that is friendly for converting from toml Value.
#[derive(Clone, Deserialize)]
struct TomlConfig {
    web: Option<bool>,
    trace_path: Option<ffi::OsString>,
    present_mode: Option<String>,
    adapter_options: Option<TomlConfigAdapter>,
}

// Local type that is friendly for converting from toml Value.
#[derive(Clone, Deserialize)]
struct TomlConfigAdapter {
    power_preference: Option<String>,
    force_fallback_adapter: Option<bool>,
}

//-----

/// Configuration type for initializing gpgpu crate.
#[derive(Clone)]
pub struct Config {
    /// Web only features are enabled in wgpu.
    pub web: bool,
    /// Path can be used for API call tracing, if that feature is enabled in wgpu-core.
    pub trace_path: Option<path::PathBuf>,
    /// Present mode configuration for window surface.
    pub present_mode: wgpu::PresentMode,
    /// Refer to [Config::to_request_adapter_options] for details.
    pub adapter_options: ConfigAdapter,
    /// Refer to [ConfigWinit] for details
    pub winit: ConfigWinit,
}

/// Configuration for [RequestAdapterOptions].
#[derive(Clone)]
pub struct ConfigAdapter {
    pub power_preference: wgpu::PowerPreference,
    pub force_fallback_adapter: bool,
}

impl Default for Config {
    fn default() -> Config {
        let adapter_options = ConfigAdapter {
            power_preference: wgpu::PowerPreference::HighPerformance,
            force_fallback_adapter: false,
        };
        Config {
            web: false,
            trace_path: None,
            present_mode: wgpu::PresentMode::Fifo,
            adapter_options: adapter_options,
            winit: ConfigWinit::default(),
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
            Some(winit_value) => ConfigWinit::from_toml(winit_value)?,
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

/// Configuration options for creating winit [Window].
///
/// ConfigWinit can be initialized programatically or via toml configuration file.
/// For the later case, refer to [ConfigWinit::from_toml] constructor. Subsequently
/// [ConfigWinit] can be converted to [WindowAttributes], via TryFrom/TryInto trait,
/// to create a winit window with desired attributes. Get Started with
/// `ConfigWinit::default()`
#[derive(Clone, Debug)]
pub struct ConfigWinit {
    pub title: String,
    pub visible: bool,
    pub transparent: bool,
    pub always_on_top: bool,
    pub maximized: bool,
    pub minimised: bool,
    pub resizable: bool,
    pub cursor_position: Option<Vec<f64>>,
    pub cursor_visible: bool,
    pub decorations: bool,
    pub inner_size: Option<Vec<f64>>,
    pub max_inner_size: Option<Vec<f64>>,
    pub min_inner_size: Option<Vec<f64>>,
    pub position: Option<Vec<f64>>,
    // TODO: ime_position
    // TODO: cursor_icon
    // TODO: fullscreen
    // TODO: window_icon: Option<ffi::OsString>,
}

// local type friendly to toml text/value.
#[derive(Clone, Deserialize)]
struct TomlConfigWinit {
    title: Option<String>,
    visible: Option<bool>,
    transparent: Option<bool>,
    always_on_top: Option<bool>,
    maximized: Option<bool>,
    minimised: Option<bool>,
    resizable: Option<bool>,
    cursor_position: Option<Vec<f64>>,
    cursor_visible: Option<bool>,
    decorations: Option<bool>,
    inner_size: Option<Vec<f64>>,
    max_inner_size: Option<Vec<f64>>,
    min_inner_size: Option<Vec<f64>>,
    position: Option<Vec<f64>>,
    // TODO: ime_position
    // TODO: cursor_icon
    // TODO: fullscreen
    // TODO: window_icon: Option<ffi::OsString>,
}

impl Default for ConfigWinit {
    fn default() -> ConfigWinit {
        ConfigWinit {
            title: "gpgpu".to_string(),
            visible: true,
            transparent: false,
            always_on_top: false,
            maximized: false,
            minimised: false,
            resizable: true,
            cursor_position: None,
            cursor_visible: true,
            decorations: true,
            #[cfg(all(unix, not(target_os = "macos")))]
            inner_size: Some(vec![800.0, 600.0]),
            #[cfg(any(target_os = "android", target_os = "macos"))]
            inner_size: None,
            max_inner_size: None,
            min_inner_size: None,
            position: None,
        }
    }
}

macro_rules! from_toml {
    ($src:ident, $field:ident, $default:ident) => {
        match $src.$field {
            Some(val) => val,
            None => $default.$field,
        }
    };
    (opt, $src:ident, $field:ident, $default:ident) => {
        match $src.$field {
            Some(val) => Some(val),
            None => $default.$field,
        }
    };
}

impl From<TomlConfigWinit> for ConfigWinit {
    fn from(toml_config: TomlConfigWinit) -> ConfigWinit {
        let c = ConfigWinit::default();
        ConfigWinit {
            title: from_toml!(toml_config, title, c),
            visible: from_toml!(toml_config, visible, c),
            transparent: from_toml!(toml_config, transparent, c),
            always_on_top: from_toml!(toml_config, always_on_top, c),
            maximized: from_toml!(toml_config, maximized, c),
            minimised: from_toml!(toml_config, minimised, c),
            resizable: from_toml!(toml_config, resizable, c),
            cursor_position: from_toml!(opt, toml_config, cursor_position, c),
            cursor_visible: from_toml!(toml_config, cursor_visible, c),
            decorations: from_toml!(toml_config, decorations, c),
            inner_size: from_toml!(opt, toml_config, inner_size, c),
            max_inner_size: from_toml!(opt, toml_config, max_inner_size, c),
            min_inner_size: from_toml!(opt, toml_config, min_inner_size, c),
            position: from_toml!(opt, toml_config, position, c),
        }
    }
}

impl TryFrom<ConfigWinit> for window::WindowAttributes {
    type Error = Error;

    fn try_from(config: ConfigWinit) -> Result<Self> {
        let val = window::WindowAttributes {
            inner_size: config.to_inner_size()?,
            min_inner_size: config.to_min_inner_size()?,
            max_inner_size: config.to_max_inner_size()?,
            position: config.to_position()?,
            resizable: config.resizable,
            fullscreen: None, // TODO fetch from config,
            title: config.title.clone(),
            maximized: config.maximized,
            visible: config.visible,
            transparent: config.transparent,
            decorations: config.decorations,
            always_on_top: config.always_on_top,
            window_icon: None, // TODO fetch from config,
        };

        Ok(val)
    }
}

impl ConfigWinit {
    /// Convert configuration values form toml to ConfigWinit.
    pub fn from_toml(val: toml::Value) -> Result<ConfigWinit> {
        let toml_config: TomlConfigWinit =
            err_at!(FailConvert, toml::from_str(&val.to_string()))?;
        Ok(toml_config.into())
    }
}

// local functions
impl ConfigWinit {
    fn to_inner_size(&self) -> Result<Option<dpi::Size>> {
        match &self.inner_size {
            Some(s) => Some(to_logical_size(s.as_slice())).transpose(),
            None => Ok(None),
        }
    }

    fn to_min_inner_size(&self) -> Result<Option<dpi::Size>> {
        match &self.min_inner_size {
            Some(s) => Some(to_logical_size(s.as_slice())).transpose(),
            None => Ok(None),
        }
    }

    fn to_max_inner_size(&self) -> Result<Option<dpi::Size>> {
        match &self.max_inner_size {
            Some(s) => Some(to_logical_size(s.as_slice())).transpose(),
            None => Ok(None),
        }
    }

    fn to_position(&self) -> Result<Option<dpi::Position>> {
        match &self.position {
            Some(s) => Some(to_logical_position(s.as_slice())).transpose(),
            None => Ok(None),
        }
    }
}

/// Convert slice of [f64] into dpi::Size. Note that length of slice should be 2 and the
/// returned size is [dpi::Size::Logical].
pub fn to_logical_size(size: &[f64]) -> Result<dpi::Size> {
    if size.len() == 2 {
        Ok(dpi::Size::Logical((size[0], size[1]).into()))
    } else {
        err_at!(Invalid, msg: "size invalid {:?}", size)
    }
}

/// Convert slice of [f64] into dpi::Position. Note that length of slice should be 2 and the
/// returned position is [dpi::Position::Logical].
pub fn to_logical_position(pos: &[f64]) -> Result<dpi::Position> {
    if pos.len() == 2 {
        Ok(dpi::Position::Logical((pos[0], pos[1]).into()))
    } else {
        err_at!(Invalid, msg: "position invalid {:?}", pos)
    }
}
