use anyhow::anyhow;
use core::fmt;
use serde::Deserialize;
use std::str::FromStr;

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
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut values: [u8; 3] = [0; 3];
        let mut num_values = 0;
        for (i, c) in s.split_terminator(',').enumerate() {
            num_values += 1;
            match i {
                0..=2 => values[i] = u8::from_str(c.trim())?,
                _ => return Err(anyhow!("too many color components, expected 3")),
            }
        }
        match num_values {
            3 => Ok(Color {
                red: values[0],
                green: values[1],
                blue: values[2],
            }),
            _ => Err(anyhow!("too few color components, expected 3")),
        }
    }
}
