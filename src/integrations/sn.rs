use crate::integrations::executable::Executable;
use crate::models::resume::Resume;
use std::error::Error;

pub(crate) struct SnIntegration {}

impl Executable for SnIntegration {
    fn execute(_: &Resume) -> Result<(String), Box<dyn Error>> {
        unimplemented!()
    }
}
