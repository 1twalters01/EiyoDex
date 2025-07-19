use super::hsl::HSL;

pub struct RGB {
    red: u8,
    green: u8,
    blue: u8,
}

pub fn normalise_angle(angle: f64) -> f64 {
    if angle < 0.0 {
        return normalise_angle(angle + 360.0)
    } else if angle > 360.0 {
        return normalise_angle(angle - 360.0)
    } else {
        return angle
    }
}

impl RGB {
    pub fn new(red: u8, green: u8, blue: u8) -> Self {
        Self { red, green, blue }
    }

    pub fn to_hsl(&self) -> HSL {
        let normalised_r = self.red as f64 / 255.0;
        let normalised_g = self.green as f64 / 255.0;
        let normalised_b = self.blue as f64 / 255.0;

        let max = normalised_r.max(normalised_g).max(normalised_b);
        let min = normalised_r.min(normalised_g).min(normalised_b);
        let chroma = max - min;

        let l = (max + min) / 2.0;

        let s = if chroma == 0.0 {
            0.0
        } else {
            chroma / (1.0 - (2.0 * l - 1.0).abs())
        };

        let h = if chroma == 0.0 {
            0.0
        } else {
            if max == normalised_r {
                let segment = (normalised_g - normalised_b) / chroma;
                normalise_angle(60.0 * (segment % 6.0))
            } else if max == normalised_g {
                let segment = (normalised_b - normalised_r) / chroma;
                normalise_angle(60.0 * (segment + 2.0))
            } else {
                let segment = (normalised_r - normalised_g) / chroma;
                normalise_angle(60.0 * (segment + 4.0))
            }
        };

        HSL::new(h, s, l)
    }
}
