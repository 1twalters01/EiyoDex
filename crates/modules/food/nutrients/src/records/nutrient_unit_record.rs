use sqlx::{Pool, Sqlite};
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};
use utils::database::DatabaseService;

use crate::nutrient_units::NutrientUnit;

pub struct NutrientUnitRecord {
    unit_type_id: Option<i64>,
    mass_type_id: Option<i64>,
    volume_type_id: Option<i64>,
    energy_type_id: Option<i64>,
}

impl NutrientUnitRecord {
    pub async fn from_nutrient_unit(nutrient_unit: NutrientUnit, pool: &Pool<Sqlite>) -> Self {
        let unit_type_id: Option<i64>;
        let mut mass_type_id: Option<i64> = None;
        let mut volume_type_id: Option<i64> = None;
        let mut energy_type_id: Option<i64> = None;

        match nutrient_unit {
            NutrientUnit::Mass(mass) => {
                unit_type_id = Some(1);
                mass_type_id = Some(mass.get_database_id(pool).await.unwrap());
            }
            NutrientUnit::Volume(volume) => {
                unit_type_id = Some(2);
                volume_type_id = Some(volume.get_database_id(pool).await.unwrap());
            }
            NutrientUnit::Energy(energy) => {
                unit_type_id = Some(3);
                energy_type_id = Some(energy.get_database_id(pool).await.unwrap());
            }
            NutrientUnit::IU => unit_type_id = Some(4),
            NutrientUnit::DFE => unit_type_id = Some(5),
            NutrientUnit::NE => unit_type_id = Some(6),
            NutrientUnit::RAE => unit_type_id = Some(7),
            NutrientUnit::PDCAAS => unit_type_id = Some(8),
            NutrientUnit::DIAAS1 => unit_type_id = Some(9),
            NutrientUnit::DIAAS2 => unit_type_id = Some(10),
            NutrientUnit::DIAAS3 => unit_type_id = Some(11),
            NutrientUnit::None => unit_type_id = None,
        };

        Self {
            unit_type_id,
            mass_type_id,
            volume_type_id,
            energy_type_id,
        }
    }

    pub async fn to_nutrient_unit(&self, pool: &Pool<Sqlite>) -> NutrientUnit {
        match self.unit_type_id {
            Some(1) => {
                let mass =
                    MassUnit::from_database_id(self.mass_type_id.expect("Invalid mass unit"), pool)
                        .await
                        .expect("Mass not found");
                NutrientUnit::Mass(mass)
            }
            Some(2) => {
                let volume =
                    VolumeUnit::from_database_id(self.volume_type_id.expect("Invalid volume unit"), pool)
                        .await
                        .expect("Volume not found");
                NutrientUnit::Volume(volume)
            }
            Some(3) => {
                let energy =
                    EnergyUnit::from_database_id(self.energy_type_id.expect("Invalid energy unit"), pool)
                        .await
                        .expect("Energy not found");
                NutrientUnit::Energy(energy)
            }
            Some(4) => NutrientUnit::IU,
            Some(5) => NutrientUnit::DFE,
            Some(6) => NutrientUnit::NE,
            Some(7) => NutrientUnit::RAE,
            Some(8) => NutrientUnit::PDCAAS,
            Some(9) => NutrientUnit::DIAAS1,
            Some(10) => NutrientUnit::DIAAS2,
            Some(11) => NutrientUnit::DIAAS3,
            None => NutrientUnit::None,
            _ => panic!("Invalid unit type id"),
        }
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

        sqlx::query!(
            r#"
                INSERT INTO nutrients_unit_table (unit_type_id, mass_type_id, volume_type_id, energy_type_id)
                VALUES (?, ?, ?, ?)
            "#,
            self.unit_type_id,
            self.mass_type_id,
            self.volume_type_id,
            self.energy_type_id,
        ).execute(pool).await?;

        Ok(())
    }

    pub fn get_unit_type_id(&self) -> Option<i64> {
        self.unit_type_id
    }

    pub async fn load_from_database(id: i64, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

        Ok(sqlx::query_as!(
            NutrientUnitRecord,
            r#"
                    SELECT
                        id as unit_type_id,
                        mass_type_id,
                        volume_type_id,
                        energy_type_id
                    FROM nutrients_unit_table
                    WHERE
                        id = ?
                "#,
            id
        )
        .fetch_one(pool)
        .await?)
    }
}
