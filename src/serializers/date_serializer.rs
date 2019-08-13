use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_string = String::deserialize(deserializer)?;
    // Sticking to ISO 8601, as guessing the date format looks like a nightmare:
    // https://github.com/dateutil/dateutil/blob/master/dateutil/parser/isoparser.py
    let delimiter = "-";
    let mut components: Vec<&str> = raw_string.split(delimiter).collect();
    // Per chrono documentation, "Out-of-bound dates or insufficient fields are errors."
    // Hence, dates missing %m or %d need to be appended with missing values
    while components.len() < 3 {
        components.push("1");
    }
    let format = String::from("%Y %m %d");
    let date_string = components.join(" ");
    NaiveDate::parse_from_str(&date_string, &format).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializers::date_serializer;
    use proptest::prelude::*;

    #[derive(Deserialize)]
    pub struct Foo {
        #[serde(with = "date_serializer")]
        pub date: NaiveDate,
    }

    proptest! {
        #[test]
        fn parses_all_valid_dates(y in 0u32..10000, m in 1u32..13, d in 1u32..29) {
            let date_string = format!("{:04}-{:02}-{:02}", y, m, d);
            let yaml = format!("date: {}", &date_string);
            let _: Foo = serde_yaml::from_str(&yaml).unwrap();
        }
    }

    #[test]
    #[should_panic]
    fn test_de_invalid_input() {
        let invalid_inputs= vec!["2019 3", "34-56-44", "2019.6.1"];
        for date_string in invalid_inputs {
            let yaml = format!("date: {}", date_string);
            let _: Foo = serde_yaml::from_str(&yaml).unwrap();
        }
    }
}
