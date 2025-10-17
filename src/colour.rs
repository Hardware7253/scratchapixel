#[derive(Clone, Copy)]
pub struct Colour8 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8,
}

impl Colour8 {
    pub fn multiply_float(&self, num: f32) -> Self {
        Colour8 {
            red: (self.red as f32 * num) as u8,
            green: (self.green as f32 * num) as u8,
            blue: (self.blue as f32 * num) as u8,
            alpha: (self.alpha as f32 * num) as u8,
        }
    }
}

impl std::ops::Add for Colour8 {
    type Output = Colour8;

    fn add(self, rhs: Self) -> Self::Output {
        Colour8 {
            red: self.red.saturating_add(rhs.red),
            green: self.green.saturating_add(rhs.green),
            blue: self.blue.saturating_add(rhs.blue),
            alpha: self.alpha.saturating_add(rhs.alpha),
        }
    }
}

// Test colours
pub const BLANK: Colour8 = Colour8 {red: 0, green: 0, blue: 0, alpha: 0};
pub const BLACK: Colour8 = Colour8 {red: 0, green: 0, blue: 0, alpha: 255};
pub const WHITE: Colour8 = Colour8 {red: 255, green: 255, blue: 255, alpha: 255};
pub const RED: Colour8 = Colour8 {red: 255, green: 0, blue: 0, alpha: 255};
pub const GREEN: Colour8 = Colour8 {red: 0, green: 255, blue: 0, alpha: 255};
pub const BLUE: Colour8 = Colour8 {red: 0, green: 0, blue: 255, alpha: 255};