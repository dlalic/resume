use core::fmt;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Color {
    pub(crate) red: u32,
    pub(crate) green: u32,
    pub(crate) blue: u32,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}, {}, {}", self.red, self.green, self.blue)
    }
}
