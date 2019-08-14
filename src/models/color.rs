use core::fmt;
use serde::Deserialize;
use std::error;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Debug, Deserialize)]
pub struct Color {
    pub(crate) red: u8,
    pub(crate) green: u8,
    pub(crate) blue: u8,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.red, self.green, self.blue)
    }
}

impl FromStr for Color {
    type Err = ParseColorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: [u8; 3] = [ 0; 3 ];
        for (i, c) in s.split_terminator(",").enumerate() {
            match i {
                0|1|2 => values[i] = u8::from_str(c.trim())?,
                _ => return Err(ParseColorError::WrongNumberOfComponents)
            }
        }
        let color: Color = Color{red: values[0], green: values[1], blue: values[2]};
        Ok(color)
    }
}

#[derive(Debug)]
pub enum ParseColorError {
    WrongNumberOfComponents,
    Parse(ParseIntError)
}

impl fmt::Display for ParseColorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseColorError::WrongNumberOfComponents =>
                write!(f, "wrong number of color components, 3 expected"),
            ParseColorError::Parse(ref e) => e.fmt(f),
        }
    }
}

impl error::Error for ParseColorError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match *self {
            ParseColorError::WrongNumberOfComponents => None,
            ParseColorError::Parse(ref e) => Some(e),
        }
    }
}

impl From<ParseIntError> for ParseColorError {
    fn from(err: ParseIntError) -> ParseColorError {
        ParseColorError::Parse(err)
    }
}