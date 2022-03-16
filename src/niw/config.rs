use serde::Deserialize;

use crate::{Error, Result};

#[derive(Clone, Deserialize)]
pub struct TomlWinitConfig {
    title: Option<String>,
    visible: Option<bool>,
    alway_on_top: Option<bool>,
    maximised: Option<bool>,
    minimised: Option<bool>,
    resizeable: Option<bool>,
    cursor_position: Option<Vec<i32>>,
    cursor_visible: Option<bool>,
    window_decorations: Option<bool>,
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

#[derive(Clone)]
pub struct WinitConfig {
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

impl Default for WinitConfig {
    fn default() -> WinitConfig {
        WinitConfig {
            title: "gpgpu".to_string(),
            visible: true,
            alway_on_top: false,
            maximised: false,
            minimised: false,
            resizeable: true,
            cursor_position: None,
            cursor_visible: true,
            window_decorations: true,
            inner_size: None,
            max_inner_size: None,
            min_inner_size: None,
            outer_position: None,
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
            alway_on_top: from_toml!(toml_config, alway_on_top, c),
            maximised: from_toml!(toml_config, maximised, c),
            minimised: from_toml!(toml_config, minimised, c),
            resizeable: from_toml!(toml_config, resizeable, c),
            cursor_position: from_toml!(opt, toml_config, cursor_position, c),
            cursor_visible: from_toml!(toml_config, cursor_visible, c),
            window_decorations: from_toml!(toml_config, window_decorations, c),
            inner_size: from_toml!(opt, toml_config, inner_size, c),
            max_inner_size: from_toml!(opt, toml_config, max_inner_size, c),
            min_inner_size: from_toml!(opt, toml_config, min_inner_size, c),
            outer_position: from_toml!(opt, toml_config, outer_position, c),
        }
    }
}

impl WinitConfig {
    pub fn from_toml(val: toml::Value) -> Result<WinitConfig> {
        let toml_config: TomlWinitConfig =
            err_at!(FailConvert, toml::from_str(&val.to_string()))?;
        Ok(toml_config.into())
    }
}
