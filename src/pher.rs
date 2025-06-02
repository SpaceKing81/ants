
use crate::consts::*;
use glam::Vec2;

// The type of pher that gets dropped:
// Ants goal tofood -> follow tofood, emit tohome
// Ants goal tohome -> follow tohome, emit tofood
// Ants goal tofight -> follow tofight, emit tofight

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Goal {
  ToHome,
  ToFood,
  ToFight
}

#[derive(Clone,
  // Copy, 
  Debug)]
pub struct Pher {
  pos: Vec2,
  goal: Goal,
  str:f32,
}

impl Pher {
  fn new(pos:Vec2, str:f32, goal:Goal) -> Self {
    Pher {
      str,
      pos,
      goal,
    }
  }
  fn new_from(&self) -> Self {
    Pher {
      pos:self.pos,
      str:self.str,
      goal:self.goal,
    }
  }
  fn spread(&mut self) -> Option<Vec<Pher>> {
    let mut holder = Vec::new();
    if self.fade() {return None}
    for x in -1..1 {
      for y in -1..1 {
        let mut new = Pher::new_from(&self);
        new.pos.x = new.pos.x.floor() + x as f32;
        new.pos.y = new.pos.y.floor() + y as f32;
        holder.push(new);
      }
    }
    Some(holder)
  }
  fn fade(&mut self) -> bool {
    self.str *= std::f32::consts::E.powi(-1);
    if self.str <= SMALLEST_PHER_VALUE {return false;}
    true
  }
  fn combine(&mut self, sacrifice:Self) {
    if self.goal != sacrifice.goal {panic!("AHHHHHHHH")}
    self.str += sacrifice.str;
  }
}