use ratatui::style::Color;

/// White / Slate / Silver color scheme
pub struct Theme {
    pub bg: Color,
    pub surface: Color,
    pub border: Color,
    pub border_active: Color,
    pub text: Color,
    pub text_dim: Color,
    pub text_muted: Color,
    pub accent: Color,
    pub accent_soft: Color,
    pub highlight_bg: Color,
    pub search_match: Color,
}

pub static THEME: Theme = Theme {
    bg: Color::Rgb(15, 23, 42),           // slate-900
    surface: Color::Rgb(30, 41, 59),      // slate-800
    border: Color::Rgb(71, 85, 105),      // slate-600
    border_active: Color::Rgb(226, 232, 240), // slate-200
    text: Color::Rgb(241, 245, 249),      // slate-100
    text_dim: Color::Rgb(148, 163, 184),  // slate-400
    text_muted: Color::Rgb(100, 116, 139),// slate-500
    accent: Color::Rgb(255, 255, 255),    // white
    accent_soft: Color::Rgb(203, 213, 225), // slate-300
    highlight_bg: Color::Rgb(51, 65, 85), // slate-700
    search_match: Color::Rgb(251, 191, 36), // amber-400
};
