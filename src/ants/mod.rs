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
  age: u64,
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
    let num:u8 = rand::gen_range(0, 150);
    let new = match num {
      0..=50 => Self::new_worker(self),
      60..=90=> Self::new_scout(self),
      100..=120=> Self::new_attacker(self),
      130..=140=> Self::new_defender(self),
      _=> Self::new_queen(self),
      // should be 0-150, tested it so it shouldn't be panicing
    };
    new
  }
  pub fn initial_spawn(x:i32,y:i32) -> (Ant,Ant,Ant,Ant,Ant,Ant,Ant) {
    // random position
    let pos = IVec2::new(x, y);
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
// Age
impl Ant { 
  pub fn check_old(&self) -> bool {
    match self.caste {
      Caste::Que => return self.check_old_queen(),
      Caste::Work => return self.check_old_worker(),
      Caste::Look => return self.check_old_scout(),
      Caste::Att => return self.check_old_attacker(),
      Caste::Def => return self.check_old_defender(),
      _=> panic!("bruh. You did something to the caste system, and now the age 
      stuff is causing errors cuz you forgot to update the match"),
    }
  }
  
  fn check_old_worker(&self) -> bool {
    if self.age > 50000{
      return true
    }
    false
  }
  fn check_old_scout(&self) -> bool {
    if self.age > 600000{
      return true
    }
    false
  }
  fn check_old_attacker(&self) -> bool {
    if self.age > 80000{
      return true
    }
    false
  }
  fn check_old_defender(&self) -> bool {
    if self.age > 100000{
      return true
    }
    false
  }
  fn check_old_queen(&self) -> bool{
    if self.age > 1000000 {
      return true
    }
    false
  }
  fn ageing(&mut self) {
    self.age += 1;
  }
}
//Death and taxes
impl Ant {
  // pub fn death() -> Food {

  }
// }


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