use serde::Deserialize;

use winit::{dpi, window};

use crate::{Error, Result};

#[derive(Clone, Deserialize)]
pub struct TomlWinitConfig {
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

//-----

#[derive(Clone, Debug)]
pub struct WinitConfig {
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

impl Default for WinitConfig {
    fn default() -> WinitConfig {
        WinitConfig {
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
            #[cfg(unix)]
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

impl From<TomlWinitConfig> for WinitConfig {
    fn from(toml_config: TomlWinitConfig) -> WinitConfig {
        let c = WinitConfig::default();
        WinitConfig {
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

impl WinitConfig {
    pub fn from_toml(val: toml::Value) -> Result<WinitConfig> {
        let toml_config: TomlWinitConfig =
            err_at!(FailConvert, toml::from_str(&val.to_string()))?;
        Ok(toml_config.into())
    }

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

    pub fn to_window_attributes(&self) -> Result<window::WindowAttributes> {
        let val = window::WindowAttributes {
            inner_size: self.to_inner_size()?,
            min_inner_size: self.to_min_inner_size()?,
            max_inner_size: self.to_max_inner_size()?,
            position: self.to_position()?,
            resizable: self.resizable,
            fullscreen: None, // TODO fetch from self,
            title: self.title.clone(),
            maximized: self.maximized,
            visible: self.visible,
            transparent: self.transparent,
            decorations: self.decorations,
            always_on_top: self.always_on_top,
            window_icon: None, // TODO fetch from self,
        };

        Ok(val)
    }
}

pub fn to_logical_size(size: &[f64]) -> Result<dpi::Size> {
    if size.len() == 2 {
        Ok(dpi::Size::Logical((size[0], size[1]).into()))
    } else {
        err_at!(Invalid, msg: "size invalid {:?}", size)
    }
}

pub fn to_logical_position(pos: &[f64]) -> Result<dpi::Position> {
    if pos.len() == 2 {
        Ok(dpi::Position::Logical((pos[0], pos[1]).into()))
    } else {
        err_at!(Invalid, msg: "position invalid {:?}", pos)
    }
}
