#[derive(Debug, Clone, Copy, Default)]
pub enum Pixel {
    /// Direct color value that will be displayed
    Color(u32),
    /// Index for color that can be chosen later
    Value(usize),
    /// No pixel is drawn
    #[default]
    None,
}

impl Pixel {
    /// returns a color if matched value or has a direct color listed
    pub fn color_by_value(&self, value: usize, color: u32) -> Option<u32> {
        match self {
            Self::Color(u) => Some(*u),
            Self::Value(u) if *u == value => Some(color),
            _ => None,
        }
    }
}
