use std::path::PathBuf;

use gpui::{App, SharedString};
use gpui_component::{Theme, ThemeRegistry};

#[derive(rust_embed::RustEmbed)]
#[folder = "./assets"]
#[include = "icons/**/*.svg"]
pub struct Assets;

impl gpui::AssetSource for Assets {
    fn load(&self, path: &str) -> anyhow::Result<Option<std::borrow::Cow<'static, [u8]>>> {
        if path.is_empty() {
            return Ok(None);
        }

        Self::get(path)
            .map(|f| Some(f.data))
            .ok_or_else(|| anyhow::anyhow!("could not find asset at path `{path}`"))
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        Ok(Self::iter()
            .filter_map(|p| p.starts_with(path).then(|| p.into()))
            .collect())
    }
}

pub fn init_theme(cx: &mut App) {
    let theme_name = "Ayu Dark";
    let theme_dir = PathBuf::from("./themes");
    let on_load = move |cx: &mut App| {
        if let Some(theme) = ThemeRegistry::global(cx).themes().get(theme_name).cloned() {
            Theme::global_mut(cx).apply_config(&theme);
        }
    };

    if let Err(err) = ThemeRegistry::watch_dir(theme_dir, cx, on_load) {
        eprintln!("Failed to watch themes directory: {}", err);
    }
}
