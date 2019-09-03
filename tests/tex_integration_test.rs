use resume::{run, Integration};
use std::fs;

#[test]
fn it_generates_correct_tex_file() {
    let config = "examples/resume.yaml";
    let run_result = run(Integration::Tex, &config);
    assert!(run_result.is_ok());
    let result = fs::read_to_string("output.tex");
    let expected = fs::read_to_string("tests/fixtures/expected.tex");
    assert!(result.is_ok() && expected.is_ok());
    assert_eq!(result.ok(), expected.ok())
}
