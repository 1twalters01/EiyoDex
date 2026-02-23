use thiserror::Error;

#[derive(Debug, Error)]
    pub enum SpecificCurrencyUnitParseError {
    #[error("Specific currency unit has an invalid Format: '{input}'")]
    InvalidFormat { input: String },

    #[error("Unknown specific currency unit: '{input}'")]
    UnknownUnit { input: String },

    #[error("Unknown currency unit: '{input}'")]
    UnknownCurrencyUnit { input: String },

    #[error("Unknown denominator unit: '{input}'")]
    UnknownDenominatorUnit { input: String },
}
