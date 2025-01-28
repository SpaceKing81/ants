use std::collections::HashMap;

use crate::matrix::Matrix;
use crate::food::Food;

// Holds the "Grand Map", which is the map referenced by all,
// along with the master clock, and the food matrix, which is
// overlayed over and essencially the same thing, but not.
// Finally, there is the life map, the thing holding all
// The colonies
struct Map {
  width: usize,
  height: usize,

  food: Matrix<Food>,
  life: HashMap<u16, Colony>,
  space: Matrix<bool>,
  time: usize,
}
// The personal map that is referenced by individual colonies,
// basiclly, what they "know"
struct Colony {
  id:u16,
  minimap: HashMap<(usize,usize),bool>,
  

}