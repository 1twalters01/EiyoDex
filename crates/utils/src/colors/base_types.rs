pub struct Angle {
    pub value: f64
}

impl Angle {
    pub fn new(degrees: f64) -> Self {
        let value = degrees.rem_euclid(360.0);
        Angle { value }
    }
}

pub struct Normalized {
    pub value: f64
}

impl Normalized {
    pub fn new(value: f64) -> Result<Self, &'static str> {
        if (0.0..=1.0).contains(&value) {
            Ok(Self{ value })
        } else {
            Err("Value must be between 0 and 1")
        }
    }
}


