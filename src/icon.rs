use anyhow::{Context, Result};
use tray_icon::Icon;

const ICON_BYTES: &[u8] = include_bytes!("assets/tray_icon.png");

pub fn load_app_icon() -> Result<Icon> {
    let image = image::load_from_memory(ICON_BYTES)
        .context("failed to parse icon image")?
        .into_rgba8(); // 强制转换为 RGBA 格式

    let (width, height) = image.dimensions();
    let rgba = image.into_raw();

    Icon::from_rgba(rgba, width, height).context("failed to create tray icon")
}
