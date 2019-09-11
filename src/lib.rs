use crate::integrations::execute::Execute;
use crate::integrations::sn::SnIntegration;
use crate::integrations::tex::TexIntegration;
use crate::models::resume::Resume;
extern crate failure;

use failure::Error;
use std::fs;

mod integrations;
// TODO: The module shouldn't be public,
//       but #[cfg(bench)] doesn't seem to do the trick.
pub mod models;
mod operations;
mod serializers;

#[derive(Debug)]
pub enum Integration {
    Tex,
    Sn,
}

pub fn run(int: Integration, config: &str) -> Result<String, Error> {
    let contents = fs::read_to_string(config)?;
    let resume: Resume = serde_yaml::from_str(&contents)?;
    match int {
        Integration::Tex => TexIntegration::execute(&resume),
        Integration::Sn => SnIntegration::execute(&resume),
    }
}
