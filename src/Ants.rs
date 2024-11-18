use macroquad::{prelude::*, rand, math};
#[derive(Clone)]
#[derive(Copy)]


enum Caste {
  Work,
  Look,
  Def,
  Att,
}
struct Ant {
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

impl Ant {
  fn new_worker(pos:IVec2) -> Ant {
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

  fn new_scout(pos:IVec2) -> Ant {
    let out:Ant = Ant {
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

}