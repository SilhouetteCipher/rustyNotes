use ratatui::prelude::*;

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum ColorScheme {
    Green,      // Classic green terminal
    Blue,       // Blue terminal theme
    Amber,      // Amber/yellow retro theme
    Orange,     // Orange retro theme
    LightGreen, // Lighter green variant
    Red,        // Red terminal theme
    BrightRed,  // Bright/vibrant red theme
}

impl ColorScheme {
    pub fn name(&self) -> &str {
        match self {
            ColorScheme::Green => "Classic Green",
            ColorScheme::Blue => "Terminal Blue",
            ColorScheme::Amber => "Retro Amber",
            ColorScheme::Orange => "Bright Orange",
            ColorScheme::LightGreen => "Light Green",
            ColorScheme::Red => "Alert Red",
            ColorScheme::BrightRed => "Vibrant Red",
        }
    }

    pub fn primary_color(&self) -> Color {
        match self {
            ColorScheme::Green => Color::Green,
            ColorScheme::Blue => Color::Cyan,
            ColorScheme::Amber => Color::Yellow,
            ColorScheme::Orange => Color::Rgb(255, 165, 0), // True orange
            ColorScheme::LightGreen => Color::LightGreen,
            ColorScheme::Red => Color::Red,
            ColorScheme::BrightRed => Color::Rgb(255, 69, 0), // Bright red-orange
        }
    }

    pub fn secondary_color(&self) -> Color {
        match self {
            ColorScheme::Green => Color::LightGreen,
            ColorScheme::Blue => Color::LightBlue,
            ColorScheme::Amber => Color::LightYellow,
            ColorScheme::Orange => Color::Rgb(255, 200, 100), // Light orange
            ColorScheme::LightGreen => Color::Green,
            ColorScheme::Red => Color::LightRed,
            ColorScheme::BrightRed => Color::Rgb(255, 100, 100), // Light bright red
        }
    }

    pub fn all_schemes() -> Vec<ColorScheme> {
        vec![
            ColorScheme::Green,
            ColorScheme::Blue,
            ColorScheme::Amber,
            ColorScheme::Orange,
            ColorScheme::LightGreen,
            ColorScheme::Red,
            ColorScheme::BrightRed,
        ]
    }

    pub fn from_string(s: &str) -> Self {
        match s {
            "Blue" => ColorScheme::Blue,
            "Amber" => ColorScheme::Amber,
            "Orange" => ColorScheme::Orange,
            "LightGreen" => ColorScheme::LightGreen,
            "Red" => ColorScheme::Red,
            "BrightRed" => ColorScheme::BrightRed,
            _ => ColorScheme::Green,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}