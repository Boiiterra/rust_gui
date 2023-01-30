use eframe::epaint::Color32;

#[derive(PartialEq, Clone, Copy)]
pub enum CustomTheme {
    Dark,
    Light,
}

impl CustomTheme {
    pub fn bg_color(&self) -> Color32 {
        match self {
            CustomTheme::Dark => Color32::from_rgba_premultiplied(34, 34, 34, 255),
            CustomTheme::Light => Color32::from_rgba_premultiplied(200, 200, 200, 255),
        }
    }

    pub fn fg_color(&self) -> Color32 {
        match self {
            CustomTheme::Dark => Color32::from_rgba_premultiplied(125, 125, 125, 255),
            CustomTheme::Light => Color32::from_rgba_premultiplied(21, 21, 21, 255)
        }
    }
}