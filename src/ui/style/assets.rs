use core::str;

use rust_embed::Embed;

#[derive(Embed)]
#[folder = "assets/"]
pub struct StyleAssets;

impl StyleAssets {
    pub fn get_default_style_file() -> String {
        // Self-hosted assets must success
        let css_file = StyleAssets::get("default-style.css").unwrap();
        let css_file_str = str::from_utf8(css_file.data.as_ref()).unwrap();
        String::from(css_file_str)
    }
}
