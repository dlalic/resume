use crate::operations::write_to_file::write_to_file;
use askama::Template;
use std::error::Error;
use std::path::PathBuf;

pub(crate) fn render_template<T: Template>(
    input: &T,
    path: &PathBuf,
) -> Result<(String), Box<dyn Error>> {
    write_to_file(&path, input.render()?.as_bytes())
}
