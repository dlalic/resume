use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct LayoutConfig {
    pub(crate) font: String,
    pub(crate) font_size: String,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        LayoutConfig {
            font: "Avenir Next".to_string(),
            font_size: "12pt".to_string(),
        }
    }
}
