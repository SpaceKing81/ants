use macroquad::{math, prelude::*, rand::{self, ChooseRandom}};
use crate::{colony};
#[derive(Clone)]
#[derive(Copy)]


enum Caste {
  Work, //Worker
  Look, //Scout
  Def,  // Defender
  Att,  //Attacker
  Que, //Queen
}
pub struct Ant {
  caste: Caste,
  pos: IVec2,
  vel: IVec2,
  hp: u32,
  age: u128,
  mass: u32,
  speed: u32,
  att_str: u32,
  armor: u32,
  stamina: u64,
}
// New Ants
impl Ant {
  // New ants
  pub fn new_ant(&mut self) -> Ant {
    let num:u8 = rand::gen_range(0, 15);
    let new = match num {
      0..=5 => Ant::new_worker(self),
      6..=9=> Ant::new_scout(self),
      10..=12=> Ant::new_attacker(self),
      13..=14=> Ant::new_defender(self),
      _=> panic!("Something broke with the rand num generator for a new ant"),
      // should be 0-15, tested it so it shouldn't be panicing
    };
    new
  }
  pub fn initial_spawn(lowx:i32,x:i32,lowy:i32,y:i32) -> (Ant,Ant,Ant,Ant,Ant,Ant,Ant) {
    // random position
    // let pos = IVec2::new(rand::gen_range(lowx, x),rand::gen_range(lowy, y));
    let pos = IVec2::new(x, y); //temp
    // starter queen
    let mut start = Ant {
      caste: Caste::Que,
      vel: IVec2::new(0, 0),
      hp: 100,
      age: 0,
      pos,
      mass: 100,
      speed: 1,
      att_str: 0,
      armor: 0,
      stamina: 50,
      };
    // starter workers...probably 
    let mut ant1 = Self::new_ant(&mut start);
    let mut ant2 = Self::new_ant(&mut start);
    let mut ant3 = Self::new_ant(&mut start);
    let mut ant4 = Self::new_ant(&mut start);
    let mut ant5 = Self::new_ant(&mut start);
    let mut ant6 = Self::new_ant(&mut start);
    
    
    (start, ant1, ant2, ant3, ant4,ant5,ant6)
  }

  fn new_worker(&mut self) -> Ant {
    let pos = self.rand_pos();
    let mut out: Ant = Ant {
      caste: Caste::Work,
      pos,
      vel: IVec2::new(0, 0),
      hp: 10,
      age: 0,
      mass: 10,
      speed: 20,
      att_str: 5,
      armor: 7,
      stamina: 200,
    };
    out
  }
  fn new_scout(&mut self) -> Ant {
    let pos = self.rand_pos();
    let mut out:Ant = Ant {
      caste: Caste::Look,
      pos,
      vel: IVec2::new(0, 0),
      hp: 10,
      age: 0,
      mass: 5,
      speed: 30,
      att_str: 2,
      armor: 4,
      stamina: 150,
    };
    out
  }
  fn new_defender(&mut self) -> Ant {
    let pos = self.rand_pos();
    let mut out :Ant = Ant {
      caste: Caste::Def,
      pos,
      vel: IVec2::new(0, 0),
      hp: 20,
      age: 0,
      mass: 20,
      speed: 10,
      att_str: 5,
      armor: 20,
      stamina: 150,
    };
    out
  }
  fn new_attacker(&mut self) -> Ant {
    let pos = self.rand_pos();
    let mut out:Ant = Ant {
      caste: Caste::Att,
      pos,
      vel: IVec2::new(0, 0),
      hp: 15,
      age: 0,
      mass: 15,
      speed: 25,
      att_str: 20,
      armor: 10,
      stamina: 200,
    };
    out
  }
  fn new_queen(&mut self) -> Ant {
    let pos = self.rand_pos();
    let mut new = Ant {
      caste: Caste::Que,
      pos,
      vel: IVec2::new(0, 0),
      hp: 100,
      age: 0,
      mass: 25,
      speed: 1,
      att_str: 0,
      armor: 0,
      stamina: 50,
      };
    new
  }
}
// Draw ants
impl Ant {
  // Draw the ants
  pub fn draw_ant(&self) {
    match self.caste {
      Caste::Work => self.draw_worker(),
      Caste::Look => self.draw_scout(),
      Caste::Def => self.draw_defender(),
      Caste::Att => self.draw_attacker(),
      Caste::Que => self.draw_queen(),
    }
  }

  fn draw_worker(&self) {
    draw_circle(self.pos.x as f32, self.pos.y as f32, (self.mass as f32)/5., DARKBLUE);
  }
  fn draw_scout(&self) {
    draw_circle(self.pos.x as f32, self.pos.y as f32, (self.mass as f32)/4., SKYBLUE);
  }
  fn draw_defender(&self) {
    draw_circle(self.pos.x as f32, self.pos.y as f32, (self.mass as f32)/2., YELLOW);
  }
  fn draw_attacker(&self) {
    draw_circle(self.pos.x as f32, self.pos.y as f32, (self.mass as f32)/2., RED);
  }
  fn draw_queen(&self) {
    draw_circle(self.pos.x as f32, self.pos.y as f32, (self.mass as f32)/10., GOLD);
  }
}
// Quality of life
impl Ant {
  // Quality of life
  fn rand_pos(&self) -> IVec2 {
    let pos = self.pos + IVec2::new(
      rand::gen_range(-20, 20),rand::gen_range(-20, 20)
    );
    pos
  }
}


/*
ThingType::Food=> draw_circle(self.pos.x, self.pos.y, self.mass, ORANGE), 
*/