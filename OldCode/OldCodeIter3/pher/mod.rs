use std::collections::HashMap;
use crate::Ant;
use crate::food::Food;

#[derive(Clone)]
#[derive(Copy)]
struct Pher {
  id:u16,
  to_que:f32,
  to_food:f32,
  to_danger:f32,
}
struct Field {
  data: Vec<Vec<HashMap<u16,Pher>>>
}

impl Pher {
  fn new_empty() -> Pher {
    let mut new = Pher {
      id:0,
      to_que:0.,
      to_food:0.,
      to_danger:0.,
    };
    new
  }

  fn fade(&mut self) {
    self.to_que = self.to_que * 0.7;
    self.to_food = self.to_food * 0.7;
    self.to_danger = self.to_danger * 0.7;
    if self.to_que < 0.2 {
      self.disipate_to_que();
    }
    if self.to_danger < 0.2 {
      self.disipate_to_danger();
    }
    if self.to_food < 0.2 {
      self.disipate_to_food();
    }
    if (self.to_food == self.to_danger) & (self.to_que == 0.) & (self.to_danger == self.to_food) {
      // remove it from the field/hashmap entirely so its presence doesnt trigger anything
    }
  }
  
  fn add_to_que(&mut self) {
    self.to_que += 1.0;
  }
  fn add_to_danger(&mut self) {
    self.to_food += 1.0;
  }
  fn add_to_food(&mut self) {
    self.to_food += 1.0;
  }
  fn disipate_to_que(&mut self) {
    self.to_que = 0.;
  }
  fn disipate_to_food(&mut self) {
    self.to_food = 0.;
  }
  fn disipate_to_danger(&mut self) {
    self.to_danger = 0.;
  }
}

impl Field {
  fn begin(x:usize,y:usize) -> Field {
    let mut new:HashMap<u16,Pher> = HashMap::new();
    // new.insert(0, Pher::new_empty());
    let mut field = vec![vec![new;x];y];
    Field {
      data:field,
    }
  }
}

