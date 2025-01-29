use std::hash::Hash;
use::macroquad::prelude::*;
use crate::matrix::Matrix;


#[derive(Clone)]
pub struct Food {
  mass: u32,
}

impl Food {
  fn new(mass:u32) -> Food {
    let new = Food {
      mass,
    };
    new
  }

  fn new_field(width:usize,height:usize) -> Matrix<Food> {
    Matrix::new(height, width, Self::new(0))
  }
}