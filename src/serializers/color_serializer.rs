use crate::models::color::Color;
use serde::{self, Deserialize, Deserializer};
use std::str::FromStr;

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<Color, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    Color::from_str(&string).map_err(serde::de::Error::custom)
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
