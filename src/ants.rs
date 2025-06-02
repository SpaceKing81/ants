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
  // mass:f32, // mass

  age:usize, // age
}
#[derive(Clone, Debug)]
struct Soldier {
  loyalty:u32,
  goal:Goal,
  hp:f32, // health
  pos:Vec2, // pos
  vel:Vec2, // vel
  // mass:f32, // mass

  age:usize, // age
}
#[derive(Clone, Debug)]
struct Defender {
  loyalty:u32,
  goal:Goal,
  hp:f32, // health
  pos:Vec2, // pos
  vel:Vec2, // vel
  // mass:f32, // mass

  age:usize, // age
}



/*
Ant {
  caste:
  goal:

  loyalty: 

  // Alterable
  hp: 
  pos:
  vel:
  mass:

  age: 0,
}
   */
/*
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
  fn drop_food(&mut self) -> Food;
  fn pick_up_food(&mut self, food:&mut Food);
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
 */

trait Ant {
  fn move_forward(&mut self);
  fn turn_left(&mut self);
  fn turn_right(&mut self);

  fn ant_behind(&self) -> Vec2;
  fn ant_front(&self) -> Vec2;
  
  fn kill(self) -> Food;
  fn check_should_die(&self) -> bool;
  fn draw(&self);
  fn emit_pher(&self) -> Pher;
}

impl Queen {
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
      pos: self.ant_behind(),
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
    Pher::new(self.ant_behind(), Goal::ToHome)
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
impl Worker {
  fn new_worker(&self) -> Self {
    Ant {
      caste:Caste::Worker,
      goal:Goal::ToFood,

      loyalty: self.loyalty,

      // Alterable
      hp: W_MAX_HP,
      pos:self.ant_behind(),
      vel:Vec2::ZERO,
      mass:W_BASE_MASS,

      age: 0,
    }
  }
  fn drop_food(&mut self) -> Food {
    self.mass -= W_STR as f32;
    Food::new(self.ant_front(), W_STR as f32)
  }
  fn pick_up_food(&mut self, food:&mut Food) {
    let tbcarried = (W_STR as f32).min(food.mass);
    food.mass -= tbcarried;
    self.mass += tbcarried;
  }
  fn w_emit_pher(&self) -> Pher {
    match self.goal {
      Goal::ToFood => Pher::new(self.ant_behind(), Goal::ToHome),
      Goal::ToHome => Pher::new(self.ant_behind(), Goal::ToFood),
      Goal::ToFight => Pher::new(self.ant_behind(), Goal::ToFight),
    }
  }
}
impl Explorer {
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
impl Soldier {
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
impl Defender {
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
