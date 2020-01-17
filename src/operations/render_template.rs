use crate::operations::write_to_file::write_to_file;
use askama::Template;
use failure::Error;
use std::path::PathBuf;

pub(crate) fn render_template<T: Template>(input: &T, path: &PathBuf) -> Result<String, Error> {
    write_to_file(&path, input.render()?.as_bytes())
}
