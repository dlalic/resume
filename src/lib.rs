use crate::integrations::browser_extension::BrowserExtensionIntegration;
use crate::integrations::execute::Execute;
use crate::integrations::tex::TexIntegration;
use crate::models::resume::Resume;

use anyhow::Result;
use clap::ValueEnum;
use std::fs;

mod integrations;
// TODO: The module shouldn't be public,
//       but #[cfg(bench)] doesn't seem to do the trick.
pub mod models;
mod operations;
mod serializers;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Integration {
    Tex,
    BrowserExtension,
}

pub fn run(int: Integration, config: &str) -> Result<String> {
    let contents = fs::read_to_string(config)?;
    let resume: Resume = serde_yaml::from_str(&contents)?;
    match int {
        Integration::Tex => TexIntegration::execute(&resume),
        Integration::BrowserExtension => BrowserExtensionIntegration::execute(&resume),
    }
}
