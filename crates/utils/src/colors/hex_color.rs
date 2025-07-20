use super::{base_types::{Angle, Normalized}, hsl::HSL, rgb::RGB, packed_rgb::PackedRGB};

pub struct HexColor {
    pub value: String,
}

impl HexColor {
    pub fn new(value: String) -> Result<Self, &'static str> {
        let red = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red component")?;
        let greed = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green component")?;
        let blue = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue component")?;

        Self { value }
    }

    pub fn to_rgb(&self) -> RGB {
        let red = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red component").unwrap();
        let greed = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green component").unwrap();
        let blue = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue component").unwrap();

        RGB { red, blue, green }
    }

    pub fn to_packed_rgb(&self) -> PackedRGB {
        self.to_rgb().to_packed_rgb()
    }

    pub fn to_hsl(&self) -> HSL {
        self.to_rgb().to_hsl()
    }
}