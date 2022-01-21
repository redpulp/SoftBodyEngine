use macroquad::prelude::*;

pub mod entities {
    pub mod dot;
    pub mod polygon;
    pub mod segment;
}

use entities::*;

#[macroquad::main("Soft Body Simulation")]
async fn main() {
    let mut main_dot = dot::Dot::new(None, None);
    let floor = polygon::Polygon::generate_floor();
    let triangle = polygon::Polygon::new(
        [
            (20., screen_height() / 2.),
            (screen_width() - 100., screen_height() - 50.),
            (20., screen_height() - 50.),
        ]
        .to_vec(),
        None,
    );
    let wall = polygon::Polygon::new(
        [
            (screen_width() - 20., screen_height() - 30.),
            (screen_width() - 20., screen_height() - 200.),
            (screen_width() - 10., screen_height() - 200.),
            (screen_width() - 10., screen_height() - 30.),
        ]
        .to_vec(),
        None,
    );

    loop {
        floor.draw();
        triangle.draw();
        wall.draw();
        main_dot.update();
        main_dot.handle_collision(&floor);
        main_dot.handle_collision(&triangle);
        main_dot.handle_collision(&wall);
        main_dot.draw();
        next_frame().await
    }
}
