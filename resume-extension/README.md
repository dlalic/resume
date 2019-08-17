# Resume extension (WIP)

## Installation

Open Firefox and navigate to `about:debugging`. Click the "Load Temporary Add-on..." button and select the `manifest.json` file from within this directory.

## Reasons against [native messaging](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_messaging)

- this is not meant to be a production plugin and the user is required to install a [manifest](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/Native_manifests#Manifest_location) on the file system, likely not to be cleaned up afterwards

## Linters

This project uses:

- [standard](https://github.com/standard/standard)

- [addons-linter](https://github.com/mozilla/addons-linter)