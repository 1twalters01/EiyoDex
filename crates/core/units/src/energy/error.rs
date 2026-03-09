use thiserror::Error;

#[derive(Debug, Error)]
pub enum EnergyUnitParseError {
    #[error("Unknown energy unit: '{input}'")]
    UnknownUnit { input: String },
}
