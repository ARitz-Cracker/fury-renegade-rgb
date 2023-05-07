use thiserror::Error;

#[derive(Error, Debug)]
pub enum FuryControllerError {
    #[error("Couldn't parse {0} as a known colour pattern style")]
    UnknownPatternStyle(String),
}
