use std::fs;
use std::path::PathBuf;
use config::{Config, File};
use csscolorparser::Color;
use serde::{Deserialize, Serialize};
use windows::Win32::Foundation::COLORREF;

#[derive(Clone,Default,Serialize,Deserialize)]
enum ColorScheme{
    #[default]
    NoChange=-1,
    Dark=1,
    Light=0
}
#[derive(Clone,Default,Serialize,Deserialize)]
enum BorderRadius{
    #[default]
    NoChange=-1,
    Rect=1,
    Round=0
}
#[derive(Clone,Serialize,Deserialize)]
struct RawConfig {
    force_color_scheme:ColorScheme,
    force_border_radius:BorderRadius,
    active_border_color:Color,
    active_topmost_border_color:Color,
    inactive_border_color:Color,
    inactive_topmost_border_color:Color,
}
impl Default for RawConfig {
    fn default() -> Self {
        Self{
            force_color_scheme:ColorScheme::default(),
            force_border_radius:BorderRadius::default(),
            active_border_color:Color::from_rgba8(0x00,0xaa,0xff,0xff),
            inactive_border_color:Color::from_rgba8(0x80,0x80,0x80,0xff),
            active_topmost_border_color:Color::from_rgba8(0xff,0xba,0x00,0xff),
            inactive_topmost_border_color:Color::from_rgba8(0x77,0x55,0x00,0xff),
        }
    }
}
#[derive(Debug)]
pub struct DecodedConfig{
    pub force_color_scheme:i32,
    pub force_border_radius:i32,
    pub active_border_color:COLORREF,
    pub active_topmost_border_color:COLORREF,
    pub inactive_border_color:COLORREF,
    pub inactive_topmost_border_color:COLORREF,
}
impl From<RawConfig> for DecodedConfig{
    fn from(value: RawConfig) -> Self {
        Self{
            force_color_scheme: value.force_color_scheme as i32,
            force_border_radius: value.force_border_radius as i32,
            active_border_color: color_to_colorref(value.active_border_color),
            active_topmost_border_color: color_to_colorref(value.active_topmost_border_color),
            inactive_border_color: color_to_colorref(value.inactive_border_color),
            inactive_topmost_border_color: color_to_colorref(value.inactive_topmost_border_color),
        }
    }
}
impl Default for DecodedConfig{
    fn default() -> Self {
        RawConfig::default().into()
    }
}
fn color_to_colorref(color: Color) -> COLORREF {
    let [r,g,b,_]=color.to_rgba8();
    COLORREF(((b as u32) << 16) | ((g as u32) << 8) | (r as u32))
}
fn get_config_path() -> PathBuf {
    let mut exe_path = std::env::current_exe().unwrap_or_else(|_| PathBuf::from("."));
    exe_path.pop();
    exe_path.push("config.toml");
    exe_path
}
pub fn load_config()->DecodedConfig{
    let config_path =get_config_path();
    if !config_path.exists() {
        fs::write(config_path.clone(), include_str!("default_config.toml")).unwrap_or(());
    }
    let settings = Config::builder()
        .add_source(File::with_name(config_path.to_str().unwrap_or("./config.toml")))
        .build().unwrap_or_default();
    settings.try_deserialize::<RawConfig>().unwrap_or_default().into()
}