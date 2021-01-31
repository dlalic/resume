use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Person {
    pub(crate) name: String,
    pub(crate) pronoun: String,
    pub(crate) location: String,
    pub(crate) contact: String,
    pub(crate) image_file: Option<String>,
}
