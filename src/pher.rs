
use std::collections::HashMap;

use crate::consts::*;
use glam::Vec2;

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
  // fn new_from(&self) -> Self {
  //   Pher {
  //     str:self.str,
  //   }
  // }
  
  // fn fade(&mut self) -> bool {
  //   self.str *= std::f32::consts::E.powi(-1);
  //   if self.str <= SMALLEST_PHER_VALUE {return false;}
  //   true
  // }
  // fn combine(&mut self, sacrifice:Self) {
  //   todo!()
  // }


impl PherMap {
  fn new(loyalty: u32,x:usize,y:usize) -> Self {
    PherMap {
      map:HashMap::new(),
    }
  }
  pub fn add(&mut self, goal:Goal, loyalty:u32, rowcol:(usize,usize)) {
    let at_pos:Option<&mut HashMap<(Goal, u32),f32>> = self.map.get_mut(&rowcol);
    if let Some(pos) = at_pos {
      let keys:Vec<&(Goal,u32)> = pos.keys().clone().collect();
      for i in keys {
        // Checks to see if there is a pher of this loyalty and goal at this location
        if i == &(goal,loyalty) {
          if let Some(e)= pos.get_mut(i) {
            *e += PHER_FULL_STR;
          } else { panic!("Dude. Just cry")}
          return;
        }
      }
      let key = (goal,loyalty);
      pos.insert(key,PHER_FULL_STR);
      
    } else {
      let inkey = (goal,loyalty);
      let outkey = rowcol;
      let entry:HashMap<(Goal,u32), f32> = HashMap::from([(inkey,PHER_FULL_STR)]);
      self.map.insert(outkey,entry);
    }
  }
  // pub fn spread(&mut self) -> Option<Vec<Pher>> {
  //   let mut holder = Vec::new();
  //   if self.fade() {return None}
  //   for x in -1..1 {
  //     for y in -1..1 {
  //       let mut new = Pher::new_from(&self);
  //       new.pos.x = new.pos.x.floor() + x as f32;
  //       new.pos.y = new.pos.y.floor() + y as f32;
  //       holder.push(new);
  //     }
  //   }
  //   Some(holder)
  // }
}