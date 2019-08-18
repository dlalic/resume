#[macro_use]
extern crate clap;

use clap::App;
use resume::Integration;

fn from_subcommand(matches: &clap::ArgMatches) -> Option<Integration> {
    match Option::from(matches.subcommand_name()) {
        Some("tex") => Some(Integration::Tex),
        Some("sn") => Some(Integration::Sn),
        _ => None,
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("examples/resume.yaml");
    println!("Using config: {}", config);
    if let Some(int) = from_subcommand(&matches) {
        match resume::run(int, config) {
            Ok(output) => println!("{}", output),
            Err(e) => println!("Error: {:?}", e),
        }
    } else {
        println!("Please specify a subcommand. Use --help for help.");
    }
}
