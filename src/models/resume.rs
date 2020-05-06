extern crate askama;
use crate::models::education::Education;
use crate::models::employment::Employment;
use crate::models::layout_config::LayoutConfig;
use crate::models::person::Person;
use askama::Template;
use serde::Deserialize;

#[derive(Debug, Deserialize, Template)]
#[template(path = "resume.tex", escape = "none")]
pub struct Resume {
    #[serde(default)]
    pub(crate) config: LayoutConfig,
    pub(crate) person: Person,
    pub(crate) employments: Vec<Employment>,
    pub(crate) educations: Vec<Education>,
}
