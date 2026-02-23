use thiserror::Error;

#[derive(Debug, Error)]
pub enum MassUnitParseError {
    #[error("Unknown mass unit: '{input}'")]
    UnknownUnit { input: String },
}
