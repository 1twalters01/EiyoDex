use std::{
    cell::RefCell,
    cmp::Ordering,
    iter::Sum,
    ops::{Add, Div, Mul, Sub},
    rc::Rc,
};

use crate::{nutrient::Nutrient, units::NutrientUnit};

#[derive(Debug, Clone, PartialEq)]
pub struct NutrientAmount {
    value: f64,
    nutrient: Option<Rc<RefCell<Nutrient>>>,
    output_unit: NutrientUnit,
}

impl NutrientAmount {
    pub fn new(
        value: f64,
        nutrient: Option<Nutrient>,
        output_unit: NutrientUnit,
    ) -> Result<Self, &'static str> {
        match nutrient {
            Some(nutrient) => {
                match nutrient.convert(output_unit, nutrient.get_main_unit()) {
                    Ok(_) => Ok(Self {
                        value,
                        nutrient: Some(Rc::new(RefCell::new(nutrient))),
                        output_unit,
                    }),
                    Err(err) => Err(err),
                }
            },
            None => {
                if value != 0f64 {
                    return Err("Value of empty nutrient must be 0")
                }
                if output_unit != NutrientUnit::None {
                    return Err("Nutrient unit of an empty nutrient must be 'None'")
                }

                Ok(Self {
                    value: 0f64,
                    nutrient: None,
                    output_unit: NutrientUnit::None,
                })
            },
        }
    }

    pub fn from_rc_refcell(
        value: f64,
        nutrient: Option<Rc<RefCell<Nutrient>>>,
        output_unit: NutrientUnit,
    ) -> Result<Self, &'static str> {
        match nutrient {
            Some(nutrient) => {
                let nutrient_borrowed = nutrient.borrow().clone();
                match nutrient_borrowed.convert(output_unit, nutrient_borrowed.get_main_unit()) {
                    Ok(_) => Ok(Self {
                        value,
                        nutrient: Some(nutrient),
                        output_unit,
                    }),
                    Err(err) => Err(err),
                }
            },
            None => {
                if value != 0f64 {
                    return Err("Value of empty nutrient must be 0")
                }
                if output_unit != NutrientUnit::None {
                    return Err("Nutrient unit of an empty nutrient must be 'None'")
                }

                Ok(Self {
                    value: 0f64,
                    nutrient: None,
                    output_unit: NutrientUnit::None,
                })
            },
        }
    }

    pub fn get_value(&self) -> f64 {
        self.value
    }

    pub fn set_value(&mut self, value: f64) {
        self.value = value;
    }

    pub fn get_nutrient(&self) -> Option<Rc<RefCell<Nutrient>>> {
        self.nutrient.clone()
    }

    pub fn set_nutrient_rc_refcell(&mut self, nutrient: Option<Rc<RefCell<Nutrient>>>) {
        self.nutrient = nutrient;
    }

    pub fn set_nutrient(&mut self, nutrient: Option<Nutrient>) {
        self.nutrient = nutrient.map(|n| Rc::new(RefCell::new(n)));
    }

    pub fn get_output_unit(&self) -> NutrientUnit {
        self.output_unit
    }

    pub fn set_output_unit(&mut self, output_unit: NutrientUnit) {
        match self.nutrient.clone() {
            Some(nutrient) => {
                let nutrient_borrowed = nutrient.borrow();
                let conversion_factor = nutrient_borrowed
                    .convert(self.output_unit, output_unit)
                    .unwrap();
                self.value = self.value * conversion_factor;
            },
            None => {},
        }

        self.output_unit = output_unit;
    }

    pub fn round(&mut self, dp: u8) -> Self {
        let factor = 10f64.powi(dp as i32);
        self.value = (self.value * factor).round() / factor;
        return self.clone();
    }

    pub fn convert(&self, unit: NutrientUnit) -> Result<f64, &'static str> {
        if let Some(nutrient) = self.nutrient.clone() {
            let nutrient_borrowed = nutrient.borrow();
            return nutrient_borrowed
                    .convert(nutrient_borrowed.get_main_unit(), unit)
                    .and_then(|c| Ok(c * self.get_value()))
            
        }
        return Err("");
    }
}

impl PartialOrd for NutrientAmount {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.nutrient.is_none() && other.nutrient.is_none() {
            return Some(Ordering::Equal);
        }

        else if self.nutrient.is_some() && other.nutrient.is_none() {
            return Some(Ordering::Greater)
        }

        if self.nutrient.is_none() && other.nutrient.is_none() {
            return Some(Ordering::Less)
        }

        let other_nutrient_option = other.nutrient.clone().unwrap();
        let other_nutrient = other_nutrient_option.borrow();
        let from_unit = other.output_unit;
        let to_unit = self.output_unit;
        let conversion_factor = other_nutrient.convert(from_unit, to_unit).unwrap();
        self.value.partial_cmp(&(other.value * conversion_factor))
    }
}

impl Eq for NutrientAmount {}

impl Ord for NutrientAmount {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.nutrient.is_none() && other.nutrient.is_none() {
            return Ordering::Equal;
        }

        else if self.nutrient.is_some() && other.nutrient.is_none() {
            return Ordering::Greater
        }

        if self.nutrient.is_none() && other.nutrient.is_none() {
            return Ordering::Less
        }

        let id_cmp = self
            .get_nutrient()
            .unwrap()
            .borrow()
            .get_name()
            .cmp(&other.get_nutrient().unwrap().borrow().get_name());
        if id_cmp != Ordering::Equal {
            return id_cmp;
        }

        let other_nutrient_option = other.nutrient.clone().unwrap();
        let other_nutrient = other_nutrient_option.borrow();
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
        if self.nutrient.is_some() != rhs.nutrient.is_some() {
            panic!(
                "Tried to add an empty nutrient to a non-empty nutrient: {:#?} + {:#?}",
                self.nutrient, rhs.nutrient
            );
        }

        let rhs_nutrient_option = rhs.nutrient.clone().unwrap();
        let rhs_nutrient = rhs_nutrient_option.borrow();
        let conversion_factor = rhs_nutrient
            .convert(rhs.output_unit, self.output_unit)
            .unwrap();
        Self {
            value: self.value + rhs.value * conversion_factor,
            nutrient: self.nutrient,
            output_unit: self.output_unit,
        }
    }
}

impl Sub for NutrientAmount {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        if self.nutrient.is_some() != rhs.nutrient.is_some() {
            panic!(
                "Tried to add an empty nutrient to a non-empty nutrient: {:#?} + {:#?}",
                self.nutrient, rhs.nutrient
            );
        }

        let rhs_nutrient_option = rhs.nutrient.clone().unwrap();
        let rhs_nutrient = rhs_nutrient_option.borrow();
        let conversion_factor = rhs_nutrient
            .convert(rhs.output_unit, self.output_unit)
            .unwrap();
        Self {
            value: self.value - rhs.value * conversion_factor,
            nutrient: self.nutrient,
            output_unit: self.output_unit,
        }
    }
}

impl Mul<f64> for NutrientAmount {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        match self.nutrient.clone() {
            Some(nutrient) => {
                let nutrient_borrowed = nutrient.borrow();
                let main_unit = nutrient_borrowed.get_main_unit();
                Self::from_rc_refcell(self.value * rhs, self.get_nutrient(), main_unit).unwrap()
            },
            None => {
                Self::from_rc_refcell(0f64 * rhs, None, NutrientUnit::None).unwrap()
            },
        }
    }
}

impl Div<f64> for NutrientAmount {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        match self.nutrient.clone() {
            Some(nutrient) => {
                let nutrient_borrowed = nutrient.borrow();
                let main_unit = nutrient_borrowed.get_main_unit();
                Self::from_rc_refcell(self.value / rhs, self.get_nutrient(), main_unit).unwrap()
            },
            None => {
                Self::from_rc_refcell(0f64 * rhs, None, NutrientUnit::None).unwrap()
            },
        }
    }
}

impl Sum<NutrientAmount> for NutrientAmount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut iter = iter.peekable();

        let nutrient = match iter.peek() {
            Some(n) => n.clone(),
            None => return NutrientAmount::new(0f64, None, NutrientUnit::None).unwrap(),
            // None => panic!("Nothing in iter"),
            // None => return NutrientAmount::new(0.0, Default::default()),
        };
        // println!("nutrient: {:#?}", nutrient);

        iter.fold(
            NutrientAmount::new(
                0.0,
                nutrient.get_nutrient().map(|nutrient| nutrient.borrow().clone()),
                nutrient.get_output_unit(),
            )
            .unwrap(),
            |acc, n| acc + n.clone(),
        )
    }
}
