use crate::webextension::input::Input;
use crate::webextension::output::Output;
use crate::webextension::output::Status::{Error, Success};
use std::io;
pub mod webextension;

fn main() {
    loop {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut input = stdin.lock();
        let mut output = stdout.lock();

        let request = Input::read_from(&mut input);
        match request {
            Ok(r) => {
                let response = Output {
                    status: Success,
                    data: "foo".to_string(),
                };
                response.write_to(&mut output).unwrap_or_else(|err| {
                    println!("Error handling request: {:?}", err);
                });
            }
            Err(e) => {
                let response = Output {
                    status: Error,
                    data: format!("Error: {:?}", e),
                };
                response.write_to(&mut output).unwrap_or_else(|err| {
                    println!("Error handling request: {:?}", err);
                });
            }
        }
    }
}
