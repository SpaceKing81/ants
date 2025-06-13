
use std::collections::HashMap;
use crate::consts::*;

// The type of pher that gets dropped:
// Ants goal tofood -> follow tofood to food, emit tohome
// Ants goal tohome -> follow tohome to queen, emit tofood
// Ants goal tofight -> follow tofight, emit tofight

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Goal {
  ToHome, // general surround the queen
  ToFood, // general explore out goal
  ToFight, // activly looking for enemy, fight mode
  Queen, // identify's queen
  Food, // identify's food
}

#[derive(Clone,
  // Copy, 
  Debug)]

pub struct PherMap {
  map:HashMap<(usize,usize), HashMap<(Goal,u32),f32>>
}


impl PherMap {
  pub fn new(x:usize,y:usize) -> Self {
    PherMap {
      map:HashMap::new(),
    }
  }
  // tbh, this whole function is probably broken. Its so jank, idek
  pub fn add(&mut self, golo:(Goal, u32), rowcol:(usize,usize)) {
    // check if there is anything at that location
    if let Some(pos) = self.map.get_mut(&rowcol) {
      let pos:&mut HashMap<(Goal,u32),f32> = pos;
      // check if there is any matching goal-loyalty pair at that location
      let pairs:Vec<(Goal,u32)> = pos.keys().copied().collect();
      for i in pairs {
        if i == golo {
          *(pos.get_mut(&golo).unwrap_or(&mut 0.0)) += PHER_FULL_STR;
          return;
        }
      }
      // add new goal-loyalty pair for that location, return
      pos.insert(golo,PHER_FULL_STR);
    } else {
      self.map.insert(
        rowcol,
        HashMap::from(
          [(golo,PHER_FULL_STR)]
        )
      );
    }
  }
  
  pub fn spread(&mut self, pos:(usize, usize), golo:(Goal, u32)) {
    if !self.fade(pos, golo) {
      if self.map.get(&pos).unwrap().len() == 0 {
        self.map.remove(&pos);
      } else {
        self.map.get_mut(&pos).unwrap().remove(&golo);
      }
      return;
    }
    for x in -1..1 {
      for y in -1..1 {
        // get the pos
        let x:i32 = x;
        let y:i32 = y;
        let posx = if x.is_negative() {
          pos.0.saturating_sub(x.abs() as usize)
        } else {
          pos.0.saturating_add(x as usize)
        };
        let posy = if y.is_negative() {
          pos.1.saturating_sub(y.abs() as usize)
        } else {
          pos.1.saturating_add(y as usize)
        };
        self.add(golo, (posx,posy));
      }
    }
  }
  fn fade(&mut self, pos:(usize, usize), golo:(Goal, u32)) -> bool {
    let mut pher:f32 = *self.map.get_mut(&pos).unwrap().get_mut(&golo).unwrap();
    pher *= std::f32::consts::E.powi(-1);
    if &pher <= &SMALLEST_PHER_VALUE {return false;}
    true
  }
}