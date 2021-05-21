use crate::integrations::execute::Execute;
use crate::models::resume::Resume;
use crate::operations::render_template::render_template;
use anyhow::Result;
use std::path::PathBuf;

pub(crate) struct TexIntegration {}

impl Execute for TexIntegration {
    fn execute(resume: &Resume) -> Result<String> {
        let path = PathBuf::from("output.tex");
        render_template(resume, &path)
    }
}
