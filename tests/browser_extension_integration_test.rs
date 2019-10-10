use resume::{run, Integration};
use std::fs;

#[test]
fn it_generates_correct_html_file() {
    let config = "examples/resume.yaml";
    let run_result = run(Integration::BrowserExtension, &config);
    assert!(run_result.is_ok());
    let result = fs::read_to_string("browser-extension/sidebar/panel.html");
    let expected = fs::read_to_string("tests/fixtures/expected.html");
    assert!(result.is_ok() && expected.is_ok());
    assert_eq!(result.ok(), expected.ok())
}
