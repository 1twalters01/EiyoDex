use super::{hex_color::HexColor, packed_rgb::PackedRGB, rgb::RGB};
use crate::base_types::{Angle, Normalized};

#[derive(Debug, PartialEq)]
pub struct HSL {
    pub hue: Angle,
    pub saturation: Normalized,
    pub lightness: Normalized,
}

impl HSL {
    pub fn new(
        hue: Angle,
        saturation: Normalized,
        lightness: Normalized,
    ) -> Result<Self, &'static str> {
        Ok(Self {
            hue,
            saturation,
            lightness,
        })
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
        self.to_rgb().to_hex_color()
    }
}

#[cfg(test)]
mod tests {
    use crate::base_types::{Angle, Normalized};

    use super::*;

    #[test]
    fn test_black_hex() {
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(0f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness);
        assert!(hsl.is_ok());
    }

    #[test]
    fn test_white_hex() {
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(1f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness);
        assert!(hsl.is_ok());
    }

    #[test]
    fn test_normal_hex() {
        let (hue, saturation, lightness) = (
            Angle::new(15.8f64),
            Normalized::new(0.42f64).unwrap(),
            Normalized::new(0.89f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness);
        assert!(hsl.is_ok());
    }

    #[test]
    fn test_black_to_rgb() {
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(0f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness).unwrap();
        let created_rgb = hsl.to_rgb();
        let manual_rgb = RGB::new(0, 0, 0);
        assert_eq!(created_rgb, manual_rgb);
    }

    #[test]
    fn test_white_to_rgb() {
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(1f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness).unwrap();
        let created_rgb = hsl.to_rgb();
        let manual_rgb = RGB::new(255, 255, 255);
        assert_eq!(created_rgb, manual_rgb);
    }

    #[test]
    fn test_normal_to_rgb() {
        let (hue, saturation, lightness) = (
            Angle::new(86.44f64),
            Normalized::new(0.24f64).unwrap(),
            Normalized::new(0.38f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness).unwrap();
        let created_rgb = hsl.to_rgb();
        let manual_rgb = RGB::new(100, 120, 74);
        assert_eq!(created_rgb, manual_rgb);
    }

    #[test]
    fn test_black_to_packed_rgb() {
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(0f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness).unwrap();
        let created_packed_rgb = hsl.to_packed_rgb();
        let manual_packed_rgb = PackedRGB::new(0).unwrap();
        assert_eq!(created_packed_rgb, manual_packed_rgb);
    }

    #[test]
    fn test_white_to_packed_rgb() {
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(1f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness).unwrap();
        let created_packed_rgb = hsl.to_packed_rgb();
        let manual_packed_rgb = PackedRGB::new(16_777_215).unwrap();
        assert_eq!(created_packed_rgb, manual_packed_rgb);
    }

    #[test]
    fn test_normal_to_packed_rgb() {
        let (hue, saturation, lightness) = (
            Angle::new(178.3f64),
            Normalized::new(0.5f64).unwrap(),
            Normalized::new(0.5f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness).unwrap();
        let created_packed_rgb = hsl.to_packed_rgb();
        let manual_packed_rgb = PackedRGB::new(4_243_388).unwrap();
        assert_eq!(created_packed_rgb, manual_packed_rgb);
    }

    #[test]
    fn test_black_to_hex() {
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(0f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness).unwrap();
        let created_hex = hsl.to_hex_color();
        let manual_hex = HexColor::new("#000000".to_string()).unwrap();
        assert_eq!(created_hex, manual_hex);
    }

    #[test]
    fn test_white_to_hex() {
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(1f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness).unwrap();
        let created_hex = hsl.to_hex_color();
        let manual_hex = HexColor::new("#FFFFFF".to_string()).unwrap();
        assert_eq!(created_hex, manual_hex);
    }

    #[test]
    fn test_normal_to_hex() {
        let (hue, saturation, lightness) = (
            Angle::new(236f64),
            Normalized::new(0.5f64).unwrap(),
            Normalized::new(0.4f64).unwrap(),
        );
        let hsl = HSL::new(hue, saturation, lightness).unwrap();
        let created_hex = hsl.to_hex_color();
        let manual_hex = HexColor::new("#333a99".to_string()).unwrap();
        assert_eq!(created_hex, manual_hex);
    }
}
