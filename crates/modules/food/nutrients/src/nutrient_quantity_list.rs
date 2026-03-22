use crate::{nutrient::Nutrient, nutrient_quantity::NutrientQuantity};
use std::{cell::RefCell, collections::BTreeSet, rc::Rc};
use units::energy::quantity::EnergyQuantity;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct NutrientQuantityList {
    id: Uuid,
    nutrient_quantities: BTreeSet<Id<NutrientQuantity>>,
}

impl NutrientQuantityList {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            nutrient_quantities: BTreeSet::new(),
        }
    }

    pub fn from_vec(nutrient_amount_vec: Vec<NutrientQuantity>) -> Self {
        let nutrient_quantities: BTreeSet<NutrientQuantity> =
            nutrient_amount_vec.into_iter().collect();
        Self {
            id: Uuid::new_v4(),
            nutrient_quantities,
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_nutrient_quantities(&self) -> BTreeSet<NutrientQuantity> {
        self.nutrient_quantities.clone()
    }

    pub fn set_nutrient_amounts(&mut self, nutrient_quantities: BTreeSet<NutrientQuantity>) {
        self.nutrient_quantities = nutrient_quantities;
    }

    pub fn get_nutrient_names(&self) -> Vec<String> {
        self.nutrient_quantities
            .iter()
            .map(|nutrient_amount| nutrient_amount.get_nutrient().borrow().get_name())
            .collect()
    }

    pub fn push(&mut self, nutrient_amount: NutrientQuantity) -> bool {
        self.nutrient_quantities.insert(nutrient_amount)
    }

    pub fn extend(&mut self, nutrient_quantities: Vec<NutrientQuantity>) {
        self.nutrient_quantities.extend(nutrient_quantities);
    }

    pub fn remove(&mut self, nutrient_amount: &NutrientQuantity) {
        self.nutrient_quantities.remove(nutrient_amount);
    }

    pub fn sum_amounts_from_ancestors_rc_refcell(
        &self,
        nutrient: Rc<RefCell<Nutrient>>,
    ) -> NutrientQuantity {
        let nutrients: Vec<Rc<RefCell<Nutrient>>> = nutrient.borrow().get_ancestors();
        let nutrient_quantities: Vec<NutrientQuantity> = self
            .nutrient_quantities
            .iter()
            .filter(|nutrient_amount| {
                nutrients
                    .iter()
                    .any(|n| Rc::ptr_eq(n, &nutrient_amount.get_nutrient()))
            })
            // .filter(|nutrient_amount| nutrients.contains(&nutrient_amount.get_nutrient()))
            .cloned()
            .collect();
        println!(
            "nutrient_quantities: {:#?}",
            nutrient_quantities
                .iter()
                .map(|n| n.get_nutrient().borrow().get_name())
        );

        let sum = nutrient_quantities.into_iter().sum::<NutrientQuantity>();
        return sum;
    }

    pub fn sum_amounts_from_ancestors(&self, nutrient: Nutrient) -> NutrientQuantity {
        let nutrients: Vec<Rc<RefCell<Nutrient>>> = nutrient.get_ancestors();
        let nutrient_quantities: Vec<NutrientQuantity> = self
            .nutrient_quantities
            .iter()
            .filter(|nutrient_amount| nutrients.contains(&nutrient_amount.get_nutrient()))
            .cloned()
            .collect();

        return nutrient_quantities.into_iter().sum::<NutrientQuantity>();
    }

    pub fn sum_amounts_from_descendants_rc_refcell(
        &self,
        nutrient: Rc<RefCell<Nutrient>>,
    ) -> NutrientQuantity {
        let nutrients: Vec<Rc<RefCell<Nutrient>>> = nutrient.borrow().get_descendants();
        let nutrient_quantities: Vec<NutrientQuantity> = self
            .nutrient_quantities
            .iter()
            .filter(|nutrient_amount| {
                nutrients
                    .iter()
                    .any(|n| Rc::ptr_eq(n, &nutrient_amount.get_nutrient()))
            })
            .cloned()
            .collect();

        let sum = nutrient_quantities.into_iter().sum::<NutrientQuantity>();
        return sum;
    }

    pub fn sum_amounts_from_descendants(&self, nutrient: Nutrient) -> NutrientQuantity {
        let nutrients: Vec<Rc<RefCell<Nutrient>>> = nutrient.get_descendants();
        let nutrient_quantities: Vec<NutrientQuantity> = self
            .nutrient_quantities
            .iter()
            .filter(|nutrient_amount| nutrients.contains(&nutrient_amount.get_nutrient()))
            .cloned()
            .collect();

        return nutrient_quantities.into_iter().sum::<NutrientQuantity>();
    }

    pub fn get_calories(&self) -> Result<EnergyQuantity, &'static str> {
        let mut calories_sum = EnergyQuantity::new(0f64, units::energy::unit::EnergyUnit::Kilocalorie); 
        for nutrient_quantity in &self.nutrient_quantities {
            let calories = nutrient_quantity.get_calories()?;
            calories_sum = calories_sum + calories;
        }

        return Ok(calories_sum)
    }
}
