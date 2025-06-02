use glam::Vec2;
use crate::pher::{Pher,Goal};
use crate::food::Food;
use crate::consts::*;

#[derive(Clone, Copy, Debug)]
enum Caste {
  Queen,
  Soldier,
  Worker,
  Explorer,
  Defender,
}

#[derive(Clone, Debug)]
struct Ant {
  pub caste:Caste, // Caste
  pub goal:Goal, // Its current task, what its doing

  loyalty:u32, // colony its part of

  // Alterable
  hp:f32, // health
  pos:Vec2, // pos
  vel:Vec2, // vel
  mass:f32, // mass

  age:usize, // age

}
/*
caste:
  goal:

  loyalty: 

  // Alterable
  hp: 
  pos:
  vel:
  mass:

  age: 0,

  str: 
  att: 
  def: 
   */

trait Queen {
  fn first_queen(pos:Vec2,loyalty:u32) -> Self;
  fn new_queen(&self) -> Self;
  fn birth(&self, caste:Caste) -> Self;
  fn eat(&mut self, food_piece:Food);
  fn q_emit_pher(&self) -> Pher;
  fn q_heal(&mut self);
}
trait Worker {
  fn new_worker(&self) -> Self;
  fn pick_up_food();
  fn drop_food();
  fn w_emit_pher(&self) -> Pher;
}
trait Explorer {
  fn new_explorer(&self) -> Self;
  fn expand_map();
  fn e_emit_pher(&self) -> Pher;
}
trait Soldier {
  fn new_soldier(&self) -> Self;
  fn s_attack();
  fn s_emit_pher(&self) -> Pher;
}
trait Defender {
  fn new_defender(&self) -> Self;
  fn d_heal();
  fn d_emit_pher(&self) -> Pher;
}


impl Ant {
  fn move_forward(&mut self) {}
  fn turn_left(&mut self) {}
  fn turn_right(&mut self) {}
  
  fn convert_to_food(self) -> Food {
    todo!()
  }
  fn check_should_die(&self) -> bool {
    todo!()
  }
  fn draw(&self) {
    todo!()
  }
  fn emit_pher(&self) -> Pher {
    match self.caste {
      Caste::Defender => self.d_emit_pher(),
      Caste::Queen => self.q_emit_pher(),
      Caste::Explorer => self.e_emit_pher(),
      Caste::Soldier => self.s_emit_pher(),
      Caste::Worker => self.w_emit_pher(),
    }
  }
}

impl Queen for Ant {
  fn first_queen(pos:Vec2,loyalty:u32) -> Self {
    Ant {
      caste:Caste::Queen,
      goal:Goal::ToFood,

      loyalty,

      // Alterable
      hp: Q_MAX_HP,
      pos,
      vel:Vec2::ZERO,
      mass:Q_BASE_MASS,

      age: 0,
    }
  }
  fn new_queen(&self) -> Self {
    Ant {
      caste:Caste::Queen,
      goal:Goal::ToFood,

      loyalty: self.loyalty,

      // Alterable
      hp: Q_MAX_HP,
      pos: self.pos,
      vel:Vec2::ZERO,
      mass:Q_BASE_MASS,

      age: 0,
    }
  }
  fn birth(&self, caste:Caste) -> Self {
    match caste {
      Caste::Defender => self.new_defender(),
      Caste::Explorer => self.new_explorer(),
      Caste::Queen => self.new_queen(),
      Caste::Soldier => self.new_soldier(),
      Caste::Worker => self.new_worker(),
    }
  }
  fn eat(&mut self, food_piece:Food) {
    self.mass += food_piece.mass;
  }
  fn q_emit_pher(&self) -> Pher {
    Pher::new(self.pos, Goal::ToHome)
  }
  fn q_heal(&mut self) {
    let damage = Q_MAX_HP - self.hp;
    let enough_food_left = self.mass >= (Q_MIN_MASS+damage);
    if enough_food_left {
      self.mass -= damage;
      self.hp = Q_MAX_HP;
      return;
    }
    let remaining = self.mass - Q_MIN_MASS;
    self.hp += remaining;
    self.mass = Q_MIN_MASS;
  }
}
impl Worker for Ant {
  fn new_worker(&self) -> Self {todo!();}
  fn drop_food() {
      todo!()
  }
  fn pick_up_food() {
      todo!()
  }
  fn w_emit_pher(&self) -> Pher {
      todo!()
  }
}
impl Explorer for Ant {
  fn new_explorer(&self) -> Self {
      todo!()
  }
  fn e_emit_pher(&self) -> Pher {
      todo!()
  }
  fn expand_map() {
      todo!()
  }
}
impl Soldier for Ant {
  fn new_soldier(&self) -> Self {
      todo!()
  }
  fn s_attack() {
      todo!()
  }
  fn s_emit_pher(&self) -> Pher {
      todo!()
  }
}
impl Defender for Ant {
  fn new_defender(&self) -> Self {
      todo!()
  }
  fn d_emit_pher(&self) -> Pher {
      todo!()
  }
  fn d_heal() {
      todo!()
  }
}
