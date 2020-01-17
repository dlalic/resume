use crate::models::color::Color;
use crate::serializers::color_serializer;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Skill {
    pub(crate) name: String,
    pub(crate) level: i8,
    #[serde(with = "color_serializer")]
    pub(crate) start_color: Color,
    #[serde(with = "color_serializer")]
    pub(crate) end_color: Color,
}

impl Skill {
    pub(crate) fn start_color_name(&self) -> String {
        format!("{}_color_start", self.name.to_lowercase())
    }
    pub(crate) fn end_color_name(&self) -> String {
        format!("{}_color_end", self.name.to_lowercase())
    }
}
