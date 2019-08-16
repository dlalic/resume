#[macro_use]
extern crate clap;

use clap::App;

use resume::Integration;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("examples/resume.yaml");
    println!("Using config: {}", config);

    if let Some(_) = matches.subcommand_matches("tex") {
         match resume::run(Integration::Tex, config) {
             Ok(output) => println!("Wrote {}", output),
             Err(e) => println!("Error: {:?}", e)
         }

    } else {
         println!("Please specify a subcommand. Use --help for help.");
    }
}
