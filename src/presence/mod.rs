use std::array;
use crate::Ant;
use crate::Food;
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
  fn check_enemy(&self, ant:&Ant) -> bool {
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
  // not sure how to do this, just trying to get the logic to work before fitting in the loops for
  // the entire area. Should output a personal matrix for that ant of whats around it, but im not
  // entirely sure why it would need it. Leaving to go figure something else out and then coming back.
  // Maybe, this whole thing might turn out to be unnessisary.
//   fn check_near(main:Vec<Vec<&Presence>>, ant:&Ant) -> Vec<Vec<Presence>> {
//     let range:usize = ant.get_detection_range();
//     let x0 = ant.pos.x;
//     let y0 = ant.pos.y;

//     let mut around:Vec<Vec<Presence>> = vec![vec![Presence::Nothing();range];range];
// // work in progress, this. Not sure how to reconicle the issues of type and whatever
//         let n=0;
//         let i=0;
//         let current = main[n][i];
//         let enemy = current.check_enemy(ant);
//         let food = current.check_food();

//     around
//   }


}
