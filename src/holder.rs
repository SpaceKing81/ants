use crate::tings::Things;
use std::collections::HashMap;
use macroquad::{prelude::*, miniquad::native::apple::frameworks::Object};

#[derive(Clone)]
pub struct Collection<'a>{
  Everything: HashMap<&'a str, HashMap<&'a str, Vec<Things>>>,
}

impl Collection<'static> {
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
      let phers_all = self.Everything.get_mut("Pher").unwrap();
      for (_key, i) in phers_all {
        old_scents.append(i);
      }
  
      old_scents = Things::disperse(old_scents);
      let f = "Foodp";
      let h = "Homep";
      let d = "Dangerp";
      let t = "To homep";
      
      let final_scents = Things::pher_sorter(old_scents, f,d, t, h);
      self.Everything.insert("Pher", final_scents);
    }


    { //dead clean up and convertion

      if let Some(all_ants) = self.Everything.get_mut("Ants") {
        for (_group, ants) in all_ants.iter_mut() { // goes through each ant type
          for ant in ants.iter_mut() { // goes over every ant in each ant type
            ant.check_dead_mut()  // check_dead_mut() modifies the 'dead' field so dead things can be found easily
          }
        } 
        let mut dead_ones: Vec<Things> = self.Everything["Ants"]
        .clone() //Clones the Hashmap so that you can make one of just dead ants
        .into_iter() // Take ownership of All_ants
        .flat_map(|(_key, ant_type)| ant_type.into_iter()) // Flatten the nested vectors
        .filter(|ant| ant.dead) // Filter dead ants
        .collect(); // Collect them into a new vector

      

      // Converts the dead ants to raw food
      let mut food_final = Things::convert_to_food(dead_ones);
      // Puts the new food into the raw food vector
      self.Everything.get_mut("Food").unwrap().get_mut("Raw_Food").unwrap().append(&mut food_final);

      
      
      }
      
      
      // Consideration: What to do with picked food that ants who just died were holding
      // Also, maybe give the dead a bit of time, maybe a holding cell where the recent dead are stored until an amount of time passes
      // for them to be 'decomposed' enough or something. Idk.
      
      
      
      
      }
  

    { //food globbing
      // Does things cuz for some reason cant directly call it
      let food = self.Everything.get_mut("Food").unwrap().get_mut("Raw_Food").unwrap();
      // Calls the glob food fn and globs the food
      Things::glob_food(food, 10.)
    }


    { // update ants
      let ants = self.Everything.get_mut("Ants");

      { //update queen
        let queens = ants.unwrap().get_mut("Raw_Food").unwrap();

            
            
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



