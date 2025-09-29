use std::fs;

use crate::{info, layout::assets::LayoutAssets, ui::StyleAssets};

pub struct AppConfig {
    xdg_dirs: xdg::BaseDirectories,
    custom_layout_path: Option<String>,
    custom_style_path: Option<String>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            xdg_dirs: xdg::BaseDirectories::with_prefix("wkeys").unwrap(),
            custom_layout_path: None,
            custom_style_path: None,
        }
    }
}

impl AppConfig {
    pub fn new(custom_layout_path: Option<String>, custom_style_path: Option<String>) -> Self {
        Self {
            xdg_dirs: xdg::BaseDirectories::with_prefix("wkeys").unwrap(),
            custom_layout_path,
            custom_style_path,
        }
    }

    fn get_config_file_content(&self, file_name: &str, fallback: &str) -> String {
        match self.xdg_dirs.find_config_file(file_name) {
            Some(css_path) => return fs::read_to_string(css_path).unwrap(),
            None => {
                return String::from(fallback);
            }
        }
    }

    fn get_custom_file_content(&self, file_path: &str) -> Result<String, std::io::Error> {
        fs::read_to_string(file_path)
    }

    pub fn get_css_file_content(&self) -> String {
        if let Some(path) = &self.custom_style_path {
            match self.get_custom_file_content(path) {
                Ok(content) => return content,
                Err(e) => info!("Error reading custom style file: {}", e)
            }
        }
        self.get_config_file_content("style.css", &StyleAssets::get_default_style_file())
    }

    pub fn get_layout_file_content(&self) -> String {
        if let Some(path) = &self.custom_layout_path {
            match self.get_custom_file_content(path) {
                Ok(content) => return content,
                Err(e) => info!("Error reading custom layout file: {}", e)
            }
        }
        self.get_config_file_content(
            "layout.toml",
            &&LayoutAssets::get_get_default_60_percent_layout_str(),
        )
    }
}
