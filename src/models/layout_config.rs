use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct LayoutConfig {
    pub(crate) font: String,
    pub(crate) font_location: String,
    pub(crate) font_size: String,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        let path: PathBuf = [".", "examples", "fonts", ""].iter().collect();
        LayoutConfig {
            font_location: path.into_os_string().into_string().unwrap(),
            font: "FiraSans".to_string(),
            font_size: "10pt".to_string(),
        }
    }
}
