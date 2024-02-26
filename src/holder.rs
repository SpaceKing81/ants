use crate::tings::Things;
use std::collections::HashMap;
use macroquad::{prelude::*, miniquad::native::apple::frameworks::Object};

#[derive(Clone)]
pub struct Collection<'a>{
  Everything: HashMap<&'a str, HashMap<&'a str, Vec<Things>>>,
}

impl Collection<'static>{
  pub fn new_collection<'a>(initial_food_pieces:u128) -> Collection<'a> { //generates the barebones structure for the tingy
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

    let Testing: Collection<'a> = Self { Everything };
    Testing
  }
  pub fn step(&mut self) {
    /*
    I put each step in its own little thingy. Might remove them if cross-reaching is hampered, but right now it just helps to look at
    first- update the phers, cuz they dont require anything exept the old phers to interact with an be updated - check
    second - update all the dead ants into food bits - need to finish removing the dead ants from the Ants vec before putting into food
    third- make the food glob together (fixed the issues?) - check?
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
      for (key,i) in self.Everything["Pher"] {
        old_scents.append(&mut i);
      }
  
      old_scents = Things::disperse(old_scents);
      let F = "Foodp";
      let H = "Homep";
      let D = "Dangerp";
      let T = "To homep";
      
      let mut final_scents = Things::pher_sorter(old_scents, F,D, T, H);
      
      for (key,i) in self.Everything["Pher"] {
        i.clear();
        match key {
          F => i = final_scents[F],
          H => i = final_scents[H],
          T => i = final_scents[T],
          D => i = final_scents[D],
          _=> println!("Error at Step- Pher Spread- Key values"),
        }
      }
    }


    { //dead clean up and convertion
      // let mut new_food: Vec<Things> = Vec::new();
  
      self.Everything["Ants"]
      .iter_mut() // Use iter_mut() to get mutable references
      .flat_map(|(_key, i)| i.iter_mut())
      .for_each(|j| j.check_dead_mut()); // check_dead_mut() modifies the 'dead' field so dead things can be found easily
  
      // Extract the dead ants and append them to the Raw_food vector
      let mut dead_ones: Vec<Things> = self.Everything["Ants"]
      .clone()
      .into_iter() // Take ownership of All_ants
      .flat_map(|(key, ant_type)| ant_type.into_iter()) // Flatten the nested vectors
      .filter(|ant| ant.dead) // Filter dead ants
      .collect(); // Collect them into a new vector
      

      /* finish this code here!!!!!!!
      self.Everything["Ants"]
      .iter_mut() // Use iter_mut() to get mutable references
      .flat_map(|(key, i)| i.remove(index))
      .for_each(|j| ); //Removes dead ants from ants
 */
      self.Everything["Food"]["Raw_Food"].append(&mut Things::convert_to_food(dead_ones)); // Append dead ants to Raw_food after makeing them food
    }
  

    {//food globbing
      Things::glob_food(&mut self.Everything["Food"]["Raw_Food"], 10.)
    }


    { // update ants

      { //update queen

        for i in self.Everything["Ants"]["Queens"] {






        }
        
        /*
        override: move away from danger gains absolute priority
        
        always per tick, emit Home phers
        
        if not close to food, move to food 
        else, consume food

        stay close to Home phers, high weight to staying around the same location
        
        if food exedes theshold, make more ants according to skewed phers
        

         */


      }

    }


  }


  
  
  pub fn test(&self) {
    self.draw_all()
  }
}



impl Collection<'static>{ //Animation
  fn animate(colony: Collection) {}
  
  fn draw_all(&self){
    for (key,i) in &self.Everything {
      for (dou_key,n) in i {
        for w in n {
          Things::color_shaper(&w);
        }
      }
    }
  }
}



