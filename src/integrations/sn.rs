use crate::integrations::execute::Execute;
use crate::models::resume::Resume;
use std::error::Error;

pub(crate) struct SnIntegration {}

impl Execute for SnIntegration {
    fn execute(_: &Resume) -> Result<(String), Box<dyn Error>> {
        // TODO:
        // Build the browser-extension target with `cargo build --release --bin browser-extension`
        // Edit native-manifest.json file so that path points to absolute path of the exec above
        // Copy the manifest file to https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_manifests#Manifest_location
        unimplemented!()
    }
}
