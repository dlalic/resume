use crate::models::education::Education;
use crate::models::employment::Employment;
use crate::models::resume::Resume;
use askama::Template;

#[derive(Debug, Template)]
#[template(path = "base.html", escape = "none")]
pub struct ReducedResume {
    pub(crate) employments: Vec<Employment>,
    pub(crate) educations: Vec<Education>,
}

impl From<&Resume> for ReducedResume {
    fn from(resume: &Resume) -> Self {
        let employments = &resume.employments;
        let educations = &resume.educations;
        ReducedResume {
            employments: employments.to_vec(),
            educations: educations.to_vec(),
        }
    }
}
