use crate::{nutrient::Nutrient, nutrient_amount::NutrientAmount, units::NutrientUnit};
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NutrientAmountList {
    id: Uuid,
    nutrient_amounts: BTreeSet<NutrientAmount>,
}

#[derive(Debug)]
pub struct SumResult {
    value: f64,
    unit: NutrientUnit,
}

impl SumResult {
    pub fn get_value(&self) -> f64 {
        self.value
    }
    pub fn get_unit(&self) -> NutrientUnit {
        self.unit
    }
}

impl NutrientAmountList {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            nutrient_amounts: BTreeSet::new(),
        }
    }

    pub fn from_vec(nutrient_amount_vec: Vec<NutrientAmount>) -> Self {
        let nutrient_amounts: BTreeSet<NutrientAmount> = nutrient_amount_vec.into_iter().collect();
        Self {
            id: Uuid::new_v4(),
            nutrient_amounts,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_nutrient_amounts(&self) -> BTreeSet<NutrientAmount> {
        self.nutrient_amounts.clone()
    }

    pub fn set_nutrient_amounts(&mut self, nutrient_amounts: BTreeSet<NutrientAmount>) {
        self.nutrient_amounts = nutrient_amounts;
    }

    pub fn get_names(&self) -> Vec<Option<String>> {
        self.nutrient_amounts.iter().map(|nutrient_amount| nutrient_amount.get_nutrient().map(|nutrient| nutrient.borrow().get_name())).collect()
    }

    pub fn push(&mut self, nutrient_amount: NutrientAmount) {
        self.nutrient_amounts.insert(nutrient_amount);
    }

    pub fn remove(&mut self, nutrient_amount: &NutrientAmount) {
        self.nutrient_amounts.remove(nutrient_amount);
    }

    pub fn sum_amounts_from_ancestors_rc_refcell(
        &self,
        nutrient: Rc<RefCell<Nutrient>>,
    ) -> SumResult {
        let nutrients_unformatted: Vec<Rc<RefCell<Nutrient>>> = nutrient.borrow().get_ancestors();
        let nutrients: Vec<Option<Rc<RefCell<Nutrient>>>> =
            nutrients_unformatted.into_iter().map(|nutrient| Some(nutrient)).collect();
        let nutrient_amounts: Vec<NutrientAmount> = self
            .nutrient_amounts
            .iter()
            .filter(|nutrient_amount| nutrients.contains(&nutrient_amount.get_nutrient()))
            .cloned()
            .collect();

        let sum = nutrient_amounts.into_iter().sum::<NutrientAmount>();
        let sum_result = SumResult {
            value: sum.get_value(),
            unit: sum.get_output_unit(),
        };
        return sum_result;
    }

    pub fn sum_amounts_from_ancestors(&self, nutrient: Nutrient) -> NutrientAmount {
        let nutrients_unformatted: Vec<Rc<RefCell<Nutrient>>> = nutrient.get_ancestors();
        let nutrients: Vec<Option<Rc<RefCell<Nutrient>>>> =
            nutrients_unformatted.into_iter().map(|nutrient| Some(nutrient)).collect();
        let nutrient_amounts: Vec<NutrientAmount> = self
            .nutrient_amounts
            .iter()
            .filter(|nutrient_amount| nutrients.contains(&nutrient_amount.get_nutrient()))
            .cloned()
            .collect();

        return nutrient_amounts.into_iter().sum::<NutrientAmount>();
    }

    pub fn sum_amounts_from_descendants_rc_refcell(
        &self,
        nutrient: Rc<RefCell<Nutrient>>,
    ) -> SumResult {
        let nutrients_unformatted: Vec<Rc<RefCell<Nutrient>>> = nutrient.borrow().get_descendants();
        let nutrients: Vec<Option<Rc<RefCell<Nutrient>>>> =
            nutrients_unformatted.into_iter().map(|nutrient| Some(nutrient)).collect();
        let nutrient_amounts: Vec<NutrientAmount> = self
            .nutrient_amounts
            .iter()
            .filter(|nutrient_amount| nutrients.contains(&nutrient_amount.get_nutrient()))
            .cloned()
            .collect();

        let sum = nutrient_amounts.into_iter().sum::<NutrientAmount>();
        let sum_result = SumResult {
            value: sum.get_value(),
            unit: sum.get_output_unit(),
        };
        return sum_result;
    }

    pub fn sum_amounts_from_descendants(&self, nutrient: Nutrient) -> NutrientAmount {
        let nutrients_unformatted: Vec<Rc<RefCell<Nutrient>>> = nutrient.get_descendants();
        let nutrients: Vec<Option<Rc<RefCell<Nutrient>>>> =
            nutrients_unformatted.into_iter().map(|nutrient| Some(nutrient)).collect();
        let nutrient_amounts: Vec<NutrientAmount> = self
            .nutrient_amounts
            .iter()
            .filter(|nutrient_amount| nutrients.contains(&nutrient_amount.get_nutrient()))
            .cloned()
            .collect();

        return nutrient_amounts.into_iter().sum::<NutrientAmount>();
    }
}
