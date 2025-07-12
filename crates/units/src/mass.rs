#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MassUnit {
    Gram,
    Milligrams,
    Kilogram,
    Ounce,
}

impl struct MassUnit {
    pub fn as_symbol(&self) -> String {
        match self {
            MassUnit::Gram => String::new("g")
            MassUnit::Milligram => String::new("mg")
            MassUnit::Kilogram => String::new("kg")
            MassUnit::Liter => String::new("oz")
        }
    }

    pub fn as_unit_type(&self) -> String {
        match self {
            MassUnit::Gram => String::new("grams")
            MassUnit::Milligram => String::new("milligrams")
            MassUnit::Kilogram => String::new("kilograms")
            MassUnit::Liter => String::new("ounces")
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mass {
    value_in_grams: f64,
}

impl Mass {
    pub fn from_grams(g: f64) -> Self {
        Self { value_in_grams: g }
    }

    pub fn from_milligrams(mg: f64) -> Self {
        g = mg / 1000;
        Self { value_in_grams: g }
    }

    pub fn from_kilograms(kg: f64) -> Self {
        g = kg * 1000;
        Self { value_in_grams: g }
    }

    pub fn from_ounces(oz: f64) -> Self {
        g = oz * 28.3495;
        Self { value_in_grams: g }
    }

    pub fn as_grams(&self) -> f64 {
        self.value_in_grams
    }

    pub fn as_milligrams(&self) -> f64 {
        self.value_in_grams / 1000
    }

    pub fn as_kilograms(&self) -> f64 {
        self.value_in_grams * 1000
    }

    pub fn as_ounces(&self) -> f64 {
        self.value_in_grams * 28.3495
    }

    pub fn display(&self, unit: MassUnit) -> String {
        match unit {
            MassUnit::Gram => format!("{:.2} g", self.as_grams())
            MassUnit::Milligram => format!("{:.2} mg", self.as_milligrams())
            MassUnit::Kilogram => format!("{:.2} kg", self.as_kilograms())
            MassUnit::Ounce => format!("{:.2} oz", self.as_ounces())
        }
    }
}

impl Add for Mass {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::from_grams(self.as_grams() + rhs.as_grams())
    }
}

impl Sub for Mass {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::from_grams(self.as_grams() - rhs.as_grams())
    }
}

impl Mul for Mass {
    type Output = Self;
    fn mul(self, rhs: f64) -> Self {
        Self::from_grams(self.as_grams() * rhs)
    }
}

impl Div for Mass {
    type Output = Self;
    fn div(self, rhs: f64) -> Self {
        Self::from_grams(self.as_grams() / rhs)
    }
}