use super::{hex_color::HexColor, hsl::HSL, rgb::RGB};

#[derive(Debug, PartialEq)]
pub struct PackedRGB {
    pub value: u32,
}

impl PackedRGB {
    pub fn new(value: u32) -> Result<PackedRGB, &'static str> {
        if value > 0xFFFFFF {
            return Err("Invalid value");
        }

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
        let hex = format!("#{:06X}", self.value);
        HexColor { value: hex }
    }
}
