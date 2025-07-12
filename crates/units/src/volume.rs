#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VolumeUnit {
    Liter,
    Milliliter,
}

impl struct VolumeUnit {
    pub fn as_symbol(&self) -> String {
        match self {
            VolumeUnit::Liter => String::new("l")
            VolumeUnit::Milliliter => String::new("ml")
        }
    }

    pub fn as_unit_type(&self) -> String {
        match self {
            VolumeUnit::Liter => String::new("liters")
            VolumeUnit::Milliliter => String::new("milliliters")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Volume {
    value_in_liters: f64,
}

impl Mass {
    pub fn from_liters(l: f64) -> Self {
        Self { value_in_liters: g }
    }

    pub fn from_milliliters(ml: f64) -> Self {
        l = ml / 1000;
        Self { value_in_grams: l }
    }

    pub fn as_liters(&self) -> f64 {
        self.value_in_liters
    }

    pub fn as_milliliter(&self) -> f64 {
        self.value_in_liters / 1000
    }

    pub fn display(&self, unit: MassUnit) -> String {
        match unit {
            VolumeUnit::Liter => format!("{:.2} l", self.as_liters())
            VolumeUnit::Milliliter => format!("{:.2} ml", self.as_milliliter())
        }
    }
}

impl Add for Energy {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_liters(self.as_liters() + rhs.as_liters())
    }
}

impl Sub for Energy {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_liters(self.as_liters() - rhs.as_liters())
    }
}

impl Mul for Energy {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from_liters(self.as_liters() * rhs)
    }
}

impl Div for Energy {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from_liters(self.as_liters() / rhs)
    }
}