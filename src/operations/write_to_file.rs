use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::Path;

pub(crate) fn write_to_file(path: &Path, contents: &[u8]) -> Result<String> {
    let mut file = File::create(&path)?;
    file.write_all(contents)?;
    Ok(format!("Wrote {}", path.display()))
}
