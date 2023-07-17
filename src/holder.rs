use crate::thingers::{Things};
use macroquad::{prelude::*, miniquad::native::apple::frameworks::Object};


struct Collection {
  Everything: Vec<Vec<Vec<Things>>>,
}

impl Collection {
  fn new_collection(initial_food_pieces: u128) -> Collection {
    let Queens = Vec::new();
    let Soldiers = Vec::new();
    let Scouts = Vec::new();
    let Defenders = Vec::new();
    let Workers = Vec::new();
    let Pher_t = Vec::new();
    let Pher_f = Vec::new();
    let Pher_h = Vec::new();
    let Pher_d = Vec::new();
    let Raw_food = Vec::from(Things::new_food(initial_food_pieces));
    let Delivered_food = Vec::new();
    let All_ants = vec![Queens ,Soldiers, Scouts, Defenders, Workers];
    let All_food = vec![Raw_food, Delivered_food];
    let All_scents = vec![Pher_f, Pher_d, Pher_t, Pher_h];
    let Everything: Vec<Vec<Vec<Things>>> = vec![All_ants, All_food, All_scents];

    let Testing = Self { Everything };
    Everything
  }

}




