use macroquad::{math, prelude::*, rand::{self, ChooseRandom}};
use std::cmp::min;
use crate::ants::Ant;

pub struct Food {
  pub mass:u32,
  pub pos:IVec2,
}
//Make new
impl Food {
  fn new(mass:u32, pos:IVec2) -> Food {
    let new = Food {
      mass,
      pos,
    };
    new
  }
}

//Eating
impl Food {
  pub fn consume_food(&mut self, ant:&mut Ant) {
    self.mass -= 1;
    ant.mass += 1;
  }
}
//Grabbing 
impl Food {
  //reduces the big food chunk
  pub fn pick_up_food(&mut self, mut ant: Ant) -> (Ant, Food) {
    let delta = min(ant.str as u32, self.mass);
    self.mass -= delta;
    ant.str -= delta as u8;
    
    let mut new_food = Self::new(delta, ant.pos.clone());
    ant.mass += new_food.mass;
    (ant, new_food)

  }
  
  // pub fn move_food(ant, food) {
  //   ant.
  // }

}