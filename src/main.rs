#[macro_use]
extern crate clap;

use clap::App;
use resume::Integration;

fn from_subcommand(matches: &clap::ArgMatches) -> Option<Integration> {
    match Option::from(matches.subcommand_name()) {
        Some("tex") => Some(Integration::Tex),
        Some("xing") => Some(Integration::Xing),
        _ => None
    }
}

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("examples/resume.yaml");

    if let Some(int) = from_subcommand(&matches) {
        println!("Using config: {}", config);
        match resume::run(int, config) {
            Ok(output) => println!("Wrote {}", output),
            Err(e) => println!("Error: {:?}", e)
        }
    } else {
        println!("Please specify a subcommand. Use --help for help.");
    }
}
