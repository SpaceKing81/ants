const TURN_DEGREE:f32= 18.0; 

use macroquad::{math, prelude::*, rand::{self, ChooseRandom}};
// use crate::{colony};
use crate::food::{Food};

// first the big number, then the number you wish base with
// ie: a modulo b, 7 mod 2 = 1
fn modulo<T>(a: T, b: T) -> T where T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy, {((a % b) + b) % b} // calculate modulus operations


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
  pub pos: IVec2,
  vel: IVec2,
  hp: u16,
  age: u64,
  pub mass: u32,
  speed: u8,
  att_str: u8,
  armor: u8,
  pub str: u8,

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
      str: 50,
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
      str: 200,
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
      str: 150,
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
      str: 150,
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
      str: 200,
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
      str: 50,
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
  pub fn death(self) -> Food {
    //converts an ant into food
    let (mass, pos) = (self.mass, self.pos);
    let mut death_tax = Food {
      mass,
      pos,
    };
    death_tax
  }


}
//Movement
impl Ant {
  // inner directions alter by TURN_DEGREE
  // Outer directions alter by twice TURN_DEGREE

  fn move_forward(&mut self) {
    self.pos += self.vel;
  }
  fn turn_right(&mut self) {
    self.turn(-TURN_DEGREE);
  }
  fn turn_left(&mut self) {
    self.turn(TURN_DEGREE);
  }
  fn turn_far_left(&mut self) {
    self.turn(2.0 * TURN_DEGREE);
  }
  fn turn_far_right(&mut self) {
    self.turn(-2.0*TURN_DEGREE);
  }
  
  //Turns an ant by a given angle
  fn turn(&mut self, angle:f32) {
    // Determine inner product
    let inn_prod_init = self.vel.x*self.vel.x + self.vel.y*self.vel.y;

    // Get the current angle
    let theta = Self::degree_from_horiz(self.vel.x, self.vel.y);
    // Get the desired angle
    let new_theta = modulo(theta + angle, 360.);
    
    // Gets the unit velocity
    let (x,y) = (new_theta.cos() as i32, new_theta.sin() as i32);
    let unit_vel = IVec2::new(x,y);
    let inn_prod_unit = unit_vel.x*unit_vel.x + unit_vel.y*unit_vel.y;
    let alpha = inn_prod_init/inn_prod_unit;

    //Alter the velocity to the new direction, and maintain the length
    self.vel = alpha * unit_vel;


  
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
  // Returns standerd degree of oriantation with angles obeying:
  // cos(0) = vel (x,0), cos(90) = vel (0,y), cos(180) = vel (-x,0), cos(270) = vel (0,-y)
  fn degree_from_horiz(x:i32,y:i32) -> f32 {
    let x: f32 = x as f32;
    let y: f32 = y as f32;
    let x1 = &x.abs() == &x;
    let y1 = &y.abs() == &y;
    if x == 0. {            
        let y = y.abs() == y;
        match y {
          true => return 90.,
          false => return 270.,
        }
    }
    if y == 0. {
      let x = x.abs() == x;
      match x {
        true => return 0.,
        false => return 180.,
      }
    }
    let degree = f32::to_degrees(f32::atan(y/x));
    match y1 {
        true => {match x1 {
            true => return degree,
            false => return 180. - degree,
        }},
        false => {match x1 {
            true => return 360. - degree,
            false => return degree + 180.,
        }}
    }
  }
  //returns the distence values between two {ants}
  fn trig_calculator(&self, i: Ant) -> (IVec2,f32,f32) {
    let x = self.pos.x - i.pos.x;
    let y = self.pos.y - i.pos.y;
    let r = ((x*x) as f32 + (y*y) as f32).sqrt();
    let theta = Self::degree_from_horiz(x, y);
    let xy = IVec2::new(x,y);
    (xy,r,theta)
}

}


/*
ThingType::Food=> draw_circle(self.pos.x, self.pos.y, self.mass, ORANGE), 
*/