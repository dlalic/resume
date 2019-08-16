use crate::models::resume::Resume;
use askama::Template;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn execute(filename: &str) -> Result<(String), Box<dyn Error>> {
    let output = "output.tex";
    let contents = fs::read_to_string(filename)?;
    let resume: Resume = serde_yaml::from_str(&contents)?;
    let mut file = File::create(output)?;
    file.write_all(&resume.render()?.as_bytes())?;
    Ok(output.to_string())
}
