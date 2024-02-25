use crate::tings::Things;
use std::collections::HashMap;
use macroquad::{prelude::*, miniquad::native::apple::frameworks::Object};

#[derive(Clone)]
pub struct Collection<'a>{
  Everything: HashMap<&'a str, HashMap<&'a str, Vec<Things>>>,
}

impl Collection{
  pub fn new_collection(initial_food_pieces: u128) -> Collection { //generates the barebones structure for the tingy
    let mut Queens = vec![Things::new_queen(rand::gen_range(0., screen_width()),rand::gen_range(0., screen_height()) , 0.)];
    let mut Soldiers = Vec::new();
    let mut Scouts = Vec::new();
    let mut Defenders = Vec::new();
    let mut Workers = Vec::new();
    let mut Held_food: Vec<Things> = Vec::new();
    let mut Pher_t = Vec::new();
    let mut Pher_f = Vec::new();
    let mut Pher_h = Vec::new();
    let mut Pher_d = Vec::new();
    let mut Raw_food = Vec::from(Things::new_food(initial_food_pieces));
    let mut Delivered_food = Vec::new();
    let mut All_ants = [("Queens", Queens), ("Soldiers",Soldiers), ("Scouts",Scouts), ("Defenders",Defenders), ("Workers",Workers)]
      .iter()
      .cloned()
      .collect();
    let mut All_food:HashMap<&str, Vec<Things>> = [("Raw_food",Raw_food), ("Delivered_food",Delivered_food), ("Held_food",Held_food)]
      .iter()
      .cloned()
      .collect();
    let mut All_scents = [("Foodp",Pher_f), ("Dangerp",Pher_d), ("To homep",Pher_t), ("Homep",Pher_h)]
      .iter()
      .cloned()
      .collect();
    let mut Everything:HashMap<&str, HashMap<&str, Vec<Things>>> = [("Ants",All_ants), ("Food",All_food), ("Pher",All_scents)]
      .iter()
      .cloned()
      .collect();

    let Testing = Self { Everything };
    Testing
  }
  pub fn step(&mut self) {
    /*
    I put each step in its own little thingy. Might remove them if cross-reaching is hampered, but right now it just helps to look at
    first- update the phers, cuz they dont require anything exept the old phers to interact with an be updated - check
    second - update all the dead ants into food bits - check
    third- make the food glob together (still have issues with this)
    fourth - update the queen
    fifth - update the defenders
    sixth - update the scouts
    seventh - update the soldiers
    eighth - update the workers
    ninth - draw everything
     */
    //for q in self.Everything[2].into_iter() 
    {//step one, pher spread
      let mut old_scents: Vec<Things> = Vec::new();
      for i in 0..3 {
        old_scents.append(&mut self.Everything[2][i])
      }
  
      old_scents = Things::disperse(old_scents);
      
      let mut final_scents = Things::pher_sorter(old_scents);
      
      for i in 0..3 {
        self.Everything[2][i].clear();
        self.Everything[2][i].append(&mut final_scents[i]);
      }
    }


    { //dead clean up and convertion
      let mut new_food: Vec<Things> = Vec::new();
  
      self.Everything[0]
      .iter_mut() // Use iter_mut() to get mutable references
      .flat_map(|i| i.iter_mut())
      .for_each(|j| j.check_dead_mut()); // Assuming check_dead_mut() modifies the 'dead' field
  
      // Extract the dead ants and append them to the Raw_food vector
      let mut dead_ones: Vec<Things> = self.Everything[0]
      .clone()
      .into_iter() // Take ownership of All_ants
      .flat_map(|ant_type| ant_type.into_iter()) // Flatten the nested vectors
      .filter(|ant| ant.dead) // Filter dead ants
      .collect(); // Collect them into a new vector
  
      self.Everything[1][0].append(&mut Things::convert_to_food(dead_ones)); // Append dead ants to Raw_food after makeing them food
    }
  

    {//food globbing
      Things::glob_food(&mut self.Everything[1][0], 10.)
    }

  }


  
  pub fn draw_all(&self){
    for (key,i) in &self.Everything {
      for (dou_key,n) in i {
        for w in n {
          Things::color_shaper(&w);
        }
      }
    }
  }
  pub fn test(&self) {
    self.draw_all()
  }
}



impl Collection { //Animation
  fn animate(colony: Collection) {

  }
  
}



