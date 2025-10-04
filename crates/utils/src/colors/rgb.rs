use super::{hex_color::HexColor, hsl::HSL, packed_rgb::PackedRGB};
use crate::base_types::{Angle, Normalized};

#[derive(Debug, PartialEq)]
pub struct RGB {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub fn normalise_angle(angle: f64) -> f64 {
    if angle < 0.0 {
        return normalise_angle(angle + 360.0);
    } else if angle > 360.0 {
        return normalise_angle(angle - 360.0);
    } else {
        return angle;
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

        let lightness = Normalized::new((max + min) / 2.0).unwrap();

        let saturation = if chroma == 0.0 {
            Normalized::new(0.0).unwrap()
        } else {
            Normalized::new(chroma / (1.0 - (2.0 * lightness.value - 1.0).abs())).unwrap()
        };

        let hue = if chroma == 0.0 {
            Angle::new(0.0)
        } else {
            if max == normalised_r {
                let segment = (normalised_g - normalised_b) / chroma;
                Angle::new(normalise_angle(60.0 * (segment % 6.0)))
            } else if max == normalised_g {
                let segment = (normalised_b - normalised_r) / chroma;
                Angle::new(normalise_angle(60.0 * (segment + 2.0)))
            } else {
                let segment = (normalised_r - normalised_g) / chroma;
                Angle::new(normalise_angle(60.0 * (segment + 4.0)))
            }
        };

        HSL {
            hue,
            saturation,
            lightness,
        }
    }

    pub fn to_packed_rgb(&self) -> PackedRGB {
        let red: u32 = (self.red as u32) << 16;
        let green: u32 = (self.green as u32) << 8;
        PackedRGB {
            value: red + green + self.blue as u32,
        }
    }

    pub fn to_hex_color(&self) -> HexColor {
        let hex = format!("#{:02X}{:02X}{:02X}", self.red, self.green, self.blue);
        HexColor { value: hex }
    }
}

#[cfg(test)]
mod tests {
    use crate::base_types::{Angle, Normalized};

    use super::*;
    #[test]
    fn test_black_to_hsl() {
        let (red, green, blue) = (0, 0, 0);
        let rgb = RGB::new(red, green, blue);
        let created_hsl = rgb.to_hsl();
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(0f64).unwrap(),
        );
        let manual_hsl = HSL::new(hue, saturation, lightness);
        assert_eq!(created_hsl, manual_hsl.unwrap());
    }

    #[test]
    fn test_white_hsl() {
        let (red, green, blue) = (255, 255, 255);
        let rgb = RGB::new(red, green, blue);
        let created_hsl = rgb.to_hsl();
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(1f64).unwrap(),
        );
        let manual_hsl = HSL::new(hue, saturation, lightness);
        assert_eq!(created_hsl, manual_hsl.unwrap());
    }

    #[test]
    fn test_normal_hsl() {
        let (red, green, blue) = (17, 200, 56);
        let rgb = RGB::new(red, green, blue);
        let created_hsl = rgb.to_hsl();
        let (hue, saturation, lightness) = (
            Angle::new(132.78688524590163),
            Normalized::new(0.8433179723502304).unwrap(),
            Normalized::new(0.42549019607843136).unwrap(),
        );
        let manual_hsl = HSL::new(hue, saturation, lightness);
        assert_eq!(created_hsl, manual_hsl.unwrap());
    }

    #[test]
    fn test_black_to_packed_rgb() {
        let (red, green, blue) = (0, 0, 0);
        let rgb = RGB::new(red, green, blue);
        let created_packed_rgb = rgb.to_packed_rgb();
        let manual_packed_rgb = PackedRGB::new(0).unwrap();
        assert_eq!(created_packed_rgb, manual_packed_rgb);
    }

    #[test]
    fn test_white_packed_rgb() {
        let (red, green, blue) = (255, 255, 255);
        let rgb = RGB::new(red, green, blue);
        let created_packed_rgb = rgb.to_packed_rgb();
        let manual_packed_rgb = PackedRGB::new(16_777_215).unwrap();
        assert_eq!(created_packed_rgb, manual_packed_rgb);
    }

    #[test]
    fn test_normal_packed_rgb() {
        let (red, green, blue) = (47, 213, 9);
        let rgb = RGB::new(red, green, blue);
        let created_packed_rgb = rgb.to_packed_rgb();
        let manual_packed_rgb = PackedRGB::new(3_134_729).unwrap();
        assert_eq!(created_packed_rgb, manual_packed_rgb);
    }

    #[test]
    fn test_black_to_hex() {
        let (red, green, blue) = (0, 0, 0);
        let rgb = RGB::new(red, green, blue);
        let created_hex = rgb.to_hex_color();
        let manual_hex = HexColor::new("#000000".to_string()).unwrap();
        assert_eq!(created_hex, manual_hex);
    }

    #[test]
    fn test_white_hex() {
        let (red, green, blue) = (255, 255, 255);
        let rgb = RGB::new(red, green, blue);
        let created_hex = rgb.to_hex_color();
        let manual_hex = HexColor::new("#FFFFFF".to_string()).unwrap();
        assert_eq!(created_hex, manual_hex);
    }

    #[test]
    fn test_normal_hex() {
        let (red, green, blue) = (37, 14, 254);
        let rgb = RGB::new(red, green, blue);
        let created_hex = rgb.to_hex_color();
        let manual_hex = HexColor::new("#250EFE".to_string()).unwrap();
        assert_eq!(created_hex, manual_hex);
    }
}
