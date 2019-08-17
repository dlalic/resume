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
            let _: Foo = serde_yaml::from_str(&yaml).unwrap();
        }
    }

    #[test]
    #[should_panic(expected="wrong number of color components")]
    fn chokes_on_wrong_number_of_components() {
        let _: Foo = serde_yaml::from_str("color: 1").unwrap();
        let _: Foo = serde_yaml::from_str("color: 1, 2").unwrap();
        let _: Foo = serde_yaml::from_str("color: 1, 2, 3, 4").unwrap();
    }

    #[test]
    #[should_panic(expected="number too large")]
    fn chokes_on_out_of_range_values() {
        let _: Foo = serde_yaml::from_str("color: 0, 128, 256").unwrap();
        let _: Foo = serde_yaml::from_str("color: -1, 0, 1").unwrap();
    }

    #[test]
    #[should_panic(expected="invalid digit found")]
    fn chokes_on_invalid_chars() {
        let _: Foo = serde_yaml::from_str("color: white").unwrap();
        let _: Foo = serde_yaml::from_str("color: #aabbcc").unwrap();
    }
}
