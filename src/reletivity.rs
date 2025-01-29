
use std::collections::HashMap;
use std::net::UdpSocket;
use crate::matrix::Matrix;
use crate::food::Food;
use crate::pheremones::Pher;
use crate::colony::Colony;

// Holds the "Grand Map", which is the map referenced by all,
// along with the master clock, and the food matrix, which is
// overlayed over and essencially the same thing, but not.
// Finally, there is the life map, the thing holding all
// The colonies
struct Map {
  width: usize,
  height: usize,
  colonynum:u16,
  
  food: Matrix<u32>,
  life: HashMap<u16, Colony>,
  pher: HashMap<u16, Pher>,

  space: Matrix<bool>,
  time: usize,
  dt: usize,
}

impl Map {
  fn new(width:usize, height:usize, dt:usize) -> Self {
    Map {
      width,
      height,
      colonynum:1,

      food:Self::new_food_field(width, height),
      life:Colony::first_colony(),
      pher:Self::new_pher_field(width,height),

      space:Self::space_new(width,height),
      time:0,
      dt,
    }
  }
}
impl Map {
  fn space_new(width:usize, height:usize) -> Matrix<bool> {
    Matrix::new(height, width, false)
  }
  fn new_food_field(width:usize, height:usize) -> Matrix<u32> {
    Matrix::new(height, width, 0)
  }
  fn new_pher_field(width:usize,height:usize) -> HashMap<u16, Pher>{
    let new = HashMap::new();
    new
  }

}

impl Map {
}
