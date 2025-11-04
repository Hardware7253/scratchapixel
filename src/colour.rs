#[derive(Clone, Copy)]
pub struct Colour {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32,
}

impl Colour {
    pub fn new() -> Self {
        BLANK
    }

    pub fn multiply_float(&self, num: f32) -> Self {
        Colour {
            red: self.red * num,
            green: self.green * num,
            blue: self.blue * num,
            alpha: self.alpha* num,
        }
    }

    pub fn to_bytes(&self) -> [u8; 4] {
        [
            normalised_to_byte(self.red),
            normalised_to_byte(self.green),
            normalised_to_byte(self.blue),
            normalised_to_byte(self.alpha),
        ]
    }
}

// Converts default colour normalised [0, 1] channel to byte channel [0, 255]
pub fn normalised_to_byte(normalised_colour_chanel: f32) -> u8 {
    (normalised_colour_chanel * 255.0).clamp(0.0, 255.0) as u8
}

// Converts byte colour channel [0, 255] to normalised channel [0, 1]
pub fn byte_to_normalised(colour_channel_byte: u8) -> f32 {
   colour_channel_byte as f32 / 255.0 
}

impl std::ops::Add for Colour {
    type Output = Colour;

    fn add(self, rhs: Self) -> Self::Output {
        Colour {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
            alpha: self.alpha + rhs.alpha,
        }
    }
}

// Test colours
pub const BLANK: Colour = Colour {red: 0.0, green: 0.0, blue: 0.0, alpha: 0.0};
pub const BLACK: Colour = Colour {red: 0.0, green: 0.0, blue: 0.0, alpha: 1.0};
pub const WHITE: Colour = Colour {red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0};
pub const RED: Colour = Colour {red: 1.0, green: 0.0, blue: 0.0, alpha: 1.0};
pub const GREEN: Colour = Colour {red: 0.0, green: 1.0, blue: 0.0, alpha: 1.0};
pub const BLUE: Colour = Colour {red: 0.0, green: 0.0, blue: 1.0, alpha: 1.0};