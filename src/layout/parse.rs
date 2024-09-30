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
    #[serde(skip_deserializing)]
    pub width: f32,
    #[serde(skip_deserializing)]
    pub height: i32,
}

impl LayoutDefinition {
    pub fn from_toml(toml_str: &str) -> Self {
        if let Ok(mut layout_definition) = toml::from_str::<LayoutDefinition>(toml_str) {
            layout_definition.height = layout_definition.layout.len() as i32;
            layout_definition.width = layout_definition
                .layout
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|key| key.width.unwrap_or(1.0))
                        .reduce(|prev, cur| prev + cur)
                        .unwrap()
                })
                .max_by(|a, b| a.partial_cmp(b).unwrap())
                .unwrap();

            layout_definition
        } else {
            panic!("Failed to parse layout definition")
        }
    }
}