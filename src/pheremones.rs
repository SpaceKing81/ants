use std::collections::HashMap;
use glam::IVec2;
use crate::ants::Ant;

enum PType {
  Enemy,
  Home,
  Food,
}


#[derive(Clone)]
pub struct Pher {
  colony_num: u16,
  enemy:HashMap<u16,u8>,
  home:HashMap<u16,u8>,
  food:HashMap<u16,u8>,
}

impl Pher {
  pub fn new() -> Self {
    let enemy = HashMap::new();
    let home = HashMap::new();
    let food = HashMap::new();

    Pher {
      colony_num:0,
      enemy,
      home,
      food,
    }
  }

// need to make it so that it checks if the pher value exists, and if it doesent, add it
  fn place_pher(&mut self, id:u16, add:PType) {
    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    match add {
      PType::Enemy => a = 100,
      PType::Home => b = 100,
      PType::Food => c = 100,
    }
    if !self.enemy.contains_key(&id) {
      self.enemy.insert(id, a);
      self.home.insert(id, b);
      self.food.insert(id, c);
      self.colony_num +=1;
      return
    }
    self.enemy.entry(id).and_modify(|v| *v = a);
    self.home.entry(id).and_modify(|v| *v = b);
    self.food.entry(id).and_modify(|v| *v = c);

  }

  // takes a pher and the 8 surounding phers and spreads the love
  fn spread(&mut self, adjace:&mut Vec<&mut Pher>) {
    let mut en_keys = Vec::new();
    let mut hm_keys = Vec::new();
    let mut fd_keys = Vec::new();
    // collects all the hashmap keys that need updating, and updates all the surounding
    // ones that are more then 8
    for (&id,&intensity) in &self.enemy {
      // If the strength is too low, just removes the thing entierly and skips it
      if intensity <= 16 {continue;}
      for i in 0..8 {
        if adjace[i].enemy.get(&id) <= Some(&8) {continue;}
        adjace[i].enemy.entry(id).and_modify(|v| *v += 1);
        en_keys.push(id);
      }
    }
    for (&id,&intensity) in &self.home {
      // If the strength is too low, just removes the thing entierly and skips it
      if intensity <= 16 {continue;}
      for i in 0..8 {
        if adjace[i].home.get(&id) <= Some(&8) {continue;}
        adjace[i].home.entry(id).and_modify(|v| *v += 1);
        hm_keys.push(id);
      }
    }
    for (&id,&intensity) in &self.food {
      // If the strength is too low, just removes the thing entierly and skips it
      if intensity <= 16 {continue;}
      for i in 0..8 {
        if adjace[i].food.get(&id) <= Some(&8) {continue;}
        adjace[i].food.entry(id).and_modify(|v| *v += 1);
        fd_keys.push(id);
      }
    }
    // updates all the self-phers to be correct
    for id in en_keys {
      self.enemy.entry(id).and_modify(|v| *v -= 2);
    }
    for id in hm_keys {
      self.home.entry(id).and_modify(|v| *v -= 2);
    }
    for id in fd_keys {
      self.food.entry(id).and_modify(|v| *v -= 2);
    }

  }
  fn remove_pher(&mut self, id:u16) {
    if self.colony_num == 0 {panic!(
      "Tried to remove a colony-pheromone set from a pher that doesn't have any pheremones"
    )}
    self.enemy.remove(&id);
    self.home.remove(&id);
    self.food.remove(&id);
    self.colony_num -= 1;
  }
  
}