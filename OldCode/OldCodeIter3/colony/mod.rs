use std::clone;

use crate::{Ant};
use macroquad::{
  prelude::*, rand, math
};
/*
Notes:
-Alter the ant spawning to be weight-based on pher when pheremones are set up 
for better spawning habits
  -ie: defenders increses spawn freqency when there are danger phermones,
  and self-balencing based on number of individual ants detected
-Actually implement large-scale control of the macro stuff here

*/


