use crate::models::color::Color;
use serde::{self, Deserialize, Deserializer};
use serde_yaml::from_str;

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_string: String = String::deserialize(deserializer)?;
    let components: Vec<&str> = raw_string.split_terminator(",").collect();

    fn color_value(component : &str) -> Result<u8, &'static str> {
       let value = from_str(component).unwrap();
       match value {
           0..=255 => Ok(value as u8),
           _ => Err("color value out of range")
       }
    }

    let color: Color = Color {
        red: color_value(components[0]).unwrap(),
        green: color_value(components[1]).unwrap(),
        blue: color_value(components[2]).unwrap()
    };
    Ok(color)
}
