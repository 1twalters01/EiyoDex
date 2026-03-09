use thiserror::Error;

#[derive(Debug, Error)]
pub enum DistanceUnitParseError {
    #[error("Unknown distance unit: '{input}'")]
    UnknownUnit { input: String },
}
