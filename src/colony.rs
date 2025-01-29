use::std::collections::HashMap;
use crate::matrix::Matrix;
use crate::pheremones::Pher;

// The personal map that is referenced by individual colonies,
// basiclly, what they "know"
struct Colony {
  id:u16,
  minimap: HashMap<(usize,usize),bool>,
  phers: Matrix<Pher>,
}