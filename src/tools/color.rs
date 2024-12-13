#[derive(Debug, Clone, Copy, Default)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl From<u32> for Color {
    fn from(value: u32) -> Self {
        // value as Color
        Color {
            red: (value >> 16) as u8,
            green: (value >> 8) as u8,
            blue: value as u8,
        }
    }
}

impl Color {
    pub fn distance_from(&self, rhs: Self) -> u8 {
        return if self.red > rhs.red {
            self.red - rhs.red
        } else {
            rhs.red - self.red
        } + if self.green > rhs.green {
            self.green - rhs.green
        } else {
            rhs.green - self.green
        } + if self.blue > rhs.blue {
            self.blue - rhs.blue
        } else {
            rhs.blue - self.blue
        };
    }
}

impl From<Color> for u32 {
    fn from(value: Color) -> Self {
        //value as u32
        ((value.red as u32) << 16) | ((value.green as u32) << 8) | (value.blue as u32)
    }
}
