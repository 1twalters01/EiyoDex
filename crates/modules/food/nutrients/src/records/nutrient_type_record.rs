use sqlx::{Pool, Sqlite};

use crate::schema::{
    carbohydrate::{Carbohydrate, CarbohydrateNutrient},
    energy::EnergyYieldingNutrients,
    lipid::{Fat, Lipid, LipidNutrient, Sterol, TransFat},
    nutrient_classes::{ChemicalType, EssentialityType, QuantityType},
    nutrient_type::NutrientType,
    protein::ProteinNutrient,
};

#[derive(Debug, PartialEq)]
pub struct NutrientTypeRecord {
    essentiality_type_id: Option<i64>,
    quantity_type_id: i64,
    chemical_type_id: i64,

    energy_type_id: Option<i64>,

    carbohydrate_type_id: Option<i64>,

    is_bcaa: Option<bool>,

    lipid_type_id: Option<i64>,
    sterol_type_id: Option<i64>,
    fat_type_id: Option<i64>,
    transfat_type_id: Option<i64>,
}

impl NutrientTypeRecord {
    pub fn from_values(
        essentiality_type_id: Option<i64>,
        quantity_type_id: i64,
        chemical_type_id: i64,
        energy_type_id: Option<i64>,
        carbohydrate_type_id: Option<i64>,
        is_bcaa: Option<bool>,
        lipid_type_id: Option<i64>,
        sterol_type_id: Option<i64>,
        fat_type_id: Option<i64>,
        transfat_type_id: Option<i64>,
    ) -> Self {
        Self {
            essentiality_type_id,
            quantity_type_id,
            chemical_type_id,
            energy_type_id,
            carbohydrate_type_id,
            is_bcaa,
            lipid_type_id,
            sterol_type_id,
            fat_type_id,
            transfat_type_id,
        }
    }

    pub fn from_nutrient_type(nutrient_type: NutrientType) -> Self {
        let quantity_type_id = match nutrient_type.quantity_type {
            QuantityType::Macronutrient => 1,
            QuantityType::Micronutrient => 2,
            QuantityType::NonNutrient => 3,
        };

        let essentiality_type_id = match nutrient_type.essentiality_type {
            Some(EssentialityType::Essential) => Some(1),
            Some(EssentialityType::ConditionallyEssential) => Some(2),
            Some(EssentialityType::NonEssential) => Some(3),
            None => None,
        };

        let chemical_type_id: i64;
        let mut energy_type_id = None;
        let mut carbohydrate_type_id = None;
        let mut is_bcaa = None;
        let mut lipid_type_id = None;
        let mut sterol_type_id = None;
        let mut fat_type_id = None;
        let mut transfat_type_id = None;

        match nutrient_type.chemical_type {
            ChemicalType::EnergyYieldingNutrients(nutrient) => {
                chemical_type_id = 1;
                match nutrient {
                    EnergyYieldingNutrients::Carbohydrate(carbohydrate) => {
                        energy_type_id = Some(1);
                        carbohydrate_type_id = match carbohydrate.carbohydrate_type {
                            Carbohydrate::Fiber => Some(1),
                            Carbohydrate::Starch => Some(2),
                            Carbohydrate::Sugar => Some(3),
                            Carbohydrate::SugarAlcohol => Some(4),
                        }
                    }
                    EnergyYieldingNutrients::Protein(protein) => {
                        energy_type_id = Some(2);
                        is_bcaa = Some(protein.is_bcaa);
                    }
                    EnergyYieldingNutrients::Lipid(lipid) => {
                        energy_type_id = Some(3);
                        match lipid.lipid_type {
                            Lipid::Sterols(sterol) => {
                                lipid_type_id = Some(1);
                                match sterol {
                                    Sterol::Cholesterol => sterol_type_id = Some(1),
                                    Sterol::Phytosterol => sterol_type_id = Some(2),
                                }
                            }
                            Lipid::Fats(fat) => {
                                lipid_type_id = Some(2);
                                match fat {
                                    Fat::Monounsaturated => fat_type_id = Some(1),
                                    Fat::Polyunsaturated => fat_type_id = Some(2),
                                    Fat::Saturated => fat_type_id = Some(3),
                                }
                            }
                            Lipid::TransFats(transfat) => {
                                lipid_type_id = Some(3);
                                match transfat {
                                    TransFat::Natural => transfat_type_id = Some(1),
                                    TransFat::Artificial => transfat_type_id = Some(2),
                                }
                            }
                            Lipid::Phospholipid => lipid_type_id = Some(4),
                        }
                    }
                    EnergyYieldingNutrients::Alcohol => energy_type_id = Some(4),
                }
            }
            ChemicalType::Water => chemical_type_id = 2,
            ChemicalType::Vitamin => chemical_type_id = 3,
            ChemicalType::Mineral => chemical_type_id = 4,
            ChemicalType::Phytonutrient => chemical_type_id = 5,
            ChemicalType::Antinutrient => chemical_type_id = 6,
            ChemicalType::Other => chemical_type_id = 7,
        };

        Self {
            quantity_type_id,
            essentiality_type_id,
            chemical_type_id,
            energy_type_id,
            carbohydrate_type_id,
            is_bcaa,
            lipid_type_id,
            sterol_type_id,
            fat_type_id,
            transfat_type_id,
        }
    }

    pub fn to_nutrient_type(&self) -> NutrientType {
        let chemical_type = match self.chemical_type_id {
            1 => match self.energy_type_id.unwrap() {
                1 => {
                    let carbohydrate_type = match self.carbohydrate_type_id.unwrap() {
                        1 => Carbohydrate::Fiber,
                        2 => Carbohydrate::Starch,
                        3 => Carbohydrate::Sugar,
                        4 => Carbohydrate::SugarAlcohol,
                        _ => panic!("Unknown carbohydrate_id"),
                    };
                    ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(
                        CarbohydrateNutrient { carbohydrate_type },
                    ))
                }
                2 => ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(
                    ProteinNutrient {
                        is_bcaa: self.is_bcaa.unwrap(),
                    },
                )),
                3 => {
                    let sterol = match self.sterol_type_id {
                        Some(1) => Some(Lipid::Sterols(Sterol::Cholesterol)),
                        Some(2) => Some(Lipid::Sterols(Sterol::Phytosterol)),
                        None => None,
                        _ => panic!("Unknown sterol"),
                    };
                    let fat = match self.fat_type_id {
                        Some(1) => Some(Lipid::Fats(Fat::Monounsaturated)),
                        Some(2) => Some(Lipid::Fats(Fat::Polyunsaturated)),
                        Some(3) => Some(Lipid::Fats(Fat::Saturated)),
                        None => None,
                        _ => panic!("Unknown fat"),
                    };
                    let transfat = match self.transfat_type_id {
                        Some(1) => Some(Lipid::TransFats(TransFat::Natural)),
                        Some(2) => Some(Lipid::TransFats(TransFat::Artificial)),
                        None => None,
                        _ => panic!("Unknown transfat"),
                    };
                    let values: Vec<Lipid> =
                        [sterol, fat, transfat].iter().filter_map(|x| *x).collect();

                    let lipid_type = match values.len() {
                        0 => panic!("Lipid was not found"),
                        1 => values[0],
                        _ => panic!("More than one type of lipid found"),
                    };

                    let lipid = LipidNutrient { lipid_type };
                    ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(lipid))
                }
                4 => ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Alcohol),
                _ => panic!("Unknown energy_id"),
            },
            2 => ChemicalType::Water,
            3 => ChemicalType::Vitamin,
            4 => ChemicalType::Mineral,
            5 => ChemicalType::Phytonutrient,
            6 => ChemicalType::Antinutrient,
            7 => ChemicalType::Other,
            _ => panic!("Unknown chemical_id"),
        };

        let quantity_type = match self.quantity_type_id {
            1 => QuantityType::Macronutrient,
            2 => QuantityType::Micronutrient,
            3 => QuantityType::NonNutrient,
            _ => panic!("Unknown quantity_id"),
        };

        let essentiality_type = match self.essentiality_type_id {
            Some(1) => Some(EssentialityType::Essential),
            Some(2) => Some(EssentialityType::ConditionallyEssential),
            Some(3) => Some(EssentialityType::NonEssential),
            None => None,
            Some(_) => panic!("Unknown essentiality_id"),
        };

        NutrientType::new(essentiality_type, quantity_type, chemical_type)
    }

    pub async fn save_to_database(&self, pool: &Pool<Sqlite>) -> Result<(), sqlx::Error> {
        let chemical_id = match self.energy_type_id {
            Some(1) => {
                assert!(self.carbohydrate_type_id.is_some());
                let carbohydrate_id = match sqlx::query!(
                    r#"
                        INSERT INTO nutrients_carbohydrate_nutrients (carbohydrate_type_id)
                        VALUES (?)
                        RETURNING id
                    "#,
                    self.carbohydrate_type_id
                )
                .fetch_optional(pool)
                .await?
                {
                    Some(row) => row.id,
                    None => sqlx::query!(
                        r#"
                            SELECT id FROM nutrients_carbohydrate_nutrients
                            WHERE carbohydrate_type_id = ?
                        "#,
                        self.carbohydrate_type_id
                    ).fetch_one(pool).await?.id
                };

                let energy_id = sqlx::query!(
                    r#"
                        INSERT INTO nutrients_energy_yielding_nutrients (energy_yielding_nutrient_type_id, carbohydrate_nutrient_id)
                        VALUES (?, ?)
                        RETURNING id
                    "#,
                    self.energy_type_id,
                    carbohydrate_id
                ).fetch_one(pool).await?.id;

                sqlx::query!(
                    r#"
                        INSERT INTO nutrients_chemical_type_table (chemical_type_id, energy_yielding_nutrient_id)
                        VALUES (?, ?)
                        RETURNING id
                    "#,
                    self.chemical_type_id,
                    energy_id
                ).fetch_one(pool).await?.id
            }
            Some(2) => {
                let protein_id = sqlx::query!(
                    r#"
                        INSERT INTO nutrients_protein_nutrients (is_bcaa)
                        VALUES (?)
                        RETURNING id
                    "#,
                    self.is_bcaa
                )
                .fetch_one(pool)
                .await?
                .id;

                let energy_id = sqlx::query!(
                    r#"
                        INSERT INTO nutrients_energy_yielding_nutrients (energy_yielding_nutrient_type_id, protein_nutrient_id)
                        VALUES (?, ?)
                        RETURNING id
                    "#,
                    self.energy_type_id,
                    protein_id
                ).fetch_one(pool).await?.id;

                sqlx::query!(
                    r#"
                        INSERT INTO nutrients_chemical_type_table (chemical_type_id, energy_yielding_nutrient_id)
                        VALUES (?, ?)
                        RETURNING id
                    "#,
                    self.chemical_type_id,
                    energy_id
                ).fetch_one(pool).await?.id
            }
            Some(3) => {
                let lipid_table_id = sqlx::query!(
                    r#"
                        INSERT INTO nutrients_lipid_table (lipid_type_id, sterol_type_id, fat_type_id, transfat_type_id)
                        VALUES (?, ?, ?, ?)
                        RETURNING id
                    "#,
                    self.lipid_type_id,
                    self.sterol_type_id,
                    self.fat_type_id,
                    self.transfat_type_id
                ).fetch_one(pool).await?.id;

                let lipid_id = sqlx::query!(
                    r#"
                        INSERT INTO nutrients_lipid_nutrients (lipid_id)
                        VALUES (?)
                        RETURNING id
                    "#,
                    lipid_table_id
                )
                .fetch_one(pool)
                .await?
                .id;

                let energy_id = sqlx::query!(
                    r#"
                        INSERT INTO nutrients_energy_yielding_nutrients (energy_yielding_nutrient_type_id, lipid_nutrient_id)
                        VALUES (?, ?)
                        RETURNING id
                    "#,
                    self.energy_type_id,
                    lipid_id
                ).fetch_one(pool).await?.id;

                sqlx::query!(
                    r#"
                        INSERT INTO nutrients_chemical_type_table (chemical_type_id, energy_yielding_nutrient_id)
                        VALUES (?, ?)
                        RETURNING id
                    "#,
                    self.chemical_type_id,
                    energy_id
                ).fetch_one(pool).await?.id
            }
            Some(4) => {
                let energy_id = sqlx::query!(
                    r#"
                        INSERT INTO nutrients_energy_yielding_nutrients (energy_yielding_nutrient_type_id)
                        VALUES (?)
                        RETURNING id
                    "#,
                    self.energy_type_id,
                ).fetch_one(pool).await?.id;

                sqlx::query!(
                    r#"
                        INSERT INTO nutrients_chemical_type_table (chemical_type_id, energy_yielding_nutrient_id)
                        VALUES (?, ?)
                        RETURNING id
                    "#,
                    self.chemical_type_id,
                    energy_id
                ).fetch_one(pool).await?.id
            }
            None => {
                let chemical_row = sqlx::query!(
                    r#"
                        SELECT chemical_type_id, id FROM nutrients_chemical_type_table
                        WHERE chemical_type_id = ?
                    "#,
                    self.chemical_type_id,
                )
                .fetch_optional(pool)
                .await?;

                match chemical_row {
                    Some(row) => row.id,
                    None => {
                        sqlx::query!(
                            r#"
                                INSERT INTO nutrients_chemical_type_table (chemical_type_id)
                                VALUES (?)
                                    RETURNING id
                            "#,
                            self.chemical_type_id,
                        ).fetch_one(pool).await?.id
                    }
                }
            }
            _ => panic!("Unknown energy type id"),
        };

        let rows = sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrient_types (essentiality_type_id, quantity_type_id, chemical_id)
                VALUES (?, ?, ?)
                ON CONFLICT(essentiality_type_id, quantity_type_id, chemical_id) DO NOTHING
            "#,
            self.essentiality_type_id,
            self.quantity_type_id,
            chemical_id
        ).execute(pool).await?;

        Ok(())
    }

    pub async fn load_from_database_from_nutrient_type_ids(
        essentiality_type_id: Option<i64>,
        quantity_type_id: i64,
        chemical_type_id: i64,
        pool: &Pool<Sqlite>,
    ) -> Result<Self, sqlx::Error> {
        let row = sqlx::query_as!(
            NutrientTypeRecord,
            r#"
                SELECT
                    n.quantity_type_id,
                    n.essentiality_type_id,
                    ch_t.chemical_type_id,
                    e.energy_yielding_nutrient_type_id as energy_type_id,
                    c.carbohydrate_type_id,
                    p.is_bcaa,
                    lt.lipid_type_id,
                    lt.sterol_type_id,
                    lt.fat_type_id,
                    lt.transfat_type_id
                FROM nutrients_nutrient_types n
                INNER JOIN nutrients_chemical_type_table ch_t
                    ON n.chemical_id = ch_t.id
                INNER JOIN nutrients_energy_yielding_nutrients e
                    ON ch_t.energy_yielding_nutrient_id = e.id
                INNER JOIN nutrients_carbohydrate_nutrients c 
                    ON e.carbohydrate_nutrient_id = c.id
                INNER JOIN nutrients_protein_nutrients p
                    ON e.protein_nutrient_id = p.id
                INNER JOIN nutrients_lipid_nutrients l 
                    ON e.lipid_nutrient_id = l.id
                INNER JOIN nutrients_lipid_table lt
                    ON l.lipid_id = lt.id
                WHERE
                    n.quantity_type_id = ?
                    AND n.essentiality_type_id IS ?
                    AND ch_t.chemical_type_id = ?
            "#,
            quantity_type_id,
            essentiality_type_id,
            chemical_type_id
        )
        .fetch_one(pool)
        .await?;

        return Ok(row);
    }

    pub async fn delete_from_database_from_nutrient_type_id(
        &self,
        pool: &Pool<Sqlite>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
                DELETE FROM nutrients_nutrient_types
                WHERE 
                    quantity_type_id = ?
                    AND (
                        essentiality_type_id = ?
                        OR (? IS NULL AND essentiality_type_id IS NULL)
                    )
                    AND chemical_id IN (
                        SELECT id
                            FROM nutrients_chemical_type_table
                            WHERE chemical_type_id = ?
                    )
            "#,
            self.essentiality_type_id,
            self.essentiality_type_id,
            self.quantity_type_id,
            self.chemical_type_id
        )
        .execute(pool)
        .await?;

        Ok(())
    }

    // pub async fn get_chemical_id(&self, pool: &Pool<Sqlite>) -> Result<i64, sqlx::Error> {
    //     let row = sqlx::query!(
    //         r#"
    //             SELECT ch_t.id 
    //             FROM nutrients_chemical_type_table ch_t
    //             INNER JOIN nutrients_nutrient_types n
    //                 ON n.chemical_id = ch_t.id
    //             WHERE
    //                 n.quantity_type_id = ?
    //                 AND n.essentiality_type_id = ?
    //                 AND n.chemical_id = ?
    //         "#,
    //         self.quantity_type_id,
    //         self.essentiality_type_id,
    //         self.chemical_type_id
    //     )
    //     .fetch_one(pool)
    //     .await?;
    //
    //     Ok(row.id.expect("Invalid record"))
    // }
}
