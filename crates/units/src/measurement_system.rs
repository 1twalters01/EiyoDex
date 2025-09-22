#[macro_export]
macro_rules! define_measurement_systems {
    ($($variant:ident),+) => {
        #[cfg(feature = "serde")]
        use serde::{Deserialize, Serialize};

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        pub enum MeasurementSystem {
            $($variant),+
        }

        impl MeasurementSystem {
            pub fn get_enumerations() -> Vec<MeasurementSystem> {
                Vec::from([$(MeasurementSystem::$variant),+])
            }
        }
    };
}

use measurement_system_macro::include_measurement_systems_from_json;
include_measurement_systems_from_json!("data/mass.json");

