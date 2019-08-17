use crate::integrations::tex::execute;
use std::error::Error;

mod integrations;
// TODO: The module shouldn't be public,
//       but #[cfg(bench)] doesn't seem to do the trick.
pub mod models;
mod serializers;

#[derive(Debug)]
pub enum Integration {
    Tex,
    Xing
}

pub fn run(int : Integration, config: &str) -> Result<String, Box<dyn Error>> {
    match int {
        Integration::Tex => execute(config),
        Integration::Xing => unimplemented!()
    }
}
