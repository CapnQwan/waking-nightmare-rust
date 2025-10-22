#![allow(dead_code)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: u8,
}

impl Color {
  pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
    Self { r, g, b, a }
  }

  fn red() -> Color {
    Self::new(255, 0, 0, 255)
  }

  fn green() -> Color {
    Self::new(0, 255, 0, 255)
  }

  fn blue() -> Color {
    Self::new(0, 0, 255, 255)
  }

  fn yellow() -> Color {
    Self::new(255, 255, 0, 255)
  }

  fn cyan() -> Color {
    Self::new(0, 255, 255, 255)
  }

  fn magenta() -> Color {
    Self::new(255, 0, 255, 255)
  }

  fn white() -> Color {
    Self::new(255, 255, 255, 255)
  }

  fn transparent() -> Color {
    Self::new(0, 0, 0, 0)
  }
}

impl Default for Color {
  fn default() -> Color {
    Self::white()
  }
}

impl ToString for Color {
  fn to_string(&self) -> String {
    format!("({}, {}, {}, {})", self.r, self.g, self.b, self.a)
  }
}
