use std::cmp::min;

use glam::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Food {
  pub pos: Vec2,
  pub mass: f32,
}

impl Food {
  pub fn new(pos:Vec2, mass:f32) -> Self {
    Food {pos,mass}
  }
  pub fn empty_new() -> Self {
    Food { pos: Vec2::ZERO, mass: 0.0 }
  }
  fn combine(&mut self, sacrifice:Self) {
    let split_percent = sacrifice.mass as f32 / (self.mass + sacrifice.mass) as f32;
    let distance = sacrifice.pos - self.pos;
    self.pos = (distance * split_percent) + self.pos;
    self.mass += sacrifice.mass;
  }
  fn to_draw(&self) -> (Vec2, usize) {
    let size:usize = min((self.mass/10.0).round() as usize, 1);
    
    // returns pos and size of food piece
    (self.pos, size)
  }
}