use crate::integrations::execute::Execute;
use crate::models::resume::Resume;
use std::error::Error;

pub(crate) struct SnIntegration {}

impl Execute for SnIntegration {
    fn execute(_: &Resume) -> Result<(String), Box<dyn Error>> {
        unimplemented!()
    }
}
