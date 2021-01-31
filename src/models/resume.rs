extern crate askama;
use crate::models::contribution::Contribution;
use crate::models::education::Education;
use crate::models::employment::Employment;
use crate::models::experience::Experience;
use crate::models::layout_config::LayoutConfig;
use crate::models::person::Person;
use crate::models::skill::Skill;
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
    pub(crate) skills: Vec<Skill>,
    pub(crate) experiences: Vec<Experience>,
    pub(crate) contributions: Vec<Contribution>,
}
