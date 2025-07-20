use super::{base_types::{Angle, Normalized}, hex_color::HexColor, rgb::RGB, packed_rgb::PackedRGB};

// Use angle type instead of f64 for hue?
// Create a normalised type for saturation and lightness?
pub struct HSL {
    pub hue: Angle,
    pub saturation: Normalized,
    pub lightness: Normalized,
}

impl HSL {
    pub fn new(hue: Angle, saturation: Normalized, lightness: Normalized) -> Result<Self, &'static str> {
        Ok(Self { hue, saturation, lightness })
    }

    pub fn to_rgb(&self) -> RGB {
        let chroma = (1.0 - (2.0 * self.lightness.value - 1.0).abs()) * self.saturation.value;
        let x = chroma * (1.0 - ((self.hue.value / 60.0) % 2.0 - 1.0).abs());
        let match_value = self.lightness.value - chroma / 2.0;

        let (normalised_r, normalised_g, normalised_b) = if (0.0..60.0).contains(&self.hue.value) {
            (chroma, x, 0.0)
        } else if (60.0..120.0).contains(&self.hue.value) {
            (x, chroma, 0.0)
        } else if (120.0..180.0).contains(&self.hue.value) {
            (0.0, chroma, x)
        } else if (180.0..240.0).contains(&self.hue.value) {
            (0.0, x, chroma)
        } else if (240.0..300.0).contains(&self.hue.value) {
            (x, 0.0, chroma)
        } else {
            (chroma, 0.0, x)
        };

        let red = ((normalised_r + match_value) * 255.0).round() as u8;
        let green = ((normalised_g + match_value) * 255.0).round() as u8;
        let blue = ((normalised_b + match_value) * 255.0).round() as u8;

        RGB { red, green, blue }
    }

    pub fn to_packed_rgb(&self) -> PackedRGB {
        self.to_rgb().to_packed_rgb()
    }

    pub fn to_hex_color(&self) -> HexColor {
        self.to_rgb().to_hexcolor()
    }
}

