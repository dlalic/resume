use byteorder::{NativeEndian, ReadBytesExt};
use serde::Deserialize;
use std::io::Read;
use std::io::StdinLock;
use std::{error, fmt};

#[derive(Debug)]
pub(crate) enum InputError {
    IOError,
    JSONError,
}

impl fmt::Display for InputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InputError::IOError => write!(f, "IO"),
            InputError::JSONError => write!(f, "JSON"),
        }
    }
}

impl error::Error for InputError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            InputError::IOError => None,
            InputError::JSONError => None,
        }
    }
}

#[derive(Deserialize)]
pub(crate) struct Input {
    #[serde(rename = "id")]
    pub(crate) element_id: String,
    #[serde(rename = "type")]
    pub(crate) element_type: String,
    #[serde(rename = "name")]
    pub(crate) element_name: String,
}

impl Input {
    pub(crate) fn read_from(input: &mut StdinLock) -> Result<Input, InputError> {
        let length = input
            .read_u32::<NativeEndian>()
            .map_err(|_err| InputError::IOError)?;
        let mut message = input.take(length as u64);
        let mut buffer = Vec::with_capacity(length as usize);
        message
            .read_to_end(&mut buffer)
            .map_err(|_err| InputError::IOError)?;
        serde_json::from_slice(&buffer).map_err(|_err| InputError::JSONError)
    }
}
