use crate::models::resume::Resume;
use failure::Error;

pub(crate) trait Execute {
    fn execute(resume: &Resume) -> Result<String, Error>;
}
