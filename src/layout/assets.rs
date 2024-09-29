use core::str;

use rust_embed::Embed;

use super::parse::LayoutDefinition;

#[derive(Embed)]
#[folder = "assets/"]
pub struct LayoutAssets;

impl LayoutAssets {
    pub fn get_default_60_percent_layout() -> LayoutDefinition {
        // Self-hosted assets must success
        let layout_file = LayoutAssets::get("defaut-60%.toml").unwrap();
        let toml_str = str::from_utf8(layout_file.data.as_ref()).unwrap();
        LayoutDefinition::from_toml(toml_str)
    }
}
