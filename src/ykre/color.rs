use colored::*;

pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgb {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
    pub fn fg(self, label: &str) -> ColoredString {
        label.truecolor(self.r, self.g, self.b) 
    }
    pub fn bg(self, label: &str) -> ColoredString {
        label.on_truecolor(self.r, self.g, self.b) 
    }
}
