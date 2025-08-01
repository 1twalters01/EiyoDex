use super::{hex_color::HexColor, hsl::HSL, rgb::RGB};

pub struct PackedRGB {
    pub value: u32,
}

impl PackedRGB {
    pub fn new(value: u32) -> Result<PackedRGB, &'static str> {
        let red = (value >> 16) & 255;
        let green = (value >> 8) & 255;
        let blue = value & 255;

        if red > 255 { return Err("Invalid red component") }
        if green > 255 { return Err("Invalid green component") }
        if blue > 255 { return Err("Invalid blue component") }

        Ok(PackedRGB { value })
    }

    pub fn to_rgb(&self) -> RGB {
        let red = ((self.value >> 16) & 255) as u8;
        let green = ((self.value >> 8) & 255) as u8;
        let blue = (self.value & 255) as u8;

        RGB::new(red, green, blue)
    }

    pub fn to_hsl(&self) -> HSL {
        self.to_rgb().to_hsl()
    }

    pub fn to_hex_color(&self) -> HexColor {
        let hex = format!("#{:02X}", self.value);
        HexColor { value: hex }
    }
}
