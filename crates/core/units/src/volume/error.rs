use thiserror::Error;

#[derive(Debug, Error)]
pub enum VolumeUnitParseError {
    #[error("Unknown volume unit: '{input}'")]
    UnknownUnit { input: String },
}
