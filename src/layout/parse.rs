use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KeyDefinition {
    pub top_legend: Option<String>,
    pub bottom_legend: Option<String>,
    pub scan_code: u16,
    pub width: Option<f32>,
}

pub type Layout = Vec<Vec<KeyDefinition>>;

#[derive(Serialize, Deserialize, Debug)]
pub struct LayoutDefinition {
    pub layout: Layout,
}

impl LayoutDefinition {
    pub fn from_toml(toml_str: &str) -> Result<Layout, toml::de::Error> {
        match toml::from_str::<LayoutDefinition>(toml_str) {
            Ok(layout_def) => Ok(layout_def.layout),
            Err(err) => Err(err),
        }
    }
}
