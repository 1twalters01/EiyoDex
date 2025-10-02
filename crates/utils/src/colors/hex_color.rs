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

    pub fn get_value(&self) -> String {
        self.value.clone()
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

#[cfg(test)]
mod tests {
    use crate::base_types::{Angle, Normalized};

    use super::*;

    #[test]
    fn test_invalid_hex() {
        let hexcode = "#GGGGGG".to_string();
        assert!(HexColor::new(hexcode).is_err());
    }

    #[test]
    fn test_black_hex() {
        let hexcode = "#000000".to_string();
        assert!(HexColor::new(hexcode).is_ok());
    }

    #[test]
    fn test_white_hex() {
        let hexcode = "#FFFFFF".to_string();
        assert!(HexColor::new(hexcode).is_ok());
    }

    #[test]
    fn test_normal_hex() {
        let hexcode = "#e2a9AC".to_string();
        assert!(HexColor::new(hexcode).is_ok());
    }

    #[test]
    fn test_black_to_rgb() {
        let hexcode = "#000000".to_string();
        let hexcolor = HexColor::new(hexcode).unwrap();
        let created_rgb = hexcolor.to_rgb();
        let manual_rgb = RGB::new(0, 0, 0);
        assert_eq!(created_rgb, manual_rgb);
    }

    #[test]
    fn test_white_to_rgb() {
        let hexcode = "#FFFFFF".to_string();
        let hexcolor = HexColor::new(hexcode).unwrap();
        let created_rgb = hexcolor.to_rgb();
        let manual_rgb = RGB::new(255, 255, 255);
        assert_eq!(created_rgb, manual_rgb);
    }

    #[test]
    fn test_normal_to_rgb() {
        let hexcode = "#d27a1c".to_string();
        let hexcolor = HexColor::new(hexcode).unwrap();
        let created_rgb = hexcolor.to_rgb();
        let manual_rgb = RGB::new(210, 122, 28);
        assert_eq!(created_rgb, manual_rgb);
    }

    #[test]
    fn test_black_to_packed_rgb() {
        let hexcode = "#000000".to_string();
        let hexcolor = HexColor::new(hexcode).unwrap();
        let created_packed_rgb = hexcolor.to_packed_rgb();
        let manual_packed_rgb = PackedRGB::new(0).unwrap();
        assert_eq!(created_packed_rgb, manual_packed_rgb);
    }

    #[test]
    fn test_white_to_packed_rgb() {
        let hexcode = "#FFFFFF".to_string();
        let hexcolor = HexColor::new(hexcode).unwrap();
        let created_packed_rgb = hexcolor.to_packed_rgb();
        let manual_packed_rgb = PackedRGB::new(16_777_215).unwrap();
        assert_eq!(created_packed_rgb, manual_packed_rgb);
    }

    #[test]
    fn test_normal_to_packed_rgb() {
        let hexcode = "#17e91e".to_string();
        let hexcolor = HexColor::new(hexcode).unwrap();
        let created_packed_rgb = hexcolor.to_packed_rgb();
        let manual_packed_rgb = PackedRGB::new(1_567_006).unwrap();
        assert_eq!(created_packed_rgb, manual_packed_rgb);
    }

    #[test]
    fn test_black_to_hsl() {
        let hexcode = "#000000".to_string();
        let hexcolor = HexColor::new(hexcode).unwrap();
        let created_hsl = hexcolor.to_hsl();
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(0f64).unwrap(),
        );
        let manual_hsl = HSL::new(hue, saturation, lightness);
        assert_eq!(created_hsl, manual_hsl.unwrap());
    }

    #[test]
    fn test_white_to_hsl() {
        let hexcode = "#FFFFFF".to_string();
        let hexcolor = HexColor::new(hexcode).unwrap();
        let created_hsl = hexcolor.to_hsl();
        let (hue, saturation, lightness) = (
            Angle::new(0f64),
            Normalized::new(0f64).unwrap(),
            Normalized::new(1f64).unwrap(),
        );
        let manual_hsl = HSL::new(hue, saturation, lightness);
        assert_eq!(created_hsl, manual_hsl.unwrap());
    }

    #[test]
    fn test_normal_to_hsl() {
        let hexcode = "#63c21a".to_string();
        let hexcolor = HexColor::new(hexcode).unwrap();
        let created_hsl = hexcolor.to_hsl();
        let (hue, saturation, lightness) = (
            Angle::new(93.92857142857143),
            Normalized::new(0.7636363636363637).unwrap(),
            Normalized::new(0.4313725490196078).unwrap(),
        );
        let manual_hsl = HSL::new(hue, saturation, lightness);
        assert_eq!(created_hsl, manual_hsl.unwrap());
    }
}
