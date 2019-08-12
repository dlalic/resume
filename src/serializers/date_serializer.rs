use chrono::NaiveDate;
use serde::{self, Deserialize, Deserializer};

pub(crate) fn deserialize<'de, D>(deserializer: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let raw_string: String = String::deserialize(deserializer)?;
    // Sticking to ISO 8601, as guessing the date format looks like a nightmare:
    // https://github.com/dateutil/dateutil/blob/master/dateutil/parser/isoparser.py
    let delimiter: &str = "-";
    let mut components: Vec<&str> = raw_string.split(delimiter).collect();
    // Per chrono documentation, "Out-of-bound dates or insufficient fields are errors."
    // Hence, dates missing %m or %d need to be appended with missing values
    while components.len() < 3 {
        components.push("1");
    }
    let format: String = String::from("%Y %m %d");
    let date_string: String = components.join(" ");
    NaiveDate::parse_from_str(&date_string, &format).map_err(serde::de::Error::custom)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serializers::date_serializer;
    use std::collections::HashMap;

    #[derive(Deserialize)]
    pub struct Foo {
        #[serde(with = "date_serializer")]
        pub date: NaiveDate,
    }

    #[test]
    fn test_de_valid_input() {
        let mut valid_input: HashMap<&str, NaiveDate> = HashMap::new();
        valid_input.insert("2019", NaiveDate::from_ymd(2019, 1, 1));
        valid_input.insert("2019-3", NaiveDate::from_ymd(2019, 3, 1));
        valid_input.insert("2019-12-1", NaiveDate::from_ymd(2019, 12, 1));
        for (&date_string, &date) in valid_input.iter() {
            let yaml: String = format!("date: {}", date_string);
            let result: Foo = serde_yaml::from_str(&yaml).unwrap();
            assert_eq!(result.date, date);
        }
    }

    #[test]
    #[should_panic]
    fn test_de_invalid_input() {
        let invalid_inputs: Vec<&str> = vec!["2019 3", "34-56-44", "2019.6.1"];
        for date_string in invalid_inputs {
            let yaml: String = format!("date: {}", date_string);
            let _: Foo = serde_yaml::from_str(&yaml).unwrap();
        }
    }
}
