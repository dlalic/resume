use std::error::Error;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

pub(crate) fn write_to_file(path: &PathBuf, contents: &[u8]) -> Result<(String), Box<dyn Error>> {
    let mut file = File::create(&path)?;
    file.write_all(contents)?;
    Ok(format!("Wrote {}", path.display()))
}
