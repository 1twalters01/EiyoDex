use crate::schema::{carbohydrate::{Carbohydrate, CarbohydrateNutrient}, energy::EnergyYieldingNutrients, lipid::{Fat, Lipid, LipidNutrient, Sterols, TransFat}, nutrient_classes::{ChemicalType, EssentialityType, QuantityType}, nutrient_type::NutrientType, protein::ProteinNutrient};

pub struct NutrientTypeRow {
    quantity_id: i64,
    essentiality_id: Option<i64>,
    chemical_id: i64,

    energy_id: Option<i64>,

    carbohydrate_id: Option<i64>,
    is_added_sugar: Option<bool>,
    glycemic_index: Option<u8>,

    is_bcaa: bool,

    sterol_id: Option<i64>,
    fat_id: Option<i64>,
    transfat_id: Option<i64>,
}

impl NutrientTypeRow {
    pub fn to_nutrient_type(&self) -> NutrientType {
        let chemical_type = match self.chemical_id {
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
                        let glycemic_index = self.glycemic_index;
                        ChemicalType::EnergyYieldingNutrients(EnergyYieldingNutrients::Carbohydrate(CarbohydrateNutrient { carbohydrate_type, is_added_sugar, glycemic_index }))
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

        NutrientType::new(chemical_type, quantity_type, essentiality_type)
    }

    pub fn load_from_database() -> Self {}
    pub fn save_to_database(&self) {}
}
