use crate::integrations::execute::Execute;
use crate::models::resume::Resume;
use askama::Template;
use std::error::Error;
use std::fs::File;
use std::io::Write;

pub(crate) struct TexIntegration {}

impl Execute for TexIntegration {
    fn execute(resume: &Resume) -> Result<(String), Box<dyn Error>> {
        let output = "output.tex";
        let mut file = File::create(output)?;
        file.write_all(resume.render()?.as_bytes())?;
        Ok(format!("Wrote {}", output.to_string()))
    }
}
