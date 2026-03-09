use utils::database::DatabaseService;

#[macro_export]
macro_rules! define_measurement_systems {
    ($($variant:ident),+) => {

        #[derive(Debug, ::serde::Serialize, ::serde::Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
        pub enum MeasurementSystem {
            $($variant),+
        }

        impl MeasurementSystem {
            pub fn get_enumerations() -> &'static [MeasurementSystem] {
                &[$(MeasurementSystem::$variant),+]
            }

            pub fn as_string(&self) -> String {
                match self {
                    $(
                        MeasurementSystem::$variant => stringify!($variant).to_string(),
                    )+
                }
            }

            pub async fn save_to_database() -> Result<(), sqlx::Error> {
                let enumerations = MeasurementSystem::get_enumerations();
                let database_service = DatabaseService::new().await?;
                for measurement_system in enumerations {
                    let name = measurement_system.as_string();
                    sqlx::query!(
                        r#"
                            INSERT OR IGNORE INTO units_measurement_systems (name)
                            VALUES (?)
                        "#,
                        name
                    )
                    .execute(&database_service.pool)
                    .await?;
                }
                return Ok(())
            }

            pub async fn get_database_id(&self) -> Result<Option<i64>, sqlx::Error> {
                let database_service = DatabaseService::new().await?;
                let name = self.as_string();
                let row = sqlx::query!(
                    r#"
                        SELECT id 
                        FROM units_measurement_systems
                        WHERE name = ?
                    "#,
                    name
                )
                .fetch_one(&database_service.pool)
                .await?;
                Ok(row.id)
            }

        }
    };
}

use units_macro::include_measurement_systems_from_json;
include_measurement_systems_from_json!(
    MassUnit => "data/units/mass",
    VolumeUnit => "data/units/volume",
    EnergyUnit => "data/units/energy",
    DistanceUnit => "data/units/distance",
    DurationUnit => "data/units/duration",
);
