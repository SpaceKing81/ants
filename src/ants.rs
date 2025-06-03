
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
  fn move_forward(&mut self);

  fn get_left_turn(&self) -> Vec2 {todo!()}
  fn get_right_turn(&self) -> Vec2 {todo!()}
  fn ant_behind(&self) -> Vec2 {
    let (pos,mut vel) = self.get_pos_vel();
    if vel.round() == Vec2::ZERO {vel = Vec2::new(1.0, 1.0)}
    let size = vel.normalize() * self.radius();
    pos - size // The direction normalized and then grown to the radius, and then placed at the position of the ant
  }
  fn ant_front(&self) -> Vec2 {
    let (pos,mut vel) = self.get_pos_vel();
    if vel.round() == Vec2::ZERO {vel = Vec2::new(1.0, 1.0)}
    let size = vel.normalize() * self.radius();
    pos + size // The direction normalized and then grown to the radius, and then placed at the position of the ant
  }
  
  fn kill(self) -> Food;
  fn check_should_die(&self) -> bool;
  fn draw(&self);
  fn emit_pher(&self) -> Pher;

  fn get_pos_vel(&self) -> (Vec2,Vec2);
  fn radius(&self) -> f32;
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
  fn get_pos_vel(&self) -> (Vec2,Vec2) {
    (self.pos,self.vel)
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
  fn move_forward(&mut self) {
    self.pos += self.vel;
  }
  fn kill(self) -> Food {
    Food::new(self.pos, self.mass + Q_BASE_MASS)
  }
  fn radius(&self) -> f32 {
    (Q_BASE_MASS + self.mass + self.hp)/Q_SIZE_DIVIDER
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
  fn get_pos_vel(&self) -> (Vec2,Vec2) {
    (self.pos,self.vel)
  }

  fn emit_pher(&self) -> Pher {
    match self.goal {
      Goal::ToFood => Pher::new(self.ant_behind(), Goal::ToHome),
      Goal::ToHome => Pher::new(self.ant_behind(), Goal::ToFood),
      Goal::ToFight => Pher::new(self.ant_behind(), Goal::ToFight),
      _ => panic!("Default cannot have anything else!!!")
    }
  }
  fn check_should_die(&self) -> bool {
    self.age > W_MAX_AGE || self.attacked == (true, true)
  }
  fn draw(&self) {
      todo!()
  }
  fn move_forward(&mut self) {
    self.pos += self.vel;
  }
  fn kill(self) -> Food {
    Food::new(self.pos, self.mass + W_BASE_MASS)
  }
  fn radius(&self) -> f32 {
    (W_BASE_MASS + self.mass)/W_SIZE_DIVIDER
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
  fn get_pos_vel(&self) -> (Vec2,Vec2) {
    (self.pos,self.vel)
  }

  fn emit_pher(&self) -> Pher {
    match self.goal {
      Goal::ToFood => Pher::new(self.ant_behind(), Goal::ToHome),
      Goal::ToHome => Pher::new(self.ant_behind(), Goal::ToFood),
      Goal::ToFight => Pher::new(self.ant_behind(), Goal::ToFight),
      _ => panic!("Default cannot have anything else!!!")
    }
  }
  fn check_should_die(&self) -> bool {
    self.age > E_MAX_AGE || self.attacked == (true, true)
  }
  fn draw(&self) {
      todo!()
  }
  fn move_forward(&mut self) {
    self.pos += self.vel;
  }
  fn kill(self) -> Food {
    Food::new(self.pos, E_BASE_MASS)
  }
  fn radius(&self) -> f32 {
    E_BASE_MASS/E_SIZE_DIVIDER
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
  fn get_pos_vel(&self) -> (Vec2,Vec2) {
    (self.pos,self.vel)
  }
  
  fn emit_pher(&self) -> Pher {
    match self.goal {
      Goal::ToFood => Pher::new(self.ant_behind(), Goal::ToHome),
      Goal::ToFight => Pher::new(self.ant_behind(), Goal::ToFight),
      _ => panic!("Fighter cannot have anything else!!!")
    }
  }
  fn check_should_die(&self) -> bool {
    self.age >= S_MAX_AGE || self.hp == 0.0
  }
  fn draw(&self) {
      todo!()
  }
  fn move_forward(&mut self) {
    self.pos += self.vel;
  }
  fn kill(self) -> Food {
    Food::new(self.pos, S_BASE_MASS + self.hp)
  }
  fn radius(&self) -> f32 {
    (S_BASE_MASS + self.hp)/S_SIZE_DIVIDER
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
  fn get_pos_vel(&self) -> (Vec2,Vec2) {
    (self.pos,self.vel)
  }
  
  fn emit_pher(&self) -> Pher {
    match self.goal {
      Goal::ToHome => Pher::new(self.ant_behind(), Goal::ToHome),
      Goal::ToFight => Pher::new(self.ant_behind(), Goal::ToFight),
      _ => panic!("Def cannot have anything else!!!")
    }
  }
  fn check_should_die(&self) -> bool {
    self.hp == 0.0
  }
  fn draw(&self) {
      todo!()
  }
  fn move_forward(&mut self) {
    self.pos += self.vel;
  }
  fn kill(self) -> Food {
    Food::new(self.pos, D_BASE_MASS)
  }
  fn radius(&self) -> f32 {
    (D_BASE_MASS + D_MAX_AGE as f32 + self.hp)/(D_SIZE_DIVIDER + self.age as f32)
  }
}

// Helper functions
fn helper() {}