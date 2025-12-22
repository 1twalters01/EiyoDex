use std::{
    cell::RefCell,
    cmp::Ordering,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::{nutrient::Nutrient, units::NutrientUnit};

#[derive(Debug, Clone, PartialEq)]
pub struct NutrientAmount {
    value: f64,
    nutrient: Rc<RefCell<Nutrient>>,
    output_unit: NutrientUnit,
}

impl NutrientAmount {
    pub fn new(
        value: f64,
        nutrient: Nutrient,
        output_unit: NutrientUnit
    ) -> Result<Self, &'static str> {
        match nutrient.convert(output_unit, nutrient.get_main_unit()) {
            Ok(_) => Ok(Self {
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
        let nutrient_borrowed = nutrient.borrow();
        match nutrient_borrowed.convert(output_unit, nutrient_borrowed.get_main_unit()) {
            Ok(_) => Ok(Self {
                value,
                nutrient: nutrient.clone(),
                output_unit,
            }),
            Err(err) => Err(err),
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn get_nutrient(&self) -> Rc<RefCell<Nutrient>> {
        self.nutrient.clone()
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

    pub fn set_output_unit(&mut self, output_unit: NutrientUnit) {
        let nutrient_borrowed = self.nutrient.borrow();
        let conversion_factor = nutrient_borrowed.convert(self.output_unit, output_unit).unwrap();
        self.value = self.value * conversion_factor;
        self.output_unit = output_unit;
    }

    pub fn round(&mut self, dp: u8) -> Self {
        let factor = 10f64.powi(dp as i32);
        self.value = (self.value * factor).round() / factor;
        return self.clone();
    }

    pub fn convert(&self, unit: NutrientUnit) -> Result<f64, &'static str> {
        let nutrient_borrowed = self.nutrient.borrow();
        nutrient_borrowed.convert(nutrient_borrowed.get_main_unit(), unit)
            .and_then(|c| Ok(c * self.get_value()))
    }
}

impl PartialOrd for NutrientAmount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let other_nutrient = other.nutrient.borrow();
        let from_unit = other.output_unit;
        let to_unit = self.output_unit;
        let conversion_factor = other_nutrient.convert(from_unit, to_unit).unwrap();
        self.value
            .partial_cmp(&(other.value * conversion_factor))
    }
}

impl Eq for NutrientAmount {}

impl Ord for NutrientAmount {
    fn cmp(&self, other: &Self) -> Ordering {
        println!("hi");
        let id_cmp = self
            .get_nutrient()
            .borrow()
            .get_name()
            .cmp(&other.get_nutrient().borrow().get_name());
        if id_cmp != Ordering::Equal {
            return id_cmp;
        }

        let other_nutrient = other.nutrient.borrow();
        let from_unit = other.output_unit;
        let to_unit = self.output_unit;
        let conversion_factor = other_nutrient.convert(from_unit, to_unit).unwrap();
        self.value
            .partial_cmp(&(other.value * conversion_factor))
            .unwrap_or(Ordering::Equal)
    }
}

impl Add for NutrientAmount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        if self.nutrient != rhs.nutrient {
            panic!(
                "Tried to add different nutrients: {:#?} + {:#?}",
                self.nutrient, rhs.nutrient
            );
        }
        let rhs_nutrient = rhs.nutrient.borrow();
        let conversion_factor = rhs_nutrient.convert(rhs.output_unit, self.output_unit).unwrap();
        Self {
            value: self.value + rhs.value * conversion_factor,
            nutrient: self.nutrient,
            output_unit: self.output_unit
        }
    }
}

impl Sub for NutrientAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.nutrient != rhs.nutrient {
            panic!(
                "Tried to add different nutrients: {:#?} + {:#?}",
                self.nutrient, rhs.nutrient
            );
        }
        let rhs_nutrient = rhs.nutrient.borrow();
        let conversion_factor = rhs_nutrient.convert(rhs.output_unit, self.output_unit).unwrap();
        Self {
            value: self.value - rhs.value * conversion_factor,
            nutrient: self.nutrient,
            output_unit: self.output_unit
        }
    }
}

impl Mul<f64> for NutrientAmount {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        let nutrient_borrowed = self.nutrient.borrow();
        let main_unit = nutrient_borrowed.get_main_unit();
        Self::from_rc_refcell(self.value * rhs, self.get_nutrient(), main_unit).unwrap()
    }
}

impl Div<f64> for NutrientAmount {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        let nutrient_borrowed = self.nutrient.borrow();
        let main_unit = nutrient_borrowed.get_main_unit();
        Self::from_rc_refcell(self.get_value() / rhs, self.get_nutrient(), main_unit).unwrap()
    }
}
