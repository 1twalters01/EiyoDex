#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct CarbohydrateNutrient {
    pub carb_type: Carbohydrate,
    pub is_added: bool,
    pub glycemic_index: Option<u8>,
}

impl CarbohydrateNutrient {
    pub fn use_in_net_carbs(&self) -> bool {
        self.carb_type.use_in_net_carbs()
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub enum Carbohydrate {
    Fiber,
    Starch,
    Sugar,
    SugarAlcohol,
}

impl Carbohydrate {
    pub fn use_in_net_carbs(&self) -> bool {
        match self {
            Carbohydrate::Fiber | Carbohydrate::SugarAlcohol => false,
            _ => true,
        }
    }
}
