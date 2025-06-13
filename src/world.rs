use crate::{ants::{Defender, Explorer, Queen, Soldier, Worker}, food::Food, matrix::Matrix, pher::PherMap};


pub struct World {
  physical_map:Matrix<bool>,
  pher_maps:Vec<PherMap>,
  // fog_of_war_maps:Vec<>,
  queens:Vec<Queen>,
  workers:Vec<Worker>,
  explorers:Vec<Explorer>,
  soldiers:Vec<Soldier>,
  defenders:Vec<Defender>,
  food:Matrix<Food>,

  num_of_colony:u32,
}