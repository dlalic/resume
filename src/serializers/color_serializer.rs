use crate::models::color::Color;
use serde::{self, Deserialize, Deserializer};
use serde_yaml::from_str;

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_string: String = String::deserialize(deserializer)?;
    let components: Vec<&str> = raw_string.split_terminator(",").collect();
    // TODO: throw if above 255
    let color: Color = Color {
        red: from_str(components[0]).unwrap(),
        green: from_str(components[1]).unwrap(),
        blue: from_str(components[2]).unwrap(),
    };
    Ok(color)
}
