use macroquad::prelude::*;

mod ants;
mod ants_collection;

use ants_collection::{WorkerAntCollection, ColonyImplimintation, FoodCollection};

fn window_conf() -> Conf {

// Personal screen size for best results
    let Pdx = 2048;
    let Pdy = 1152;

    Conf {
        window_title: "Ants".to_owned(),
        fullscreen: true,
        window_height:Pdy,
        window_width:Pdx,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {

    // rand::srand(miniquad::date::now().to_bits());

    let colony = ColonyImplimintation::new();
    let mut ants = WorkerAntCollection::new(600, colony);
    let mut pieces = FoodCollection::new(600);
    let colony = ColonyImplimintation::new();
    let mut time:u128 = 0;
    loop {

        //Draw the simulation
        clear_background(Color::new(0.,0.,0.1,1.0));
        ants.draw_ant();
        pieces.draw_piece();
        ColonyImplimintation::draw_colony(&colony);
        draw_text(&format!("FPS: {}, ants: {}, SD: {}x{}, time: {}", get_fps(), ants.len(), screen_width(), screen_height(), time),
        screen_width()-400., screen_height()-5.,
        18.,
        LIGHTGRAY);

        //Advance the data
        time +=1;
        ants.step();
        colony.step();
        pieces.step();

        next_frame().await
    }
}