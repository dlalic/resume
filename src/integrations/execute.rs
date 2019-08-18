use crate::models::resume::Resume;
use std::error::Error;

pub(crate) trait Execute {
    fn execute(resume: &Resume) -> Result<String, Box<dyn Error>>;
}
