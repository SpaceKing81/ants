use std::array;
use crate::Ant;
#[derive(Clone)]
#[derive(Copy)]
enum Presence {
  Ant(u16),
  Food(),
  Nothing(),
}
impl Presence {
  fn new_field(x:usize,y:usize) -> Vec<Vec<Presence>> {
    vec![vec![Presence::Nothing();x];y]
  }
  pub fn check_enemy(&self, ant:&Ant) -> bool {
    match self {
      Presence::Ant(u) => *u != ant.id,
      _=> false,
    }
  }
  fn check_food(&self) -> bool {
    match self {
      Presence::Food() => true,
      _=> false,
    }
  }
  
  fn check_near(&self, ant:&Ant) -> Vec<Vec<Presence>> {
    let loop_range:usize = ant.get_detection_range();
    let mut around:Vec<Vec<Presence>> = vec![vec![Presence::Nothing();loop_range];loop_range];
    let x0 = ant.pos.x;
    let y0 = ant.pos.y;
    let range = loop_range as i32;
// work in progress, this. Not sure how to reconicle the issues of type and whatever
    for n in 0..loop_range {
      for i in 0..loop_range {
        let x = n as i32;
        let y = i as i32;
        around[n][i] = self[x0+x][y0+y];
      }
    }
    around
  }


}
