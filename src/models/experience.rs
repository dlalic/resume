use crate::models::color::Color;
use crate::serializers::color_serializer;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Experience {
    pub(crate) name: String,
    pub(crate) percent: i8,
    #[serde(with = "color_serializer")]
    pub(crate) color: Color,
}

impl Experience {
    pub(crate) fn color_name(&self) -> String {
        format!("{}_color", self.name.to_lowercase())
    }
}
