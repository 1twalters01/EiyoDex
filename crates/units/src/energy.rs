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
            EnergyUnit::Kcal => String::new("kilocalorie")
            EnergyUnit::KJ => String::new("kilojoule")
        }
    }

    pub fn as_unit_type_plural(&self) -> String {
        match self {
            EnergyUnit::Kcal => String::new("kilocalories")
            EnergyUnit::KJ => String::new("kilojoules")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Energy {
    value: f64,
    unit: EnergyUnit,
}

impl Energy {
    pub fn new(value: f64, unit: EnergyUnit) -> Self {
        Self { value, unit }
    }

    pub fn from_kcal(kcal: f64) -> Self {
        Self::new(kcal, EnergyUnit::Kcal)
    }

    pub fn from_kj(kj: f64) -> Self {
        Self::new(kj, EnergyUnit::KJ)
    }

    pub fn as_kcal(&self) -> f64 {
        match self.unit {
            EnergyUnit::Kcal => self.value,
            EnergyUnit::KJ => self.value / 4.184,
        }
    }

    pub fn as_kj(&self) -> f64 {
        match self.unit {
            EnergyUnit::Kcal => self.value * 4.184,
            EnergyUnit::KJ => self.value,
        }
    }

    pub fn to_unit(&self, unit: EnergyUnit) -> Self {
        let value = match unit {
            EnergyUnit::Kcal => Self::as_kcal(),
            EnergyUnit::KJ => Self::as_kj(),
        }
        Self { value, unit }
    }

    pub fn to_kcal(&self) -> Self {
        Self::to_unit(EnergyUnit::Kcal)
    }

    pub fn to_kj(&self) -> Self {
        Self::to_unit(EnergyUnit::KJ)
    }

    pub fn get_symbol(&self) -> String {
        self.unit.as_symbol
    }

    pub get_unit_type(&self) -> String {
        self.unit.as_unit_type
    }

    pub get_unit_type_plural(&self) -> String {
        self.unit.as_unit_type_plural
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