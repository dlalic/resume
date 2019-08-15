use crate::integrations::tex::execute;
use clap::ArgMatches;
use std::error::Error;

mod integrations;
// TODO: The module shouldn't be public,
//       but #[cfg(bench)] doesn't seem to do the trick.
pub mod models;
mod serializers;

pub fn run(matches: &ArgMatches, config: &str) -> Result<(), Box<dyn Error>> {
    if let Some(_) = matches.subcommand_matches("tex") {
        if let Err(e) = execute(config.to_string()) {
            println!("{:?}", e);
        }
    } else {
        println!("Please specify a subcommand. Use --help for help.");
    }
    Ok(())
}
