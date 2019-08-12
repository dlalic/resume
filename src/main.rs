fn main() {
    if let Err(e) = resume::run("examples/resume.yaml".to_string()) {
        println!("{:?}", e);
    }
}
