use::macroquad::prelude::*;


pub struct Food {
  mass: u32,
  pos: IVec2,
}

impl Food {
  fn new(mass:u32, pos:IVec2) -> Food {
    let new = Food {
      mass,
      pos,
    };
    new
  }
}

