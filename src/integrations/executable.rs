use crate::models::resume::Resume;
use std::error::Error;

pub(crate) trait Executable {
    fn execute(resume: &Resume) -> Result<String, Box<dyn Error>>;
}
