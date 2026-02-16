use utils::{
    base_types::{angle::Angle, normalized::Normalized},
    colors::{hex_color::HexColor, hsl::HSL, packed_rgb::PackedRGB, rgb::RGB},
};

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
