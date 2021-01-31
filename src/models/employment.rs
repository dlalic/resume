use crate::serializers::date_serializer;
use chrono::NaiveDate;
use serde::de;
use serde::{Deserialize, Deserializer};
use std::fmt;

#[derive(Debug, Clone)]
pub struct StartDate(NaiveDate);

#[derive(Debug, Clone)]
pub enum EndDate {
    None,
    Date(NaiveDate),
}

#[derive(Debug, Clone, Deserialize)]
pub struct Employment {
    pub(crate) company: String,
    pub(crate) title: String,
    #[serde(deserialize_with = "deserialize_start_date")]
    pub(crate) start: StartDate,
    #[serde(deserialize_with = "deserialize_end_date")]
    pub(crate) end: EndDate,
    pub(crate) keywords: Vec<String>,
    pub(crate) points: Vec<String>,
}

impl Employment {
    pub(crate) fn duration(&self) -> String {
        format!("{} - {}", self.start, self.end)
    }
}

impl fmt::Display for StartDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format("%b %Y"))
    }
}

impl fmt::Display for EndDate {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => write!(f, "present"),
            Self::Date(date) => write!(f, "{}", date.format("%b %Y")),
        }
    }
}

fn deserialize_start_date<'de, D>(deserializer: D) -> Result<StartDate, D::Error>
where
    D: Deserializer<'de>,
{
    date_serializer::deserialize(deserializer).map(StartDate)
}

fn deserialize_end_date<'de, D>(deserializer: D) -> Result<EndDate, D::Error>
where
    D: Deserializer<'de>,
{
    struct EndDateVisitor;

    impl<'de> de::Visitor<'de> for EndDateVisitor {
        type Value = EndDate;

        fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
            formatter.write_str("a date or the string 'none'")
        }

        fn visit_str<E: de::Error>(self, s: &str) -> Result<Self::Value, E> {
            match s {
                "none" => Ok(Self::Value::None),
                _ => date_serializer::deserialize_str(&s)
                    .map(EndDate::Date)
                    .map_err(serde::de::Error::custom),
            }
        }
    }

    deserializer.deserialize_str(EndDateVisitor)
}
