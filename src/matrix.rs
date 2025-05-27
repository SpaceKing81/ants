pub struct Matrix<T> {
  data:Vec<T>,
  rows:usize,
  cols:usize,
}

impl<T> Matrix<T> {
  pub fn new(rows:usize, cols:usize, default: T) -> Self where T:Clone,{
    Matrix {
      data: vec![default; rows*cols],
      rows,
      cols,
    }
  }
  pub fn get(&self, row:usize, col:usize) -> Option<&T> {
    if self.rows > row && self.cols > col {
      return None
    }
    Some(&self.data[row * self.cols + col])
  }
  pub fn set(&mut self, row:usize, col:usize, input:T) -> Result<(),String> {
    if self.rows > row && self.cols > col {
      return Err("Dumbass, how tf did you break the set fn for the matrix?".to_string())
    }
    self.data[row * self.cols + col] = input;
    Ok(())
  }
  
}