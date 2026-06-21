use std::{cell::RefCell, cmp::Ordering, iter::Sum, ops::{Add, Div, Mul, Sub}, rc::Rc};

use units::{
    duration::quantity::DurationQuantity, energy::quantity::EnergyQuantity,
};

use crate::exercise::Exercise;

#[derive(Debug, Clone)]
pub struct ExerciseQuantity {
    duration_quantity: DurationQuantity,
    exercise: Rc<RefCell<Exercise>>,
}

impl ExerciseQuantity {
    pub fn new(duration_quantity: DurationQuantity, exercise: Exercise) -> Self {
        Self {
            duration_quantity,
            exercise: Rc::new(RefCell::new(exercise)),
        }
    }

    pub fn from_rc_refcell(duration_quantity: DurationQuantity, exercise: Rc<RefCell<Exercise>>) -> Self {
        Self {
            duration_quantity,
            exercise,
        }
    }

    pub fn get_duration_quantity(&self) -> DurationQuantity {
        self.duration_quantity.clone()
    }

    pub fn set_duration_quantity(&mut self, duration_quantity: DurationQuantity) {
        self.duration_quantity = duration_quantity;
    }

    pub fn get_exercise(&self) -> Rc<RefCell<Exercise>> {
        self.exercise.clone()
    }

    pub fn set_exercise_rc_refcell(&mut self, exercise: Rc<RefCell<Exercise>>) {
        self.exercise = exercise;
    }

    pub fn set_exercise(&mut self, exercise: Exercise) {
        self.exercise = Rc::new(RefCell::new(exercise));
    }

    pub fn get_calories(&self) -> EnergyQuantity {
        self.exercise.borrow().get_power_quantity().clone() * self.get_duration_quantity()
    }
}

impl PartialEq for ExerciseQuantity {
    fn eq(&self, other: &Self) -> bool {
        if !(self.get_exercise() == other.get_exercise()) { return false }
        self.duration_quantity == other.duration_quantity
    }
}

impl Eq for ExerciseQuantity {}

impl Ord for ExerciseQuantity {
    fn cmp(&self, other: &Self) -> Ordering {
        let self_exercise = self.get_exercise();
        let other_exercise = other.get_exercise();

        if Rc::ptr_eq(&self_exercise, &other_exercise) {
            let lhs = self.duration_quantity;
            let rhs = other.duration_quantity;

            lhs.partial_cmp(&rhs).unwrap_or(Ordering::Equal)
        } else {
            self_exercise
                .borrow()
                .get_name()
                .cmp(&other_exercise.borrow().get_name())
        }
    }
}

impl PartialOrd for ExerciseQuantity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Add for ExerciseQuantity {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            exercise: self.exercise,
            duration_quantity: self.duration_quantity + rhs.duration_quantity,
        }
    }
}

impl Sub for ExerciseQuantity {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            exercise: self.exercise,
            duration_quantity: self.duration_quantity - rhs.duration_quantity,
        }
    }
}

impl Mul<f64> for ExerciseQuantity {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from_rc_refcell(self.duration_quantity * rhs, self.exercise)
    }
}

impl Div<f64> for ExerciseQuantity {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from_rc_refcell(self.duration_quantity / rhs, self.exercise)
    }
}

impl Sum<ExerciseQuantity> for ExerciseQuantity {
    fn sum<I: Iterator<Item = Self>>(mut iter: I) -> Self {
        if let Some(first) = iter.next() {
            iter.fold(first, |acc, n| acc + n)
        } else {
            panic!("Zero elements in list")
        }
    }
}
