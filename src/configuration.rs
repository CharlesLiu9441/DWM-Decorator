use config::{Config, File};
use csscolorparser::Color;
use handy_keys::{Hotkey, Key, Modifiers};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};
use tracing::{error, info, warn};
use windows::Win32::Foundation::COLORREF;

#[derive(Clone, Default, Serialize, Deserialize)]
enum ColorScheme {
    #[default]
    NoChange = -1,
    Dark = 1,
    Light = 0,
}
#[derive(Clone, Default, Serialize, Deserialize)]
enum BorderRadius {
    #[default]
    NoChange = -1,
    Rect = 1,
    Round = 0,
}
#[derive(Clone, Serialize, Deserialize)]
struct RawConfig {
    force_color_scheme: ColorScheme,
    force_border_radius: BorderRadius,
    active_border_color: Color,
    active_topmost_border_color: Color,
    inactive_border_color: Color,
    inactive_topmost_border_color: Color,
    key_toggle_topmost: String,
    key_increase_transparency: String,
    key_decrease_transparency: String,
    active_title_color: Option<Color>,
    inactive_title_color: Option<Color>,
    active_text_color: Option<Color>,
    inactive_text_color: Option<Color>,
}
impl Default for RawConfig {
    fn default() -> Self {
        Self {
            force_color_scheme: ColorScheme::default(),
            force_border_radius: BorderRadius::default(),
            active_border_color: Color::from_rgba8(0x00, 0xaa, 0xff, 0xff),
            inactive_border_color: Color::from_rgba8(0x80, 0x80, 0x80, 0xff),
            active_topmost_border_color: Color::from_rgba8(0xff, 0xba, 0x00, 0xff),
            inactive_topmost_border_color: Color::from_rgba8(0x77, 0x55, 0x00, 0xff),
            key_toggle_topmost: "Ctrl+Keypad0".to_string(),
            key_increase_transparency: "Ctrl+Keypad2".to_string(),
            key_decrease_transparency: "Ctrl+Keypad8".to_string(),
            active_title_color: None,
            inactive_title_color: None,
            active_text_color: None,
            inactive_text_color: None,
        }
    }
}
#[derive(Debug, Copy, Clone)]
pub struct DecodedConfig {
    pub force_color_scheme: i32,
    pub force_border_radius: i32,
    pub active_border_color: COLORREF,
    pub active_topmost_border_color: COLORREF,
    pub inactive_border_color: COLORREF,
    pub inactive_topmost_border_color: COLORREF,
    pub key_toggle_topmost: Hotkey,
    pub key_increase_transparency: Hotkey,
    pub key_decrease_transparency: Hotkey,
    pub active_title_color: Option<COLORREF>,
    pub inactive_title_color: Option<COLORREF>,
    pub active_text_color: Option<COLORREF>,
    pub inactive_text_color: Option<COLORREF>,
}
impl From<RawConfig> for DecodedConfig {
    fn from(value: RawConfig) -> Self {
        Self {
            force_color_scheme: value.force_color_scheme as i32,
            force_border_radius: value.force_border_radius as i32,
            active_border_color: color_to_color_ref(value.active_border_color),
            active_topmost_border_color: color_to_color_ref(value.active_topmost_border_color),
            inactive_border_color: color_to_color_ref(value.inactive_border_color),
            inactive_topmost_border_color: color_to_color_ref(value.inactive_topmost_border_color),
            key_toggle_topmost: value.key_toggle_topmost.parse().unwrap_or_else(|error| {
                warn!(%error,"Fail to parse key configuration for topmost; using default one");
                Hotkey::new(Modifiers::CTRL, Key::Keypad0).unwrap()
            }),
            key_decrease_transparency: value.key_decrease_transparency.parse().unwrap_or_else(|error| {
                warn!(%error,"Fail to parse key configuration for decrease transparency; using default one");
                Hotkey::new(Modifiers::CTRL, Key::Keypad8).unwrap()
            }),
            key_increase_transparency: value.key_increase_transparency.parse().unwrap_or_else(|error| {
                warn!(%error,"Fail to parse key configuration for increase transparency; using default one");
                Hotkey::new(Modifiers::CTRL, Key::Keypad2).unwrap()
            }),
            active_title_color: option_color_to_color_ref(value.active_title_color),
            inactive_title_color: option_color_to_color_ref(value.inactive_title_color),
            active_text_color: option_color_to_color_ref(value.active_text_color),
            inactive_text_color: option_color_to_color_ref(value.inactive_text_color),
        }
    }
}
impl Default for DecodedConfig {
    fn default() -> Self {
        RawConfig::default().into()
    }
}
fn color_to_color_ref(color: Color) -> COLORREF {
    let [r, g, b, _] = color.to_rgba8();
    COLORREF(((b as u32) << 16) | ((g as u32) << 8) | (r as u32))
}
fn option_color_to_color_ref(color: Option<Color>) -> Option<COLORREF> {
    match color {
        Some(color) => Some(color_to_color_ref(color)),
        None => None,
    }
}
fn get_config_path() -> PathBuf {
    let mut exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    exe_path.pop();
    exe_path.push("config.toml");
    exe_path
}
pub fn load_config() -> DecodedConfig {
    let config_path = get_config_path();
    info!("Loading configuration from {}", config_path.display());
    if !config_path.exists() {
        fs::write(config_path.clone(), include_str!("default_config.toml"))
            .unwrap_or_else(|error| error!(%error,"Error generating default config.toml"));
    }
    let settings = Config::builder()
        .add_source(File::from(config_path))
        .build()
        .unwrap_or_else(|error| {
            warn!(%error,"Could not load configuration file; using default.");
            Config::default()
        });
    settings
        .try_deserialize::<RawConfig>()
        .unwrap_or_else(|error| {
            warn!(%error,"Could not parse config; using default.");
            RawConfig::default()
        })
        .into()
}
