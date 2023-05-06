use macroquad::prelude::*;

mod ants;
mod ants_collection;

use ants_collection::WorkerAntCollection;

fn window_conf() -> Conf {
    Conf {
        window_title: "Ants".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {

    rand::srand(miniquad::date::now().to_bits());
    let mut ants = WorkerAntCollection::new(600);

    loop {
        ants.step();
        clear_background(Color::new(0.,0.,0.1,1.0));
        ants.draw();
        draw_text(&format!("FPS: {}, ants: {}", get_fps(), ants.len()),
            screen_width()-200., screen_height()-5.,
            18.,
            LIGHTGRAY);
        next_frame().await
    }
}