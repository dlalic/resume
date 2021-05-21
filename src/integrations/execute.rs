use crate::models::resume::Resume;
use anyhow::Result;

pub(crate) trait Execute {
    fn execute(resume: &Resume) -> Result<String>;
}
