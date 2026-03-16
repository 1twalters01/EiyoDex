use sqlx::{Pool, Sqlite};
use units::{energy::unit::EnergyUnit, mass::unit::MassUnit, volume::unit::VolumeUnit};

use crate::nutrient_units::NutrientUnit;

#[derive(Debug, PartialEq)]
pub struct NutrientUnitRecord {
    unit_type_id: i64,
    mass_type_id: Option<i64>,
    volume_type_id: Option<i64>,
    energy_type_id: Option<i64>,
}

impl NutrientUnitRecord {
    pub async fn from_values(unit_type_id: i64, mass_type_id: Option<i64>, volume_type_id: Option<i64>, energy_type_id: Option<i64>) -> Self {
        Self {
            unit_type_id,
            mass_type_id,
            volume_type_id,
            energy_type_id,
        }
    }

    pub async fn from_nutrient_unit(nutrient_unit: NutrientUnit, pool: &Pool<Sqlite>) -> Self {
        let unit_type_id: i64;
        let mut mass_type_id: Option<i64> = None;
        let mut volume_type_id: Option<i64> = None;
        let mut energy_type_id: Option<i64> = None;

        match nutrient_unit {
            NutrientUnit::Mass(mass) => {
                unit_type_id = 1;
                mass_type_id = Some(mass.get_database_id(pool).await.unwrap());
            }
            NutrientUnit::Volume(volume) => {
                unit_type_id = 2;
                volume_type_id = Some(volume.get_database_id(pool).await.unwrap());
            }
            NutrientUnit::Energy(energy) => {
                unit_type_id = 3;
                energy_type_id = Some(energy.get_database_id(pool).await.unwrap());
            }
            NutrientUnit::IU => unit_type_id = 4,
            NutrientUnit::DFE => unit_type_id = 5,
            NutrientUnit::NE => unit_type_id = 6,
            NutrientUnit::RAE => unit_type_id = 7,
            NutrientUnit::PDCAAS => unit_type_id = 8,
            NutrientUnit::DIAAS1 => unit_type_id = 9,
            NutrientUnit::DIAAS2 => unit_type_id = 10,
            NutrientUnit::DIAAS3 => unit_type_id = 11,
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
            1 => {
                let mass =
                    MassUnit::from_database_id(self.mass_type_id.expect("Invalid mass unit"), pool)
                        .await
                        .expect("Mass not found");
                NutrientUnit::Mass(mass)
            }
            2 => {
                let volume =
                    VolumeUnit::from_database_id(self.volume_type_id.expect("Invalid volume unit"), pool)
                        .await
                        .expect("Volume not found");
                NutrientUnit::Volume(volume)
            }
            3 => {
                let energy =
                    EnergyUnit::from_database_id(self.energy_type_id.expect("Invalid energy unit"), pool)
                        .await
                        .expect("Energy not found");
                NutrientUnit::Energy(energy)
            }
            4 => NutrientUnit::IU,
            5 => NutrientUnit::DFE,
            6 => NutrientUnit::NE,
            7 => NutrientUnit::RAE,
            8 => NutrientUnit::PDCAAS,
            9 => NutrientUnit::DIAAS1,
            10 => NutrientUnit::DIAAS2,
            11 => NutrientUnit::DIAAS3,
            _ => panic!("Invalid unit type id"),
        }
    }

    pub async fn save_enumerations_to_database(pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let mut tx = pool.begin().await?;

        let enumerations = NutrientUnit::get_enumerations();

        for nutrient_unit in enumerations {
            let unit_record = NutrientUnitRecord::from_nutrient_unit(nutrient_unit.clone(), pool).await;

            let unit_type_id = unit_record.unit_type_id;
            let mass_type_id = unit_record.mass_type_id;
            let volume_type_id = unit_record.volume_type_id;
            let energy_type_id = unit_record.energy_type_id;

            sqlx::query!(
                r#"
                    INSERT OR IGNORE INTO nutrients_nutrient_units (unit_type_id, mass_type_id, volume_type_id, energy_type_id)
                    VALUES (?, ?, ?, ?)
                "#,
                unit_type_id,
                mass_type_id,
                volume_type_id,
                energy_type_id,
            )
                .execute(&mut *tx)
            .await?;
        }

        tx.commit().await?;
        return Ok(())
    }

    pub async fn load_from_database(id: i64, pool: &Pool<Sqlite>) -> Result<Self, sqlx::Error> {
        Ok(sqlx::query_as!(
            NutrientUnitRecord,
            r#"
                SELECT
                    unit_type_id,
                    mass_type_id,
                    volume_type_id,
                    energy_type_id
                FROM nutrients_nutrient_units
                WHERE
                    id = ?
            "#,
            id
        )
            .fetch_one(pool)
            .await?)
    }

    pub async fn delete_from_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM nutrients_nutrient_units
                WHERE
                    unit_type_id = ?
                    AND mass_type_id IS ?
                    AND volume_type_id IS ?
                    AND energy_type_id IS ?
            "#,
            self.unit_type_id,
            self.mass_type_id,
            self.volume_type_id,
            self.energy_type_id,
        ).execute(pool).await?;

        return Ok(())
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_units (unit_type_id, mass_type_id, volume_type_id, energy_type_id)
                VALUES (?, ?, ?, ?)
            "#,
            self.unit_type_id,
            self.mass_type_id,
            self.volume_type_id,
            self.energy_type_id,
        ).execute(pool).await?;

        Ok(())
    }

    pub async fn get_database_id(&self, pool: &Pool<Sqlite>) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT id 
                FROM nutrients_nutrient_units
                WHERE
                    unit_type_id IS ?
                    AND mass_type_id IS ?
                    AND volume_type_id IS ?
                    AND energy_type_id IS ?
            "#,
            self.unit_type_id,
            self.mass_type_id,
            self.volume_type_id,
            self.energy_type_id,
        ).fetch_one(pool).await?.id)
    }

    pub async fn add_unit_type(name: String, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                INSERT OR IGNORE INTO nutrients_unit_types (name)
                VALUES (?)
            "#,
            name,
        )
            .execute(pool)
            .await?;

        return Ok(())
    }

    pub async fn get_unit_type_database_id(name: String, pool: &Pool<Sqlite>) -> Result<i64, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT id
                FROM nutrients_unit_types 
                WHERE
                    name = ?
            "#,
            name
        ).fetch_one(pool).await?.id)
    }

    pub async fn load_unit_type_from_database(id: i64, pool: &Pool<Sqlite>) -> Result<String, sqlx::Error> {
        Ok(sqlx::query!(
            r#"
                SELECT name
                FROM nutrients_unit_types
                WHERE
                    id = ?
            "#,
            id
        ).fetch_one(pool).await?.name)
    }

    pub async fn delete_unit_type_from_database(name: String, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM nutrients_unit_types 
                WHERE
                name = ?
            "#,
            name,
        )
            .execute(pool)
            .await?;

        return Ok(())
    }
}
