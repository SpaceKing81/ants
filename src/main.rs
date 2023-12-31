mod holder;
mod tings;
use holder::Collection;
use macroquad::prelude::*;
use tings::Things;


fn window_conf() -> Conf {

// Personal screen size for best results
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
    let mappy = Collection::new_collection(500);
    loop {
        // for mut i in 0..colonies.len() {
        
        // // Draw the simulation
        clear_background(Color::new(0.,0.,0.1,1.0));
        // // ants.draw_ant();
        // // pieces.draw_piece();
        // // ColonyImplimintation::draw_colony(&colony);
        // // draw_text(&format!("FPS: {}, ants: {}, Food: {}, SD: {}x{}, time: {}", get_fps(), ants.len(), pieces.len(), screen_width(), screen_height(), time),
        // // screen_width()-500., screen_height()-5.,
        // // 18.,
        // // LIGHTGRAY);
        //     holder::Collection::step(&mut colonies[i]);
        // }
        Collection::test(&mappy);
        // clear_background(Color::new(0.18,0.09,0.,1.0));
        // time +=1;
        next_frame().await
    }
}
