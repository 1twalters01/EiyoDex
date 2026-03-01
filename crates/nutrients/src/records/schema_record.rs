use utils::database::DatabaseService;
use uuid::Uuid;

use crate::schema::{carbohydrate::{Carbohydrate, CarbohydrateNutrient}, energy::EnergyYieldingNutrients, lipid::{Fat, Lipid, LipidNutrient, Sterols, TransFat}, nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType, protein::ProteinNutrient};

// Revisit the sql tables and this data
pub struct NutrientTypeRow {
    nutrient_id: Vec<u8>,
    quantity_id: i64,
    essentiality_id: Option<i64>,
    chemical_kind_id: i64,

    energy_id: Option<i64>,

    carbohydrate_id: Option<i64>,
    is_added_sugar: Option<bool>,
    glycemic_index: Option<i64>,

    is_bcaa: Option<bool>,

    sterol_id: Option<i64>,
    fat_id: Option<i64>,
    transfat_id: Option<i64>,
}

impl NutrientTypeRow {
    pub fn from_nutrient_type(nutrient_type: NutrientType) -> Self {
        let nutrient_id = nutrient_type.id;
        
        let chemical_type_id = match nutrient_type.chemical_type {
            ChemicalType::EnergyYieldingNutrients => 1,
            ChemicalType::Water => 2,
            ChemicalType::Vitamin => 3,
            ChemicalType::Mineral => 4,
            ChemicalType::Phytonutrient => 5,
            ChemicalType::Antinutrient => 6,
            ChemicalType::Other => 7,
        };

        let quantity_type_id = match self.quantity_id {
            QuantityType::Macronutrient => 1,
            QuantityType::Micronutrient => 2,
            QuantityType::NonNutrient => 3,
        };

        let essentiality_type_id = match self.essentiality_id {
            Some(EssentialityType::Essential) => Some(1),
            Some(EssentialityType::ConditionallyEssential) => Some(2),
            Some(EssentialityType::NonEssential) => Some(3),
            None => None,
        };

        let energy_id = None;
        let carbohydrate_id = None;
        let is_added_sugar = None;
        let glycemic_index = None;
        let is_bcaa = None;
        let sterol_id = None;
        let fat_id = None;
        let transfat_id = None;
        if chemical_type_id == 1 {
            if let ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(carbohydrate) = nutrient_type.chemical_id {
                carbohydrate_id = match carbohydrate {
                    Carbohydrate::Fiber => Some(1),
                    Carbohydrate::Starch => Some(2),
                    Carbohydrate::Sugar => Some(3),
                    Carbohydrate::SugarAlcohol => Some(4),
                };
                is_added_sugar = Some(carbohydrate.is_sugar);
                glycemic_index = Some(carbohydrate.glycemic_index);
            }
            if let ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(protein) = nutrient_type.chemical_id {
                is_bcaa = Some(protein.is_bcaa);
            }
            if let ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Lipid(lipid) = nutrient_type.chemical_id {
                match lipid {
                    Lipid::Sterol(sterol) => match sterol {
                        Sterol::Cholesterol => Some(1),
                        Sterol::Phytosterol => Some(2),
                    }
                    Lipid::Fat(fat) => match fat {
                        Fat::Monounsaturated => fat_id = Some(1),
                        Fat::Polyunsaturated => fat_id = Some(2),
                        Fat::Saturated => fat_id = Some(3),
                    }
                    Lipid::TransFat => match transfat {
                        Transfat::Natural => transfat_id = Some(1),
                        Transfat::Artificial => transfat_id = Some(2),
                    }
                }
            }
        }
        
        Self {
            nutrient_id,
            quantity_id,
            essentiality_id,
            chemical_kind_id,
            energy_id,
            carbohydrate_id,
            is_added_sugar,
            glycemic_index,
            is_bcaa,
            sterol_id,
            fat_id,
            transfat_id,
        }
    }
    pub fn to_nutrient_type(&self) -> NutrientType {
        let chemical_type = match self.chemical_kind_id {
            1 => {
                match self.energy_id.unwrap() {
                    1 => {
                        let carbohydrate_type = match self.carbohydrate_id.unwrap() {
                            1 => Carbohydrate::Fiber,
                            2 => Carbohydrate::Starch,
                            3 => Carbohydrate::Sugar,
                            4 => Carbohydrate::SugarAlcohol,
                            _ => panic!("Unknown carbohydrate_id"),
                        };
                        let is_added_sugar = self.is_added_sugar.expect("is_added_sugar was not found");
                        let glycemic_index = self.glycemic_index.map(|gi| gi as u8);
                        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type, is_added_sugar, glyce_glycemic_index index }))
                    }
                    2 => ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Protein(ProteinNutrient { is_bcaa: self.is_bcaa })),
                    3 => {
                        let sterol = match self.sterol_id {
                            Some(1) => Some(Lipid::Sterols(Sterols::Cholesterol)),
                            Some(2) => Some(Lipid::Sterols(Sterols::Phytosterol)),
                            None => None,
                            _ => panic!("Unknown sterol"),
                        };
                        let fat = match self.fat_id {
                            Some(1) => Some(Lipid::Fats(Fat::Monounsaturated)),
                            Some(2) => Some(Lipid::Fats(Fat::Polyunsaturated)),
                            Some(3) => Some(Lipid::Fats(Fat::Saturated)),
                            None => None,
                            _ => panic!("Unknown fat"),
                        };
                        let transfat = match self.transfat_id {
                            Some(1) => Some(Lipid::TransFats(TransFat::Natural)),
                            Some(2) => Some(Lipid::TransFats(TransFat::Artificial)),
                            None => None,
                            _ => panic!("Unknown transfat"),
                        };
                        let values: Vec<Lipid> = [sterol, fat, transfat].iter().filter_map(|x| *x).collect();

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
                }
            },
            2 => ChemicalType::Water,
            3 => ChemicalType::Vitamin,
            4 => ChemicalType::Mineral,
            5 => ChemicalType::Phytonutrient,
            6 => ChemicalType::Antinutrient,
            7 => ChemicalType::Other,
            _ => panic!("Unknown chemical_id"),
        };

        let quantity_type = match self.quantity_id {
            1 => QuantityType::Macronutrient,
            2 => QuantityType::Micronutrient,
            3 => QuantityType::NonNutrient,
            _ => panic!("Unknown quantity_id"),
        };

        let essentiality_type = match self.essentiality_id {
            Some(1) => Some(EssentialityType::Essential),
            Some(2) => Some(EssentialityType::ConditionallyEssential),
            Some(3) => Some(EssentialityType::NonEssential),
            None => None,
            Some(_) => panic!("Unknown essentiality_id"),
        };

        let id = Uuid::from_slice(&self.nutrient_id).expect("Invalid nutrient_id");

        NutrientType::new_with_id(id, chemical_type, quantity_type, essentiality_type)
    }

    pub async fn save_to_database(&self, chemical_id_option: Option<i64>) -> Result<(), sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();

        // Save Chemical type
        let chemical_id: i64 = match chemical_id_option {
            Some(id) => {
                let row = sqlx::query!(
                r#"
                    INSERT INTO nutrients_chemical_types (id, kind_id)
                    VALUES (?, ?)
                    ON CONFLICT(id) DO UPDATE SET
                        id = excluded.id,
                        kind_id = excluded.kind_id
                    RETURNING id
                "#,
                id,
                self.chemical_kind_id
                )
                .fetch_one(&database_service.pool)
                .await?;

                row.id.unwrap()
            },
            None => {
                let row = sqlx::query!(
                r#"
                    INSERT INTO nutrients_chemical_types (kind_id)
                    VALUES (?)
                    RETURNING id
                "#,
                self.chemical_kind_id
                )
                .fetch_one(&database_service.pool)
                .await?;

                row.id.unwrap()
            },
        };

        sqlx::query!(
            r#"
                INSERT INTO nutrients_nutrients (id, chemical_id, quantity_id, essentiality_id)
                VALUES (?, ?, ?, ?)
                ON CONFLICT(id) DO UPDATE SET
                    chemical_id = excluded.chemical_id,
                    quantity_id = excluded.quantity_id,
                    essentiality_id = excluded.essentiality_id
            "#,
            self.nutrient_id,
            self.chemical_kind_id,
            self.quantity_id,
            self.essentiality_id
        )
        .execute(&database_service.pool)
        .await?;

        if chemical_kind_id == 1 {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_energy_yielding_nutrients (chemical_id, kind_id)
                VALUES (?, ?)
                ON CONFLICT(chemical_id) DO UPDATE SET
                    chemical_id = excluded.chemical_id,
                    kind_id = excluded.kind_id
            "#,
            chemical_id,
            self.chemical_kind_id,
        )
        .execute(&database_service.pool)
        .await?;

            if energy_id == Some(1) {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_carbohydrate_nutrients (energy_id, carbohydrate_id, is_added_sugar, glycemic_index)
                VALUES (?, ?, ?, ?)
                ON CONFLICT(energy_id) DO UPDATE SET
                    energy_id = excluded.energy_id,
                    carbohydrate_id = excluded.carbohydrate_id,
                    is_added_sugar = excluded.is_added_sugar,
                    glycemic_index = excluded.glycemic_index
            "#,
            self.energy_id,
            self.carbohydrate_id,
            self.is_added_sugar,
            self.glycemic_index
        )
        .execute(&database_service.pool)
        .await?;
            }

            if energy_id == Some(2) {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_protein_nutrients (energy_id, is_bcaa)
                VALUES (?, ?)
                ON CONFLICT(energy_id) DO UPDATE SET
                    energy_id = excluded.energy_id,
                    is_bcaa = is_bcaa
            "#,
            self.energy_id,
            self.is_bcaa
        )
        .execute(&database_service.pool)
        .await?;
            }

            if energy_id == Some(3) {
        sqlx::query!(
            r#"
                INSERT INTO nutrients_lipid_nutrients (energy_id, sterol_id, fat_id, transfat_id)
                VALUES (?, ?, ?, ?)
                ON CONFLICT(energy_id) DO UPDATE SET
                    energy_id = excluded.energy_id,
                    sterol_id = excluded.sterol_id,
                    fat_id = excluded.fat_id,
                    transfat_id = excluded.transfat_id
            "#,
            self.energy_id,
            self.sterol_id,
            self.fat_id,
            self.transfat_id
        )
        .execute(&database_service.pool)
        .await?;
        }
        }
        Ok(())
    }

    pub async fn load_from_database_from_nutrient_id(nutrient_id: Uuid) -> Result<Self, sqlx::Error> {
        let database_service = DatabaseService::new().await.unwrap();
        let n_id: &[u8] = &nutrient_id.as_bytes()[..];
        sqlx::query_as!(
                NutrientTypeRow,
                r#"
                SELECT
                    n.id as nutrient_id,
                    n.quantity_id,
                    n.essentiality_id,
                    ch_t.kind_id as chemical_kind_id,
                    e.kind_id as energy_id,
                    c.carbohydrate_id,
                    c.is_added_sugar,
                    c.glycemic_index,
                    p.is_bcaa,
                    l.sterol_id,
                    l.fat_id,
                    l.transfat_id
                FROM nutrients_nutrients n 
                INNER JOIN nutrients_chemical_types ch_t
                    ON n.chemical_id = ch_t.id
                INNER JOIN nutrients_energy_yielding_nutrients e
                    ON ch_t.id = e.chemical_id
                INNER JOIN nutrients_carbohydrate_nutrients c 
                    ON e.chemical_id = c.energy_id
                INNER JOIN nutrients_protein_nutrients p
                    ON e.chemical_id = p.energy_id
                INNER JOIN nutrients_lipid_nutrients l
                    ON e.chemical_id = l.energy_id
                WHERE n.id = ?"#,
            n_id
        )
            .fetch_one(&database_service.pool)
            .await
    }
}

