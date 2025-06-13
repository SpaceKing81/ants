use glam::Vec2;

use crate::{ants::{Defender, Explorer, Queen, Soldier, Worker}, food::Food, matrix::Matrix, pher::PherMap};


pub struct World {
  physical_map:Matrix<bool>, // if there is an object at this pos, either food, ant, or wall. food with 0 mass is not registered
  pher_map:PherMap,
  // fog_of_war_maps:Vec<>,
  queens:Vec<Queen>,
  workers:Vec<Worker>,
  explorers:Vec<Explorer>,
  soldiers:Vec<Soldier>,
  defenders:Vec<Defender>,
  food:Matrix<Food>,

  num_of_colony:u32,
}

impl World {
  fn new(x:usize, y:usize) -> Self {
    // make food matrix positions
    let mut food_mat = Matrix::new(y,x, Food::empty_new());
    for col in 0..x {
      for row in 0..y {
        food_mat.set(
          row, 
          col, 
          Food::new(Vec2::new(row as f32,col as f32), 0.0)
        ).expect("def shouldn't panic")
      }
    }

    World {
      physical_map: Matrix::new(y,x, false),
      pher_map: PherMap::new(x,y),
      queens: Vec::new(),
      workers: Vec::new(),
      explorers: Vec::new(),
      soldiers: Vec::new(),
      defenders: Vec::new(),
      food: food_mat,
      num_of_colony: 0,
    }
  }
}