use core::fmt;
use serde::Deserialize;

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
