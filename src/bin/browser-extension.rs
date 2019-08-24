use byteorder::{NativeEndian, ReadBytesExt, WriteBytesExt};
use serde::{Deserialize, Serialize};
use std::io;
use std::io::Read;
use std::io::{StdinLock, StdoutLock, Write};

#[derive(Serialize, Debug)]
enum Error {
    IOError,
    BadJSON,
}

#[derive(Deserialize)]
struct BrowserRequest {
    #[serde(rename = "id")]
    element_id: String,
    #[serde(rename = "type")]
    element_type: String,
    #[serde(rename = "name")]
    element_name: String,
}

#[derive(Serialize)]
struct BrowserResponse {
    suggestions: Vec<String>,
}

impl BrowserRequest {
    fn read_from(input: &mut StdinLock) -> Result<BrowserRequest, Error> {
        let length = input
            .read_u32::<NativeEndian>()
            .map_err(|_err| Error::IOError)?;
        let mut message = input.take(length as u64);
        let mut buffer = Vec::with_capacity(length as usize);
        message
            .read_to_end(&mut buffer)
            .map_err(|_err| Error::IOError)?;
        serde_json::from_slice(&buffer).map_err(|err| Error::BadJSON)
    }
}

impl BrowserResponse {
    fn write_to(&self, output: &mut StdoutLock) -> io::Result<()> {
        let message = serde_json::to_vec(self)?;
        output.write_u32::<NativeEndian>(message.len() as u32)?;
        output.write_all(&message)?;
        output.flush()
    }
}

fn main() {
    loop {
        let stdin = io::stdin();
        let stdout = io::stdout();
        let mut input = stdin.lock();
        let mut output = stdout.lock();

        let request = BrowserRequest::read_from(&mut input);
        match request {
            Ok(r) => {
                let response = BrowserResponse {
                    suggestions: vec!["1".to_string()],
                };
                response.write_to(&mut output).unwrap_or_else(|err| {
                    println!("Error handling request: {:?}", err);
                });
            }
            Err(e) => {
                let error = format!("Error: {:?}", e);
                let response = BrowserResponse {
                    suggestions: vec![error],
                };
                response.write_to(&mut output).unwrap_or_else(|err| {
                    println!("Error handling request: {:?}", err);
                });
            }
        }
    }
}
