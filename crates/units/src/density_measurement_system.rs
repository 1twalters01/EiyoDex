use crate::measurement_system::MeasurementSystem;
use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq)]
pub struct DensityMeasurementSystem {
    mass_measurement_system: MeasurementSystem,
    volume_measurement_system: MeasurementSystem,
}

impl DensityMeasurementSystem {
    pub fn new(
        mass_measurement_system: MeasurementSystem,
        volume_measurement_system: MeasurementSystem,
    ) -> DensityMeasurementSystem {
        Self {
            mass_measurement_system,
            volume_measurement_system,
        }
    }

    pub fn get_mass_measurement_system(&self) -> MeasurementSystem {
        self.mass_measurement_system
    }

    pub fn get_volume_measurement_system(&self) -> MeasurementSystem {
        self.volume_measurement_system
    }
}
