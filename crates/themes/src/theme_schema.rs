pub struct ThemeFieldInfo {
    pub key: &'static str,       // e.g. "primary"
    pub description: &'static str, // e.g. "Primary brand color, used for main UI elements"
}

pub struct ThemeSchema {
    primary: ThemeFieldInfo,
    primary_variant: ThemeFieldInfo,
    secondary: ThemeFieldInfo,
    secondary_variant: ThemeFieldInfo,
    tertiary: ThemeFieldInfo,
    
    // Feedback Colours
    success: ThemeFieldInfo,
    warning: ThemeFieldInfo,
    error: ThemeFieldInfo,
    info: ThemeFieldInfo,
    
    // Text Colours
    text_primary: ThemeFieldInfo,
    text_secondary: ThemeFieldInfo,
    text_disabled: ThemeFieldInfo,
    text_on_primary: ThemeFieldInfo,
    text_on_secondary: ThemeFieldInfo,
    text_on_tertiary: ThemeFieldInfo,
    text_on_success: ThemeFieldInfo,
    text_on_warning: ThemeFieldInfo,
    text_on_error: ThemeFieldInfo,
    text_on_info: ThemeFieldInfo,
    link: ThemeFieldInfo,
    hover: ThemeFieldInfo,
    
    // Other Colours
    background: ThemeFieldInfo,
}

impl ThemeSchema {
    pub fn default() -> Self {
        Self {
            primary: ThemeFieldInfo { key: "primary", description: "Used for main UI elements",
            },
            primary_variant: ThemeFieldInfo { key: "primary_variant", description: "Alternate version of primary",
            },
            secondary: ThemeFieldInfo { key: "secondary", description: "Supporting accent color",
            },
            secondary_variant: ThemeFieldInfo { key: "secondary_variant", description: "Alternate version of secondary",
            },
            tertiary: ThemeFieldInfo { key: "tertiary", description: "Accent color for highlightsx or special UI",
            },
            success: ThemeFieldInfo { key: "success", description: "For confirmations or completed actions",
            },
            warning: ThemeFieldInfo { key: "warning", description: "For alerts or caution states",
            },
            error: ThemeFieldInfo { key: "error", description: "For validation errors or destructive actions",
            },
            info: ThemeFieldInfo { key: "info", description: "For hints, neutral alerts and guidance",
            },
            text_primary: ThemeFieldInfo { key: "text_primary", description: "Main text",
            },
            text_secondary: ThemeFieldInfo { key: "text_secondary", description: "De-emphasized text, subtitles and hints",
            },
            text_disabled: ThemeFieldInfo { key: "text_disabled", description: "Disabled/unavailable text",
            },
            text_on_primary: ThemeFieldInfo { key: "text_on_primary", description: "Text shown over primary surfaces",
            },
            text_on_secondary: ThemeFieldInfo { key: "text_on_secondary", description: "Text shown over secondary surfaces",
            },
            text_on_tertiary: ThemeFieldInfo { key: "text_on_tertiary", description: "Text shown over tertiary surfaces",
            },
            text_on_success: ThemeFieldInfo { key: "text_on_success", description: "Text shown over success surfaces",
            },
            text_on_warning: ThemeFieldInfo { key: "text_on_warning", description: "Text shown over warning surfaces",
            },
            text_on_error: ThemeFieldInfo { key: "text_on_error", description: "Text shown over error surfaces",
            },
            text_on_info: ThemeFieldInfo { key: "text_on_info", description: "Text shown over info surfaces",
            },
            link: ThemeFieldInfo { key: "link", description: "Hyperlink or interactive text color",
            },
            hover: ThemeFieldInfo { key: "hover", description: "Hover color",
            },
            background: ThemeFieldInfo { key: "background", description: "Main app background",
            },
        }
    }
}

