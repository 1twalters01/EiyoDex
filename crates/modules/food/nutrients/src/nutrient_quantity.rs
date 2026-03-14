use std::{
    cell::RefCell,
    cmp::Ordering,
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use units::{energy::quantity::EnergyQuantity, mass::unit::MassUnit};
use uuid::Uuid;

use crate::{nutrient::Nutrient, nutrient_units::NutrientUnit};

#[derive(Debug, Clone)]
pub struct NutrientQuantity {
    id: Uuid,
    value: f64,
    nutrient: Rc<RefCell<Nutrient>>,
    output_unit: NutrientUnit,
}

impl NutrientQuantity {
    pub fn new(
        value: f64,
        nutrient: Nutrient,
        output_unit: NutrientUnit,
    ) -> Result<Self, &'static str> {
        let main_unit = match nutrient.get_main_unit() {
            Some(main_unit) => main_unit,
            None => return Err("Main unit of nutrient cannot be none"),
        };

        match nutrient.get_conversion_factor(output_unit, main_unit) {
            Ok(_) => Ok(Self {
                id: Uuid::new_v4(),
                value,
                nutrient: Rc::new(RefCell::new(nutrient)),
                output_unit,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn from_rc_refcell(
        value: f64,
        nutrient: Rc<RefCell<Nutrient>>,
        output_unit: NutrientUnit,
    ) -> Result<Self, &'static str> {
        let nutrient_borrowed = nutrient.borrow().clone();
        let main_unit = match nutrient_borrowed.get_main_unit() {
            Some(main_unit) => main_unit,
            None => return Err("Main unit of nutrient cannot be none"),
        };

        match nutrient_borrowed
            .get_conversion_factor(output_unit, main_unit)
        {
            Ok(_) => Ok(Self {
                id: Uuid::new_v4(),
                value,
                nutrient: nutrient,
                output_unit,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn get_id(&self) -> Uuid {
        self.id
    }

    pub fn set_id(&mut self, id: Uuid) {
        self.id = id;
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn get_value_in(&self, nutrient_unit: NutrientUnit) -> Result<f64, &'static str> {
        let nutrient_borrowed = self.nutrient.borrow();
        nutrient_borrowed
            .get_conversion_factor(self.output_unit, nutrient_unit)
            .and_then(|conversion_factor| Ok(self.value * conversion_factor))
    }

    pub fn get_nutrient(&self) -> Rc<RefCell<Nutrient>> {
        self.nutrient.clone()
    }

    pub fn get_nutrient_id(&self) -> Uuid {
        self.nutrient.borrow().get_id()
    }

    pub fn set_nutrient_rc_refcell(&mut self, nutrient: Rc<RefCell<Nutrient>>) {
        self.nutrient = nutrient;
    }

    pub fn set_nutrient(&mut self, nutrient: Nutrient) {
        self.nutrient = Rc::new(RefCell::new(nutrient));
    }

    pub fn get_output_unit(&self) -> NutrientUnit {
        self.output_unit
    }

    pub fn set_output_unit(&mut self, output_unit: NutrientUnit) -> Result<(), &'static str> {
        let nutrient_borrowed = self.nutrient.borrow();
        match nutrient_borrowed.get_conversion_factor(self.output_unit, output_unit) {
            Ok(conversion_factor) => {
                self.value = self.value * conversion_factor;
                self.output_unit = output_unit;
                Ok(())
            }
            Err(err) => Err(err),
        }
    }

    pub fn get_calories(&self) -> Result<EnergyQuantity, &'static str> {
        let nutrient_borrowed = self.nutrient.borrow();
        let calories_per_gram = nutrient_borrowed.get_calories_per_gram();
        if calories_per_gram.is_zero() {
            return Ok(calories_per_gram)
        }

        match nutrient_borrowed.get_conversion_factor(self.output_unit, NutrientUnit::Mass(MassUnit::Gram)) {
            Ok(conversion_factor) => {
                Ok(calories_per_gram * self.value * conversion_factor)
            },
            Err(err) => Err(err),
        }
    }

    pub fn round(&mut self, dp: u8) -> Self {
        let factor = 10f64.powi(dp as i32);
        self.value = (self.value * factor).round() / factor;
        return self.clone();
    }
}

impl PartialEq for NutrientQuantity {
    fn eq(&self, other: &Self) -> bool {
        self.get_nutrient_id() == other.get_nutrient_id()
    }
}

impl Eq for NutrientQuantity {}

impl Ord for NutrientQuantity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.get_nutrient_id().cmp(&other.get_nutrient_id())
    }
}

impl PartialOrd for NutrientQuantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for NutrientQuantity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        let rhs_nutrient_option = rhs.nutrient.clone();
        let rhs_nutrient = rhs_nutrient_option.borrow();
        let conversion_factor = rhs_nutrient
            .get_conversion_factor(rhs.output_unit, self.output_unit)
            .unwrap();
        Self {
            id: Uuid::new_v4(),
            value: self.value + rhs.value * conversion_factor,
            nutrient: self.nutrient,
            output_unit: self.output_unit,
        }
    }
}

impl Sub for NutrientQuantity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let rhs_nutrient_option = rhs.nutrient.clone();
        let rhs_nutrient = rhs_nutrient_option.borrow();
        let conversion_factor = rhs_nutrient
            .get_conversion_factor(rhs.output_unit, self.output_unit)
            .unwrap();
        Self {
            id: Uuid::new_v4(),
            value: self.value - rhs.value * conversion_factor,
            nutrient: self.nutrient,
            output_unit: self.output_unit,
        }
    }
}

impl Mul<f64> for NutrientQuantity {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        let nutrient_borrowed = self.nutrient.borrow();
        let main_unit = match nutrient_borrowed.get_main_unit() {
            Some(unit) => unit,
            None => panic!("No unit found"),
        };

        Self::from_rc_refcell(self.value * rhs, self.get_nutrient(), main_unit).unwrap()
    }
}

impl Div<f64> for NutrientQuantity {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        let nutrient_borrowed = self.nutrient.borrow();
        let main_unit = match nutrient_borrowed.get_main_unit() {
            Some(unit) => unit,
            None => panic!("No unit found"),
        };

        Self::from_rc_refcell(self.value / rhs, self.get_nutrient(), main_unit).unwrap()
    }
}

impl Sum<NutrientQuantity> for NutrientQuantity {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        if let Some(first) = iter.next() {
            iter.fold(first, |acc, n| acc + n)
        } else {
            panic!("Zero elements in list")
            // NutrientQuantity::new(0f64, Nutrient::default(), NutrientUnit::None).unwrap()
        }
    }
}
