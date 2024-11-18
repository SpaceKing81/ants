use std::*;
use macroquad::{
  miniquad::{gl::PFNGLCOMPRESSEDTEXIMAGE1DPROC, native::apple::frameworks::Object},
  prelude::*, rand, math
};


struct Colony {
  pos: Vec2,
  hp: u32,
  hunger: u32,
}
