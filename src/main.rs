use macroquad::prelude::*;
// pub use crate::colony:: as Colony;


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
#[macroquad::main(window_conf)]
async fn main() {
  // rand::srand(miniquad::date::now().to_bits());
  // let g: usize = 1;
  // let mut colonies = [];
  // for i in 0..g {
  //     colonies[i] = holder::Collection::new_collection(200)
  // }
  // let mut mappy = Collection::new_collection(500);
  let test1 = Ant::initial_spawn(
    750, 500, 0
  );
  let mut test0: Vec<Ant> = vec![test1.0,test1.1,test1.2,test1.3,test1.4,test1.5,test1.6];
  for _i in 0..2 {
    let test1 = Ant::initial_spawn(
      750, 500, 0
    );
    let mut temp2: Vec<Ant> = vec![test1.0,test1.1,test1.2,test1.3,test1.4,test1.5,test1.6];
    test0.append(&mut temp2);
  }
  for i in 0..((&test0).len()) {test0[i].accelerate(1);}

  loop {
    //quit option
    clear_background(Color::new(0.,0.,0.1,1.0));
    if is_key_down(KeyCode::Escape){
      break;
    }
    if is_mouse_button_down(MouseButton::Left) {

    }
    // for mut i in 0..colonies.len() {
    // // Draw the simulation
      
      // /*
    //   Testing
    {
      // Test something
      {

      }
      // Random move Ants
      {
        for i in 0..((&test0).len()) {
          test0[i].move_forward();
          let chance = rand::gen_range(0, 10);
          if chance > 5 {
            match chance {
              6=> test0[i].turn_far_left(),
              7=> test0[i].turn_left(),
              8=> test0[i].turn_right(),
              9=> test0[i].turn_far_right(),
              _=> panic!("Test error: Movement")
            }
          }
        }
      }
      

      //Draw
      {
        let temp = test0.clone();
        let it = temp.iter();
        for i in it {
          i.draw_ant();
        }
        draw_text(&format!("time: {}",get_time().round()), screen_width()-500., screen_height()-5., 18., LIGHTGRAY);
      }
    }
        // */

      // // ColonyImplimintation::draw_colony(&colony);
      // // draw_text(&format!("FPS: {}, ants: {}, Food: {}, SD: {}x{}, time: {}", get_fps(), ants.len(), pieces.len(), screen_width(), screen_height(), get_time()),
      // // screen_width()-500., screen_height()-5.,
      // // 18.,
      // // LIGHTGRAY);
      //     holder::Collection::step(&mut colonies[i]);
      // }
      // Collection::test(&mappy);

      // Collection::step(&mut mappy);
      next_frame().await
  }
}