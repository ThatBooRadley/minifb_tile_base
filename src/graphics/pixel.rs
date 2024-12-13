use crate::tools::color::Color;

#[derive(Debug, Clone, Copy, Default)]
pub enum Pixel {
    /// Direct color value that will be displayed
    Color(Color),
    /// No pixel is drawn
    #[default]
    None,
}
