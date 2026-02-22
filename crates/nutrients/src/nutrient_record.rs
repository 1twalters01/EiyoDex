pub struct NutrientRecord {
    id: Uuid,
    name: String,
    description: String,
    nutrient_type: NutrientType,
    unit_conversions: BTreeMap<NutrientUnit, f64>, // 1 unit = factor * main_unit
    main_unit: NutrientUnit,
}

impl NutrientRecord {
    pub fn from_nutrient(nutrient: Nutrient) -> Self {}
    pub fn from_nutrient_vec(nutrient_vec: Vec<Nutrient>) -> Vec<Self> {}
    pub fn load_from_sqlite() -> Vec<Self> {}
}

pub struct NutrientLinkRecord {
    parent_id: Uuid,
    child_id: Uuid,
}

impl NutrientLinkRecord {
    pub fn from_nutrient(nutrient: Nutrient) -> Self {}
    pub fn from_nutrient_vec(nutrient_vec: Vec<Nutrient>) -> Vec<Self> {}
    pub fn load_from_sqlite() -> Vec<Self> {}
}
