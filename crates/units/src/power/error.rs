use thiserror::Error;

#[derive(Debug, Error)]
pub enum PowerUnitParseError {
    #[error("Unknown power unit: '{input}'")]
    UnknownUnit { input: String },
}
