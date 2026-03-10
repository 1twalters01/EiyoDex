use sqlx::{Pool, Sqlite};

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

            pub async fn save_enumerations_to_database(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
                let enumerations = MeasurementSystem::get_enumerations();
                for measurement_system in enumerations {
                    let name = measurement_system.as_string();
                    sqlx::query!(
                        r#"
                            INSERT OR IGNORE INTO units_measurement_systems (name)
                            VALUES (?)
                        "#,
                        name
                    )
                    .execute(pool)
                    .await?;
                }
                return Ok(())
            }

            pub async fn get_database_id(&self, pool: &Pool<Sqlite>) -> Result<Option<i64>, sqlx::Error> {
                let name = self.as_string();
                let row = sqlx::query!(
                    r#"
                        SELECT id 
                        FROM units_measurement_systems
                        WHERE name = ?
                    "#,
                    name
                )
                .fetch_one(pool)
                .await?;
                Ok(row.id)
            }

            // pub async fn from_database_id(id: i64, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
            //     let row = sqlx::query!(
            //         r#"
            //             SELECT name
            //             FROM units_measurement_systems
            //             WHERE id = ?
            //         "#,
            //         id
            //     )
            //     .fetch_one(pool)
            //     .await?;
            //
            //     // FIX THIS
            //     Ok(Self::from_str(&row.name).unwrap())
            // }
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
