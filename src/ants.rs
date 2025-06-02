
use glam::Vec2;
use crate::pher::{Pher,Goal};
use crate::food::Food;
use crate::consts::*;

#[derive(Clone, Debug)]
struct Queen {
  loyalty:u32,
  hp:f32, // health
  pos:Vec2, // pos
  vel:Vec2, // vel
  mass:f32, // food stored
  age:usize, // age
}
#[derive(Clone, Debug)]
struct Worker {
  loyalty:u32,
  goal:Goal,
  attacked:(bool,bool), // health value irrelevent, 2 attacks by worker/single by solider kills
  pos:Vec2, // pos
  vel:Vec2, // vel
  mass:f32, // food held
  age:usize, // age
}
#[derive(Clone, Debug)]
struct Explorer {
  loyalty:u32,
  goal:Goal,
  attacked:(bool,bool), // health value irrelevent, 2 attacks by worker/single by solider kills
  pos:Vec2, // pos
  vel:Vec2, // vel
  age:usize, // age
}
#[derive(Clone, Debug)]
struct Soldier {
  loyalty:u32,
  goal:Goal,
  hp:f32, // health
  pos:Vec2, // pos
  vel:Vec2, // vel
  age:usize, // age
}
#[derive(Clone, Debug)]
struct Defender {
  loyalty:u32,
  goal:Goal,
  hp:f32, // health
  pos:Vec2, // pos
  vel:Vec2, // vel
  age:usize, // age
}

trait Ant {
  fn new(queen:&Queen) -> Self;
  fn move_forward(&mut self) {todo!()}
  fn turn_left(&mut self) {todo!()}
  fn turn_right(&mut self) {todo!()}

  fn ant_behind(&self) -> Vec2 {todo!()}
  fn ant_front(&self) -> Vec2 {todo!()}
  
  fn kill(self) -> Food;
  fn check_should_die(&self) -> bool;
  fn draw(&self);
  fn emit_pher(&self) -> Pher;
}

impl Queen {
  fn first_queen(pos:Vec2,loyalty:u32) -> Self {
    Self {
      loyalty,

      // Alterable
      hp: Q_MAX_HP,
      pos,
      vel:Vec2::ZERO,
      mass:Q_BASE_MASS,

      age: 0,
    }
  }
  fn birth<A:Ant>(&self) -> A {
    todo!()
  }
  fn eat(&mut self, food_piece:Food) {
    self.mass += food_piece.mass;
  }
  
  fn heal(&mut self) {
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
impl Ant for Queen {
  fn new(queen:&Queen) -> Self {
    Queen {
      loyalty: queen.loyalty,

      // Alterable
      hp: Q_MAX_HP,
      pos: queen.ant_behind(),
      vel:Vec2::ZERO,
      mass:Q_BASE_MASS,

      age: 0,
    }
  }
  fn emit_pher(&self) -> Pher {
    Pher::new(self.ant_behind(), Goal::Queen)
  }
  fn check_should_die(&self) -> bool {
    self.age > Q_MAX_AGE || self.hp <= 0.0
  }
  fn draw(&self) {
      todo!()
  }
  fn kill(self) -> Food {
      Food::new(self.pos, self.mass + Q_BASE_MASS)
  }
}
impl Worker {
  fn drop_food(&mut self) -> Food {
    self.mass -= W_STR as f32;
    Food::new(self.ant_front(), W_STR as f32)
  }
  fn pick_up_food(&mut self, food:&mut Food) {
    let tbcarried = (W_STR as f32).min(food.mass);
    food.mass -= tbcarried;
    self.mass += tbcarried;
  }
}
impl Ant for Worker { 
  fn new(queen:&Queen) -> Self {
    Worker {
      loyalty: queen.loyalty,
      goal:Goal::ToFood,

      // Alterable
      attacked: (false,false),
      pos: queen.ant_behind(),
      vel:Vec2::ZERO,
      mass:Q_BASE_MASS,

      age: 0,
    }
  }
  fn emit_pher(&self) -> Pher {
    todo!()
  }
  fn check_should_die(&self) -> bool {
      todo!()
  }
  fn draw(&self) {
      todo!()
  }
  fn kill(self) -> Food {
      todo!()
  }

}
impl Explorer {
  fn expand_map() {
      todo!()
  }
}
impl Ant for Explorer {
  fn new(queen:&Queen) -> Self {
    Self {
      goal:Goal::ToFood,
      loyalty: queen.loyalty,

      // Alterable
      attacked:(false,false),
      pos: queen.ant_behind(),
      vel:Vec2::ZERO,

      age: 0,
    }
  }
  fn emit_pher(&self) -> Pher {
    match self.goal {
      Goal::ToFood => Pher::new(self.ant_behind(), Goal::ToHome),
      Goal::ToHome => Pher::new(self.ant_behind(), Goal::ToFood),
      Goal::ToFight => Pher::new(self.ant_behind(), Goal::ToFight),
      _ => panic!("Explorer cannot have anything else!!!")
    }
  }
  fn check_should_die(&self) -> bool {
      todo!()
  }
  fn draw(&self) {
      todo!()
  }
  fn kill(self) -> Food {
      todo!()
  }

}
impl Soldier {
  fn attack() {
      todo!()
  }
}
impl Ant for Soldier {
  fn new(queen:&Queen) -> Self {
    Self {
      goal: Goal::ToFood,
      loyalty: queen.loyalty,

      // Alterable
      hp: S_MAX_HP,
      pos: queen.ant_behind(),
      vel: Vec2::ZERO,

      age: 0,
    }
  }
  fn emit_pher(&self) -> Pher {
    todo!()
  }
  fn check_should_die(&self) -> bool {
      todo!()
  }
  fn draw(&self) {
      todo!()
  }
  fn kill(self) -> Food {
      todo!()
  }
}
impl Defender {
  fn heal() {
      todo!()
  }
}
impl Ant for Defender { 
  fn new(queen:&Queen) -> Self {
    Self {
      goal:Goal::ToHome,
      loyalty: queen.loyalty,

      // Alterable
      hp: D_MAX_HP,
      pos: queen.ant_behind(),
      vel:Vec2::ZERO,

      age: 0,
    }
  }
  fn emit_pher(&self) -> Pher {
    todo!()
  }
  fn check_should_die(&self) -> bool {
      todo!()
  }
  fn draw(&self) {
      todo!()
  }
  fn kill(self) -> Food {
      todo!()
  }
}