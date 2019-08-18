use crate::integrations::execute::Execute;
use crate::integrations::sn::SnIntegration;
use crate::integrations::tex::TexIntegration;
use crate::models::resume::Resume;
use clap::ArgMatches;
use std::error::Error;
use std::fs;

mod integrations;
// TODO: The module shouldn't be public,
//       but #[cfg(bench)] doesn't seem to do the trick.
pub mod models;
mod serializers;

pub fn run(matches: &ArgMatches, config: &str) -> Result<String, Box<dyn Error>> {
    let contents = fs::read_to_string(config)?;
    let resume: Resume = serde_yaml::from_str(&contents)?;
    match Option::from(matches.subcommand_name()) {
        Some("tex") => TexIntegration::execute(&resume),
        Some("sn") => SnIntegration::execute(&resume),
        _ => Ok("Please specify a subcommand. Use --help for help.".to_string()),
    }
}
