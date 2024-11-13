use std::fs;

use crate::{layout::assets::LayoutAssets, ui::StyleAssets};

pub struct AppConfig {
    xdg_dirs: xdg::BaseDirectories,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            xdg_dirs: xdg::BaseDirectories::with_prefix("wkeys").unwrap(),
        }
    }
}

impl AppConfig {
    fn get_config_file_content(&self, file_name: &str, fallback: &str) -> String {
        match self.xdg_dirs.find_config_file(file_name) {
            Some(css_path) => return fs::read_to_string(css_path).unwrap(),
            None => {
                return String::from(fallback);
            }
        }
    }

    pub fn get_css_file_content(&self) -> String {
        self.get_config_file_content("style.css", &StyleAssets::get_default_style_file())
    }

    pub fn get_layout_file_content(&self) -> String {
        self.get_config_file_content(
            "layout.toml",
            &&LayoutAssets::get_get_default_60_percent_layout_str(),
        )
    }
}
