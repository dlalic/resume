use crate::serializers::date_serializer;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Employment {
    pub(crate) company: String,
    pub(crate) title: String,
    #[serde(with = "date_serializer")]
    pub(crate) start: NaiveDate,
    #[serde(with = "date_serializer")]
    pub(crate) end: NaiveDate,
    pub(crate) keywords: Vec<String>,
    pub(crate) points: Vec<String>,
}

impl Employment {
    pub(crate) fn duration(&self) -> String {
        format!("{} - {}", self.start.format("%Y"), self.end.format("%Y"))
    }
}
