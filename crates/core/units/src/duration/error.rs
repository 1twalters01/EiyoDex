use thiserror::Error;

#[derive(Debug, Error)]
pub enum DurationUnitParseError {
    #[error("Unknown duration unit: '{input}'")]
    UnknownUnit { input: String },
}
