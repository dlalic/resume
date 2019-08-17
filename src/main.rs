#[macro_use]
extern crate clap;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("examples/resume.yaml");
    println!("Using config: {}", config);
    match resume::run(&matches, config) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("Error: {:?}", e)
    }
}
