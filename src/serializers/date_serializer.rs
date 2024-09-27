use chrono::{NaiveDate, ParseResult};
use serde::{self, Deserialize, Deserializer};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    deserialize_str(&s).map_err(serde::de::Error::custom)
}

pub(crate) fn deserialize_str(s: &str) -> ParseResult<NaiveDate> {
    // Sticking to ISO 8601, as guessing the date format looks like a nightmare:
    // https://github.com/dateutil/dateutil/blob/master/dateutil/parser/isoparser.py
    let delimiter = "-";
    let mut components: Vec<&str> = s.split(delimiter).collect();
    // Per chrono documentation, "Out-of-bound dates or insufficient fields are errors."
    // Hence, dates missing %m or %d need to be appended with missing values
    while components.len() < 3 {
        components.push("1");
    }
    let format = "%Y %m %d";
    let date_string = components.join(" ");
    NaiveDate::parse_from_str(&date_string, format)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializers::date_serializer;
    use proptest::prelude::*;

    #[derive(Deserialize)]
    pub struct Foo {
        #[serde(with = "date_serializer")]
        pub _date: NaiveDate,
    }

    fn yaml_value(date_string: &str) -> Foo {
        let yaml = format!("_date: {}", date_string);
        serde_yaml::from_str(&yaml).unwrap()
    }

    proptest! {
        #[test]
        fn parses_all_valid_dates(component_count in 1usize..3, y in 0u32..10000, m in 1u32..13, d in 1u32..29) {
            let date_string = format!("{:04}-{:02}-{:02}", y, m ,d);
            let (first, _) = date_string.split_at(component_count * 4);
            let mut result = first.to_string();
            if result.ends_with("-") {
                result.pop();
            }
            let _: Foo = yaml_value(&result);
        }

        #[test]
        #[should_panic]
        fn crashes_on_invalid_characters(s in "\\PC*") {
            let _: Foo = yaml_value(&s);
        }

        #[test]
        #[should_panic]
        fn crashes_on_wrong_delimiter(s in "[0-9]{4}[^-][0-9]{2}[^-][0-9]{2}") {
            let _: Foo = yaml_value(&s);
        }
    }
}
