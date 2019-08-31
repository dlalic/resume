use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NativeManifest {
    name: String,
    description: String,
    pub(crate) path: String,
    #[serde(rename = "type")]
    nm_type: String,
    allowed_extensions: Vec<String>,
}

impl Default for NativeManifest {
    fn default() -> Self {
        NativeManifest {
            name: "resume".to_string(),
            description: "Bridge between the browser extension and the Rust librar".to_string(),
            path: "".to_string(),
            nm_type: "stdio".to_string(),
            // TODO: read from manifest.json instead of hardcoding
            allowed_extensions: vec!["dlalic@users.noreply.github.com".to_string()],
        }
    }
}
