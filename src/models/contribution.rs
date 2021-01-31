use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Contribution {
    pub(crate) name: String,
    pub(crate) url: String,
}
