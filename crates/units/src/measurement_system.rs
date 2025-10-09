#[macro_export]
macro_rules! define_measurement_systems {
    ($($variant:ident),+) => {
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum MeasurementSystem {
            $($variant),+
        }
        pub fn get_enumerations() -> &'static [MeasurementSystem] {
            &[$(MeasurementSystem::$variant),+]
        }

        impl MeasurementSystem {
            pub fn get_enumerations() -> Vec<MeasurementSystem> {
                Vec::from([$(MeasurementSystem::$variant),+])
            }
        }
    };
}

use measurement_system_macro::include_measurement_systems_from_json;
include_measurement_systems_from_json!(
    MassUnit => "data/units/mass",
    VolumeUnit => "data/units/volume",
    EnergyUnit => "data/units/energy",
    DistanceUnit => "data/units/distance",
    DurationUnit => "data/units/duration",
);
