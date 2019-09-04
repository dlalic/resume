use crate::integrations::execute::Execute;
use crate::models::reduced_resume::ReducedResume;
use crate::models::resume::Resume;
use crate::operations::build_extension::build_extension;
use crate::operations::render_template::render_template;
use crate::operations::write_manifest::write_manifest;
use failure::Error;
use std::path::PathBuf;

extern crate dirs;
pub(crate) struct SnIntegration {}

impl Execute for SnIntegration {
    fn execute(resume: &Resume) -> Result<(String), Error> {
        let reduced = ReducedResume::from(resume);
        let path: PathBuf = ["browser-extension", "sidebar", "panel.html"]
            .iter()
            .collect();
        render_template(&reduced, &path)?;
        build_extension()?;
        write_manifest()?;
        Ok("Open Firefox and navigate to about:debugging. \
            Click the 'Load Temporary Add-on...' button and \
            select the manifest.json file from browser-extension directory."
            .to_string())
    }
}
