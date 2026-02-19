use crate::measurement_system::MeasurementSystem;

#[derive(Debug, PartialEq)]
pub struct PowerMeasurementSystem {
    energy_measurement_system: MeasurementSystem,
    duration_measurement_system: MeasurementSystem,
}

impl PowerMeasurementSystem {
    pub fn new(
        energy_measurement_system: MeasurementSystem,
        duration_measurement_system: MeasurementSystem,
    ) -> PowerMeasurementSystem {
        Self {
            energy_measurement_system,
            duration_measurement_system,
        }
    }

    pub fn get_energy_measurement_system(&self) -> MeasurementSystem {
        self.energy_measurement_system
    }

    pub fn get_duration_measurement_system(&self) -> MeasurementSystem {
        self.duration_measurement_system
    }
}
