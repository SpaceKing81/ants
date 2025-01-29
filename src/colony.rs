use::std::collections::HashMap;
use crate::ants::Ant;

// The personal map that is referenced by individual colonies,
// basiclly, what they "know"
pub struct Colony {
  id:u16,
  minimap: HashMap<(usize,usize),bool>,
}

impl Colony {
  // minimum id should be 1
  pub fn new(id:u16) -> Self {
    Colony {
      id,
      minimap: Ant::new_fogowar(),
    }
  }
  pub fn first_colony() -> HashMap<u16,Self> {
    let mut new = HashMap::new();
    new.insert(1,Self::new(1));
    new

  }
  


}