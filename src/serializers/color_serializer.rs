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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializers::color_serializer;
    use proptest::prelude::*;

    #[derive(Deserialize)]
    pub struct Foo {
        #[serde(with = "color_serializer")]
        pub color: Color,
    }

    proptest! {
        #[test]
        fn parses_all_valid_colors(r in 0u8..255, g in 0u8..255, b in 0u8..255) {
            let color_string = format!("{}, {}, {}", r, g, b);
            let yaml: String = format!("color: {}", &color_string);
            let _result: Foo = serde_yaml::from_str(&yaml).unwrap();
        }
    }
}
