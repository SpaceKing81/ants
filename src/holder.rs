use crate::thingers::Things;
use macroquad::{prelude::*, miniquad::native::apple::frameworks::Object};


struct Collection {
  Everything: Vec<Vec<Vec<Things>>>,
}

impl Collection {
  pub fn new_collection(initial_food_pieces: u128) -> Collection { //generates the barebones structure for the thingy
    let mut Queens = Vec::new();
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
    let mut All_ants = vec![Queens, Soldiers, Scouts, Defenders, Workers];
    let mut All_food = vec![Raw_food, Delivered_food, Held_food];
    let mut All_scents = vec![Pher_f, Pher_d, Pher_t, Pher_h];
    let mut Everything: Vec<Vec<Vec<Things>>> = vec![All_ants, All_food, All_scents];

    let Testing = Self { Everything };
    Testing
  }
  fn step(&self) {
    /*
    first- update the phers, cuz they dont require anything exept the old phers to interact with an be updated
    second - update all the dead ants into food bits
    third- update the food, both positions and size and combination and stuff
    fourth - update the queen
    fifth - update the defenders
    sixth - update the scouts
    seventh - update the soldiers
    eighth - update the workers
     */
    //for q in self.Everything[2].into_iter() 
    let mut old_scents: Vec<Things> = Vec::new();
    old_scents.append(self.Everything[2][0])


    }



  }

impl Collection { //Animation
  fn animate(colony: Collection) {

  }
}



