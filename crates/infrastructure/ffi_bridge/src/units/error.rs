#[derive(Debug, Copy, Clone)]
pub enum UnitConversionError {
    InvalidMassUnitValue,
    InvalidVolumeUnitValue,
    InvalidEnergyUnitValue,
}
