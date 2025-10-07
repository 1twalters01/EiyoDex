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
    MassUnit => "data/units/mass/mass.json", "data/units/mass/fake_mass.json",
    VolumeUnit => "data/units/volume/volume.json", "data/units/volume/fake_volume.json",
    EnergyUnit => "data/units/energy/energy.json", "data/units/energy/fake_energy.json",
    DistanceUnit => "data/units/distance/distance.json", "data/units/distance/fake_distance.json",
);
