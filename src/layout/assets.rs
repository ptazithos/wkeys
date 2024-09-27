use rust_embed::Embed;
use tracing::info;

#[derive(Embed)]
#[folder = "assets/"]
pub struct LayoutAssets;

impl LayoutAssets {
    pub fn get_default_60_percent_layout() {
        // Self-hosted assets must success
        let layout_file = LayoutAssets::get("defaut-60%.toml").unwrap();
        info!("{:?}", std::str::from_utf8(layout_file.data.as_ref()));
    }
}
