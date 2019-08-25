use crate::integrations::execute::Execute;
use crate::models::resume::Resume;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;
extern crate dirs;
pub(crate) struct SnIntegration {}

impl Execute for SnIntegration {
    fn execute(_: &Resume) -> Result<(String), Box<dyn Error>> {
        SnIntegration::build_extension()?;
        let result = SnIntegration::load_and_edit_manifest()?;
        SnIntegration::dump_manifest(&result)
    }
}

impl SnIntegration {
    fn build_extension() -> Result<(String), Box<dyn Error>> {
        let output = Command::new("cargo")
            .arg("build")
            .arg("--release")
            .arg("--bin")
            .arg("browser-extension")
            .output()?;
        Ok(output.status.to_string())
    }

    fn load_and_edit_manifest() -> Result<(String), Box<dyn Error>> {
        let contents = fs::read_to_string("browser-extension/native-manifest.json")?;
        let path = Path::new("./target/release/browser-extension");
        let abs_path = path.canonicalize()?;
        let error = format!("Cannot convert path {} to string.", path.display());
        let os_str = abs_path.into_os_string().into_string().expect(&error);
        Ok(contents.replace("/path/to/executable", &os_str))
    }

    fn dump_manifest(manifest: &String) -> Result<(String), Box<dyn Error>> {
        let home_dir = dirs::home_dir().expect("Home directory not found.");
        let mut path = PathBuf::from(home_dir);
        if cfg!(target_os = "linux") {
            let local: PathBuf = [".mozilla", "native-messaging-hosts"].iter().collect();
            path.push(local);
        } else if cfg!(target_os = "windows") {
            let local: PathBuf = ["SOFTWARE", "Mozilla", "NativeMessagingHosts"]
                .iter()
                .collect();
            path.push(local);
        } else if cfg!(target_os = "macos") {
            let local: PathBuf = [
                "Library",
                "Application Support",
                "Mozilla",
                "NativeMessagingHosts",
            ]
            .iter()
            .collect();
            path.push(local);
        };
        path.push("resume");
        path.set_extension("json");
        let mut file = File::create(&path)?;
        file.write_all(manifest.as_ref())?;
        Ok(format!("Wrote manifest to: {}", path.display()))
    }
}
