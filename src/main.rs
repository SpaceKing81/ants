use macroquad::prelude::*;

mod ants;
mod ants_collection;

use ants_collection::{};

fn window_conf() -> Conf {

// Personal screen size for best results
    let pdx = 2048;
    let pdy = 1152;

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


    loop {

        //Draw the simulation
        // clear_background(Color::new(0.,0.,0.1,1.0));
        // ants.draw_ant();
        // pieces.draw_piece();
        // ColonyImplimintation::draw_colony(&colony);
        // draw_text(&format!("FPS: {}, ants: {}, Food: {}, SD: {}x{}, time: {}", get_fps(), ants.len(), pieces.len(), screen_width(), screen_height(), time),
        // screen_width()-500., screen_height()-5.,
        // 18.,
        // LIGHTGRAY);

        //Advance the data
        // time +=1;

        next_frame().await
    }
}