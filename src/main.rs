mod colony;
mod ants;
use macroquad::prelude::*;
// pub use crate::colony:: as Colony;
pub use crate::ants::Ant as Ant;


fn window_conf() -> Conf {

  // Personal screen size for best results
  // For rn at least
      let pdx = 1512;
      let pdy = 1964;
  
      Conf {
          window_title: "Ants".to_owned(),
          fullscreen: true,
          window_height:pdy,
          window_width:pdx,
          ..Default::default()
      }
  }
  
  
  #[macroquad::main(window_conf())]
  async fn main() {
      // rand::srand(miniquad::date::now().to_bits());
      // let g: usize = 1;
      // let mut colonies = [];
      // for i in 0..g {
      //     colonies[i] = holder::Collection::new_collection(200)
      // }
      // let mut mappy = Collection::new_collection(500);
    let test = Ant::initial_spawn(
        750, 750, 500, 500
    );
      loop {
          // for mut i in 0..colonies.len() {
          // // Draw the simulation
          clear_background(Color::new(0.,0.,0.1,1.0));
          
          test.0.draw_ant();
          test.1.draw_ant();
          test.2.draw_ant();
          test.3.draw_ant();
          test.4.draw_ant();
          // // pieces.draw_piece();
          // // ColonyImplimintation::draw_colony(&colony);
          // // draw_text(&format!("FPS: {}, ants: {}, Food: {}, SD: {}x{}, time: {}", get_fps(), ants.len(), pieces.len(), screen_width(), screen_height(), time),
          // // screen_width()-500., screen_height()-5.,
          // // 18.,
          // // LIGHTGRAY);
          //     holder::Collection::step(&mut colonies[i]);
          // }
          // Collection::test(&mappy);
          // time +=1;
  
          // Collection::step(&mut mappy);
          next_frame().await
      }
  }