use std::{collections::HashMap};
use macroquad::{prelude::*, rand};
use miniquad::gl::user_addr_t;
use crate::matrix::Matrix;


// first the big number, then the number you wish base with
// ie: a modulo b, 7 mod 2 = 1
fn modulo<T>(a: T, b: T) -> T where T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy, {((a % b) + b) % b} // calculate modulus operations

// Constant vars, levers to be dialed carefully
const TURN_DEGREE:f32 = 18.0;

const WORK_DETECT:u32 = 5;
const LOOK_DETECT:u32 = 20;
const ATT_DETECT:u32 = 10;
const DEF_DETECT:u32 = 5;
const QUE_DETECT:u32 = 1;



#[derive(Clone)]
enum Caste {
  Work, //Worker
  Look, //Scout
  Def,  // Defender
  Att,  //Attacker
  Que, //Queen
}
#[derive(Clone)]
enum Task {
  ToFood,
  ToHome,
  ToFight,
}

#[derive(Clone)]
pub struct Ant {
  caste: Caste,
  pub id: u16,
  pub pos: IVec2,
  vel: IVec2,
  hp: u16,
  age: u64,
  pub mass: u32,
  speed: u8,
  att_str: u8,
  armor: u8,
  pub str: u8,
  task: Task,
  holding: Option<u32>,
}

// New ants
impl Ant {
  pub fn birth_ant(&mut self) -> Ant {
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
  pub fn initial_spawn(x:i32,y:i32, id:u16) -> (Ant,Ant,Ant,Ant,Ant,Ant,Ant) {
    // random position
    let pos = IVec2::new(x, y);
    // starter queen
    let mut queen = Ant {
      caste: Caste::Que,
      id,
      vel: IVec2::new(0, 0),
      hp: 100,
      age: 0,
      pos,
      mass: 100,
      speed: 1,
      att_str: 0,
      armor: 0,
      str: 50,
      task: Task::ToFood,
      holding: None,
      };
    // starter workers...probably 
    let mut ant1 = Self::birth_ant(&mut queen);
    let mut ant2 = Self::birth_ant(&mut queen);
    let mut ant3 = Self::birth_ant(&mut queen);
    let mut ant4 = Self::birth_ant(&mut queen);
    let mut ant5 = Self::birth_ant(&mut queen);
    let mut ant6 = Self::birth_ant(&mut queen);
    
    
    (queen, ant1, ant2, ant3, ant4,ant5,ant6)
  }

  fn new_worker(&mut self) -> Ant {
    let pos = self.rand_pos();
    let id = self.id;
    Ant {
      caste: Caste::Work,
      pos,
      id,
      vel: IVec2::new(0, 0),
      hp: 10,
      age: 0,
      mass: 10,
      speed: 20,
      att_str: 5,
      armor: 7,
      str: 200,
      task: Task::ToFood,
      holding: None,
    }
  }
  fn new_scout(&mut self) -> Ant {
    let pos = self.rand_pos();
    let id = self.id;
    Ant {
      caste: Caste::Look,
      pos,
      id,
      vel: IVec2::new(0, 0),
      hp: 10,
      age: 0,
      mass: 5,
      speed: 30,
      att_str: 2,
      armor: 4,
      str: 150,
      task: Task::ToFood,
      holding: None,
    }
  }
  fn new_defender(&mut self) -> Ant {
    let pos = self.rand_pos();
    let id = self.id;
    Ant {
      caste: Caste::Def,
      pos,
      id,
      vel: IVec2::new(0, 0),
      hp: 20,
      age: 0,
      mass: 20,
      speed: 10,
      att_str: 5,
      armor: 20,
      str: 150,
      task: Task::ToFood,
      holding: None,
    }
  }
  fn new_attacker(&mut self) -> Ant {
    let pos = self.rand_pos();
    let id = self.id;
    Ant {
      caste: Caste::Att,
      pos,
      id,
      vel: IVec2::new(0, 0),
      hp: 15,
      age: 0,
      mass: 15,
      speed: 25,
      att_str: 20,
      armor: 10,
      str: 200,
      task: Task::ToFood,
      holding: None,
    }
  }
  fn new_queen(&mut self) -> Ant {
    let pos = self.rand_pos();
    let id = self.id;
    Ant {
      caste: Caste::Que,
      pos,
      id,
      vel: IVec2::new(0, 0),
      hp: 100,
      age: 0,
      mass: 25,
      speed: 1,
      att_str: 0,
      armor: 0,
      str: 50,
      task: Task::ToFood,
      holding: None,
      }
  }
}

// Derived Values
impl Ant {
  // Get Functions
  pub fn get_size(&self) -> u32 {
    match self.caste {
      Caste::Que => self.mass/10,
      Caste::Work => self.mass/4,
      _=> self.mass/3,
    }
  } 
  
  pub fn get_detection_range(&self) -> usize {
    match self.caste {
      Caste::Work => self.detect_range(WORK_DETECT),
      Caste::Que => self.detect_range(QUE_DETECT),
      Caste::Att => self.detect_range(ATT_DETECT),
      Caste::Def => self.detect_range(DEF_DETECT),
      Caste::Look => self.detect_range(LOOK_DETECT),
    } 
  }
  fn detect_range(&self, x:u32) -> usize {
    (self.get_size() + x ) as usize

  }
  
}

// Death and Taxes
impl Ant {
  pub fn death(self) { //tbd
    //converts an ant into food
    let (mass, pos) = (self.mass, self.pos);
    //food thing not yet made
  }


}

// Eternal March of Time
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


//Fog of War
impl Ant {
    pub fn new_fogowar() -> HashMap<(usize,usize),bool>{
      let new:HashMap<(usize,usize),bool> = HashMap::new();
      new
    }
    // checks if the given coordinate is in the fog of war(false) or not(true)
    fn is_not_fog_war(cord:(usize, usize), fogowar:&mut HashMap<(usize,usize),bool>) -> bool {
      let test = *fogowar.entry(cord).or_insert(false);
      test

      // - Access values with "let value = map.entry(key).or_insert(false);". This
      // allows the accessing of unknown, and not present values, as well as existing
      // ones, and return false to the unknown ones.


    }
    // Takes a matrix value from the fog of war and makes it known
    fn discover_from_fog(cord:(usize, usize), fogowar:&mut HashMap<(usize,usize),bool>) {
      if Self::is_not_fog_war(cord, fogowar) {return}
      fogowar.entry(cord).and_modify(|v| *v = true);
      
      // - This statment will modify the map at the key value, and
      // change it to true. Idk why its structured like that, but 
      // it needs to be, can't direct edit it due to reference rules
    }
    // Or unknown
    fn lost_to_fog(cord:(usize, usize), fogowar:&mut HashMap<(usize,usize),bool>) {
      if Self::is_not_fog_war(cord, fogowar) {
        fogowar.entry(cord).and_modify(|v| *v = false);
      }
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
