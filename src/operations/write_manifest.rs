use crate::models::native_manifest::NativeManifest;
use crate::operations::write_to_file::write_to_file;
use failure::Error;
use std::path::PathBuf;

fn determine_manifest_path() -> PathBuf {
    let home_dir = dirs::home_dir().expect("Home directory not found.");
    let mut path = PathBuf::from(home_dir);
    // https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_manifests#Manifest_location
    if cfg!(target_os = "linux") {
        let local = r".mozilla/native-messaging-hosts";
        path.push(local);
    } else if cfg!(target_os = "windows") {
        let local = r"SOFTWARE\Mozilla\NativeMessagingHosts";
        path.push(local);
    } else if cfg!(target_os = "macos") {
        let local = r"Library/Application Support/Mozilla/NativeMessagingHosts";
        path.push(local);
    };
    path.push("resume");
    path.set_extension("json");
    path
}

fn determine_executable_path() -> Result<(String), Error> {
    let path: PathBuf = ["target", "release", "browser-extension"].iter().collect();
    let abs_path = path.canonicalize()?;
    let into = abs_path.into_os_string();
    match into.into_string() {
        Ok(result) => Ok(result),
        Err(_) => Err(format_err!("Cannot convert path {} to string.", path.display())),
    }
}

pub(crate) fn write_manifest() -> Result<(String), Error> {
    let path = determine_manifest_path();
    let mut manifest = NativeManifest::default();
    manifest.path = determine_executable_path()?;
    let json = serde_json::to_string(&manifest)?;
    write_to_file(&path, json.as_ref())
}