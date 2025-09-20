use super::{hsl::HSL, packed_rgb::PackedRGB, rgb::RGB};

pub struct HexColor {
    pub value: String,
}

impl HexColor {
    pub fn new(hex: String) -> Result<Self, &'static str> {
        if !hex.starts_with("#") {
            return Err("Invalid hex");
        }
        let _red = u8::from_str_radix(&hex[1..3], 16).map_err(|_| "Invalid red component")?;
        let _greed = u8::from_str_radix(&hex[3..5], 16).map_err(|_| "Invalid green component")?;
        let _blue = u8::from_str_radix(&hex[5..7], 16).map_err(|_| "Invalid blue component")?;

        Ok(Self { value: hex })
    }

    pub fn to_rgb(&self) -> RGB {
        let red = u8::from_str_radix(&self.value[1..3], 16)
            .map_err(|_| "Invalid red component")
            .unwrap();
        let green = u8::from_str_radix(&self.value[3..5], 16)
            .map_err(|_| "Invalid green component")
            .unwrap();
        let blue = u8::from_str_radix(&self.value[5..7], 16)
            .map_err(|_| "Invalid blue component")
            .unwrap();

        RGB { red, blue, green }
    }

    pub fn to_packed_rgb(&self) -> PackedRGB {
        self.to_rgb().to_packed_rgb()
    }

    pub fn to_hsl(&self) -> HSL {
        self.to_rgb().to_hsl()
    }
}
