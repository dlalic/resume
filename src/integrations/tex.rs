use crate::models::resume::Resume;
use askama::Template;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn execute(filename: String) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(filename)?;
    let resume: Resume = serde_yaml::from_str(&contents).unwrap();
    let mut file = File::create("output.tex")?;
    match file.write_all(&resume.render().unwrap().as_bytes()) {
        Err(why) => panic!("Couldn't write to disk: {}", why.description()),
        Ok(_) => println!("Done!"),
    }
    Ok(())
}