use super::rgb::RGB;

// Use angle type instead of f64 for hue?
// Create a normalised type for saturation and lightness?
pub struct HSL {
    hue: f64,
    saturation: f64,
    lightness: f64,
}

impl HSL {
    pub fn new(hue: f64, saturation: f64, lightness: f64) -> Self {
        Self { hue, saturation, lightness }
    }

    pub fn to_rgb(&self) -> RGB {
        let chroma = (1.0 - (2.0 * self.lightness - 1.0).abs()) * self.saturation;
        let x = chroma * (1.0 - ((self.hue / 60.0) % 2.0 - 1.0).abs());
        let match_value = self.lightness - chroma / 2.0;

        let (normalised_r, normalised_g, normalised_b) = if (0.0..60.0).contains(&self.hue) {
            (chroma, x, 0.0)
        } else if (60.0..120.0).contains(&self.hue) {
            (x, chroma, 0.0)
        } else if (120.0..180.0).contains(&self.hue) {
            (0.0, chroma, x)
        } else if (180.0..240.0).contains(&self.hue) {
            (0.0, x, chroma)
        } else if (240.0..300.0).contains(&self.hue) {
            (x, 0.0, chroma)
        } else {
            (chroma, 0.0, x)
        };

        let red = ((normalised_r + match_value) * 255.0).round() as u8;
        let green = ((normalised_g + match_value) * 255.0).round() as u8;
        let blue = ((normalised_b + match_value) * 255.0).round() as u8;

        RGB::new(red, green, blue)
    }
}

