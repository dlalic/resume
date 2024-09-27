use resume::Integration;

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// A resume yaml specification, see examples/resume.yaml
    #[clap(short, long, default_value = "examples/resume.yaml")]
    config: String,

    /// Render the resume to a .tex file or the browser extension
    #[clap(value_enum)]
    mode: Integration,
}

fn main() {
    let cli = Cli::parse();
    println!("Using config: {}", &cli.config);
    match resume::run(cli.mode, &cli.config) {
        Ok(output) => println!("{}", output),
        Err(e) => println!("Error: {:?}", e),
    }
}
