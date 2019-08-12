use crate::serializers::date_serializer;
use chrono::NaiveDate;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Education {
    pub(crate) institution: String,
    pub(crate) title: String,
    #[serde(with = "date_serializer")]
    pub(crate) start: NaiveDate,
    #[serde(with = "date_serializer")]
    pub(crate) end: NaiveDate,
}

impl Education {
    pub(crate) fn duration(&self) -> String {
        format!("{} - {}", self.start.format("%Y"), self.end.format("%Y"))
    }
}
