#[macro_use]
extern crate clap;

use clap::App;
use resume::Integration;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let config = matches.value_of("config").unwrap_or("examples/resume.yaml");
    println!("Using config: {}", config);
    let mode = matches.value_of("mode").unwrap_or("tex");
    let int = match mode {
        "tex" => Integration::Tex,
        "be" => Integration::BrowserExtension,
        _ => Integration::Tex,
    };
    match resume::run(int, config) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("Error: {:?}", e),
    }
}
