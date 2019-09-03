use failure::Error;
use std::process::Command;

pub(crate) fn build_extension() -> Result<(String), Error> {
    let output = Command::new("cargo")
        .arg("build")
        .arg("--release")
        .arg("--bin")
        .arg("browser-extension")
        .output()?;
    Ok(output.status.to_string())
}
