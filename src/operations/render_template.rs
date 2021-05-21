use crate::operations::write_to_file::write_to_file;
use askama::Template;
use failure::Error;
use std::path::Path;

pub(crate) fn render_template<T: Template>(input: &T, path: &Path) -> Result<String, Error> {
    write_to_file(&path, input.render()?.as_bytes())
}
