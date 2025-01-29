use std::collections::HashMap;
use glam::IVec2;

pub struct Pher {
  pos:IVec2,
  id:u16,

  enemy:u8,
  home:u8,
  food:u8,
}

impl Pher {
  fn new(id:u16, pos:IVec2) -> Self {
    Pher {
      pos,
      id,
      enemy:0,
      home:0,
      food:0,
    }
  }
  
  fn place_enemy(&mut self) {
    self.enemy = 100;
  }
  fn place_home(&mut self) {
    self.home = 100;
  }
  fn place_food(&mut self) {
    self.food = 100;
  }

  fn spread(&mut self, adjace:&mut Vec<&mut Pher>) {

    let enemy = self.enemy;
    let home = self.home;
    let food = self.food;

    self.enemy -= 10;
    self.home -= 10;
    self.food -= 10;

    for i in 0..8 {
      adjace[i].enemy += 1;
      adjace[i].home += 1;
      adjace[i].food += 1;
    }
  }
  fn kill(self) -> Self {
    let (enemy,home,food)= (0,0,0);
    if self.enemy >= 10 {
      let enemy=self.enemy;
    }
    if self.home >= 10 {
      let home=self.home;
    }
    if self.food >= 10 {
      let food=self.food;
    }
    Pher {
      pos: self.pos,
      id: self.id,
      enemy,
      home,
      food,
    }
  }
  
}