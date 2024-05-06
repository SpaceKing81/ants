use crate::tings::*;
use std::collections::HashMap;
use macroquad::{prelude::*, miniquad::native::apple::frameworks::Object};

#[derive(Clone)]
pub struct Collection{
  everything: Vec<Vec<Vec<Things>>>,
}

impl Collection{
  pub fn new_collection(initial_food_pieces:u128) -> Collection { //generates the barebones structure for the tingy
    let mut queens = vec![Things::new_queen(rand::gen_range(0., screen_width()),rand::gen_range(0., screen_height()) , 0.)];
    let mut soldiers = Vec::new();
    let mut scouts = Vec::new();
    let mut defenders = Vec::new();
    let mut workers = Vec::new();
    let mut pher_t = Vec::new();
    let mut pher_f = Vec::new();
    let mut pher_h = Vec::new();
    let mut pher_d = Vec::new();
    let mut held_food: Vec<Things> = Vec::new();
    let mut raw_food = Vec::from(Things::new_food(initial_food_pieces));
    let mut delivered_food = Vec::new();
    let mut all_ants = vec![queens,soldiers,scouts,defenders,workers];
    let mut all_food:Vec<Vec<Things>> = vec![raw_food, delivered_food, held_food];
    let mut all_scents = vec![pher_f, pher_d, pher_t, pher_h];
    let mut everything:Vec<Vec<Vec<Things>>> = vec![all_ants,all_food,all_scents];

    let testing: Collection = Self { everything };
    testing
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
    //for q in self.everything[2].into_iter() 
    {//step one, pher spread
      let mut old_scents: Vec<Things> = Vec::new();
      let phers_all = self.everything[2].clone();
      for mut i in phers_all {
        old_scents.append(&mut i);
      }
  
      old_scents = Things::disperse(old_scents);

      
      let final_scents = Things::pher_sorter(old_scents);
      self.everything[2] = final_scents;
    }


    { //dead clean up and convertion
      let mut deads:Vec<Things> = Vec::new();
      for i in &self.everything[0] {
        for j in i {
          let mut current = j;
          if current.check_dead_mut() {
            deads.push(*j);
          }
          
          // INCOMPLETE!!!! NEED TO FINISH MAKING THE DEAD CHECKER AND SETTER
          /* 
          For some reason cannot change the dead field of the Things without causing errors. Also will not allow the removal of a Thing from
          the vec and transfer to another, and I cant figure out why. On top of that, im tired and frusterated with this stupid code, and chat
          is the epitome of unhelpful to the point of absolute rage quitting.
          */ 
        }
      }


      let mut dead_ones: Vec<Things> = self.everything[0]
      .clone() //Clones the Hashmap so that you can make one of just dead ants
      .into_iter() // Take ownership of All_ants
      .flat_map(|(ant_type)| ant_type.into_iter()) // Flatten the nested vectors
      .filter(|ant|ant.dead) // Filter dead ants
      .collect(); // Collect them into a new vector

      

      // Converts the dead ants to raw food
      let mut food_final = Things::convert_to_food(dead_ones);
      // Puts the new food into the raw food vector
      self.everything[1][0].append(&mut food_final);

      
      
      // Consideration: What to do with picked food that ants who just died were holding
      // Also, maybe give the dead a bit of time, maybe a holding cell where the recent dead are stored until an amount of time passes
      // for them to be 'decomposed' enough or something. Idk.
      
      
      
      
      }
  

    { //food globbing
      // Calls the glob food fn and globs the food
      Things::glob_food(&mut self.everything[2][0], 10.)
    }


    { // update ants
      let all_ants = &self.everything[0];

      { //update queen
        let queens = &all_ants[0];

        

            
            
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


impl Collection{ //Animation
  fn animate(colony: Collection) {}
  
  fn draw_all(&self){
    for i in &self.everything {
      for j in i {
        for w in j {
          Things::color_shaper(&w);
        }
      }
    }
  }
}
