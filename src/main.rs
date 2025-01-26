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
  loop {
    //quit option
    clear_background(Color::new(0.,0.,0.1,1.0));
    if is_key_down(KeyCode::Escape){
      break;
    }

    if is_mouse_button_down(MouseButton::Left) {
      //No clue whats in here
    }
    

    // Draw the simulation
      // /*
    //   Testing
    {
      // Test something
      {

      }
      

      //Draw
      {
        


        draw_text(&format!("time: {}",get_time().round()), screen_width()-500., screen_height()-5., 18., LIGHTGRAY);
      }
    }
        // */

      // draw_text(&format!("FPS: {}, ants: {}, Food: {}, SD: {}x{}, time: {}", 
      // get_fps(), ants.len(), pieces.len(), screen_width(), screen_height(), get_time()),
      // screen_width()-500., screen_height()-5., 18., LIGHTGRAY);
      
      next_frame().await
  }
}