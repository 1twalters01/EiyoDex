use utils::{
    base_types::{angle::Angle, normalized::Normalized},
    colors::{hex_color::HexColor, hsl::HSL, packed_rgb::PackedRGB, rgb::RGB},
};

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
