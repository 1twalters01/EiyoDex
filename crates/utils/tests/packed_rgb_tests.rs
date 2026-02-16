use utils::{
    base_types::{angle::Angle, normalized::Normalized},
    colors::{hex_color::HexColor, hsl::HSL, packed_rgb::PackedRGB, rgb::RGB},
};

#[test]
fn test_invalid_packed_rgb() {
    let packed_rgb = PackedRGB::new(16_777_216);
    assert!(packed_rgb.is_err());
}

#[test]
fn test_black_packed_rgb() {
    let packed_rgb = PackedRGB::new(0);
    assert!(packed_rgb.is_ok());
}

#[test]
fn test_white_packed_rgb() {
    let packed_rgb = PackedRGB::new(16_777_215);
    assert!(packed_rgb.is_ok());
}

#[test]
fn test_normal_packed_rgb() {
    let packed_rgb = PackedRGB::new(1_557_080);
    assert!(packed_rgb.is_ok());
}

#[test]
fn test_black_to_rgb() {
    let packed_rgb = PackedRGB::new(0).unwrap();
    let created_rgb = packed_rgb.to_rgb();
    let manual_rgb = RGB::new(0, 0, 0);
    assert_eq!(created_rgb, manual_rgb);
}

#[test]
fn test_white_to_rgb() {
    let packed_rgb = PackedRGB::new(16_777_215).unwrap();
    let created_rgb = packed_rgb.to_rgb();
    let manual_rgb = RGB::new(255, 255, 255);
    assert_eq!(created_rgb, manual_rgb);
}

#[test]
fn test_normal_to_rgb() {
    let packed_rgb = PackedRGB::new(8_172_256).unwrap();
    let created_rgb = packed_rgb.to_rgb();
    let manual_rgb = RGB::new(124, 178, 224);
    assert_eq!(created_rgb, manual_rgb);
}

#[test]
fn test_black_to_hsl() {
    let packed_rgb = PackedRGB::new(0).unwrap();
    let created_hsl = packed_rgb.to_hsl();
    let (hue, saturation, lightness) = (
        Angle::new(0f64),
        Normalized::new(0f64).unwrap(),
        Normalized::new(0f64).unwrap(),
    );
    let manual_hsl = HSL::new(hue, saturation, lightness).unwrap();
    assert_eq!(created_hsl, manual_hsl);
}

#[test]
fn test_white_to_hsl() {
    let packed_rgb = PackedRGB::new(16_777_215).unwrap();
    let created_hsl = packed_rgb.to_hsl();
    let (hue, saturation, lightness) = (
        Angle::new(0f64),
        Normalized::new(0f64).unwrap(),
        Normalized::new(1f64).unwrap(),
    );
    let manual_hsl = HSL::new(hue, saturation, lightness).unwrap();
    assert_eq!(created_hsl, manual_hsl);
}

#[test]
fn test_normal_to_hsl() {
    let packed_rgb = PackedRGB::new(2_833_192).unwrap();
    let created_hsl = packed_rgb.to_hsl();
    let (hue, saturation, lightness) = (
        Angle::new(110.52631578947368),
        Normalized::new(0.191919191919192).unwrap(),
        Normalized::new(0.19411764705882353).unwrap(),
    );
    let manual_hsl = HSL::new(hue, saturation, lightness).unwrap();
    assert_eq!(created_hsl, manual_hsl);
}

#[test]
fn test_black_to_hex() {
    let packed_rgb = PackedRGB::new(0).unwrap();
    let created_hex = packed_rgb.to_hex_color();
    let manual_hex = HexColor::new("#000000".to_string()).unwrap();
    assert_eq!(created_hex, manual_hex);
}

#[test]
fn test_white_to_hex() {
    let packed_rgb = PackedRGB::new(16_777_215).unwrap();
    let created_hex = packed_rgb.to_hex_color();
    let manual_hex = HexColor::new("#FFFFFF".to_string()).unwrap();
    assert_eq!(created_hex, manual_hex);
}

#[test]
fn test_normal_to_hex() {
    let packed_rgb = PackedRGB::new(12_341_222).unwrap();
    let created_hex = packed_rgb.to_hex_color();
    let manual_hex = HexColor::new("#BC4FE6".to_string()).unwrap();
    assert_eq!(created_hex, manual_hex);
}
