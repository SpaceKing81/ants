
use std::collections::HashMap;
use crate::matrix::Matrix;
use crate::food::Food;
use crate::pheremones::Pher;

// Holds the "Grand Map", which is the map referenced by all,
// along with the master clock, and the food matrix, which is
// overlayed over and essencially the same thing, but not.
// Finally, there is the life map, the thing holding all
// The colonies
struct Map {
  width: usize,
  height: usize,
  
  food: HashMap<Food>,
  life: HashMap<u16, Colony>,
  pher: HashMap<u16, Pher>,

  space: Matrix<bool>,
  time: usize,
}