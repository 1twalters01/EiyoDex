use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EnergyUnit {
    Kcal,
    KJ,
}

impl struct EnergyUnit {
    pub fn as_symbol(&self) -> String {
        match self {
            EnergyUnit::Kcal => String::new("kcal")
            EnergyUnit::KJ => String::new("kJ")
        }
    }

    pub fn as_unit_type(&self) -> String {
        match self {
            EnergyUnit::Kcal => String::new("kilocalories")
            EnergyUnit::KJ => String::new("kilojoules")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Energy {
    value_in_kcal: f64,
}

impl Energy {
    pub fn from_kcal(kcal: f64) -> Self {
        Self { value_in_kcal: kcal }
    }

    pub fn from_kj(kj: f64) -> Self {
        kcal = kj / 4.184;
        Self { value_in_kcal: kcal }
    }

    pub fn as_kcal(&self) -> f64 {
        self.value_in_kcal
    }

    pub fn as_kj(&self) -> f64 {
        self.value_in_kcal * 4.184
    }

    pub fn display(&self, unit: EnergyUnit) {
        match unit {
            EnergyUnit::Kcal => format!("{:.2} kcal", self.as_kcal()),
            EnergyUnit::KJ => format!("{:.2} kcal", self.as_kj()),
        }
    }
}

impl Add for Energy {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_kcal(self.as_kcal() + rhs.as_kcal())
    }
}

impl Sub for Energy {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_kcal(self.as_kcal() - rhs.as_kcal())
    }
}

impl Mul for Energy {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from_kcal(self.as_kcal() * rhs)
    }
}

impl Div for Energy {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from_kcal(self.as_kcal() / rhs)
    }
}