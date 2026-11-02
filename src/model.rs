#[derive(Debug, Clone, Copy)]
pub struct IconDef {
    pub glyph: &'static str,
    pub unicode_fallback: &'static str,
    pub color_code: &'static str, // ANSI escape sequences (e.g., "\x1b[38;5;208m")
}

impl IconDef {
    pub const fn new(glyph: &'static str, unicode_fallback: &'static str, color_code: &'static str) -> Self {
        Self {glyph, unicode_fallback, color_code}
    }

    //Renders the glyph with its ANSI color code followed by a color reset
    #[inline(always)]
    pub fn render(&self, use_unicode: bool) -> String {
        let sym = if use_unicode { self.unicode_fallback } else { self.glyph };
        format!("{}{}\x1b[0m", self.color_code, sym)
    }
}