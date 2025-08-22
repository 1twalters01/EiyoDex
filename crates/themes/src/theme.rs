use utils::colors::{packed_rgb::PackedRGB, hex_color::HexColor};

pub struct Theme {
    // metadata
    // id: Uuid,
    name: String,
    about: String,

    // Core Colours
    primary: PackedRGB,
    primary_variant: PackedRGB,
    secondary: PackedRGB,
    secondary_variant: PackedRGB,
    tertiary: PackedRGB,
    
    // Feedback Colours
    success: PackedRGB,
    warning: PackedRGB,
    error: PackedRGB,
    info: PackedRGB,
    
    // Text Colours
    text_primary: PackedRGB,
    text_secondary: PackedRGB,
    text_disabled: PackedRGB,
    text_on_primary: PackedRGB,
    text_on_secondary: PackedRGB,
    text_on_tertiary: PackedRGB,
    text_on_success: PackedRGB,
    text_on_warning: PackedRGB,
    text_on_error: PackedRGB,
    text_on_info: PackedRGB,
    link: PackedRGB,
    hover: PackedRGB,
    
    // Other Colours
    background: PackedRGB,
}

impl Theme {
    pub fn default_themes() -> [Theme; 2] {
        [
            Theme {
                name: "Dark Theme 1".to_string(),
                about: "Dark Theme 1 information".to_string(),
                primary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                primary_variant: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                secondary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                secondary_variant: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                tertiary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                success: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                warning: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                error: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                info: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_primary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_secondary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_disabled: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_primary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_secondary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_tertiary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_success: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_warning: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_error: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_info: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                link: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                hover: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                background: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
            },
            Theme {
                name: "Dark Theme 2".to_string(),
                about: "Dark Theme 2 information".to_string(),
                primary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                primary_variant: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                secondary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                secondary_variant: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                tertiary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                success: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                warning: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                error: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                info: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_primary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_secondary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_disabled: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_primary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_secondary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_tertiary: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_success: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_warning: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_error: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                text_on_info: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                link: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                hover: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
                background: HexColor::new("#000000".to_string()).unwrap().to_packed_rgb(),
            },
        ]
    }
}
