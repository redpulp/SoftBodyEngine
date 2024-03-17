use entities::*;
use input::*;
use macroquad::input::*;
use macroquad::prelude::*;

pub mod entities {
    pub mod dot;
    pub mod incomplete_polygon;
    pub mod polygon;
    pub mod segment;
}

pub mod input;

#[macroquad::main("Soft Body Simulation")]
async fn main() {
    let mut is_creating;
    let mut creating_entity: Entities = Entities::Dot;

    // let floor = polygon::Polygon::generate_floor();
    // let triangle = polygon::Polygon::new(
    //     [
    //         (20., screen_height() / 2.),
    //         (screen_width() - 100., screen_height() - 50.),
    //         (20., screen_height() - 50.),
    //     ]
    //     .to_vec(),
    //     None,
    // );
    // let wall = polygon::Polygon::new(
    //     [
    //         (screen_width() - 20., screen_height() - 30.),
    //         (screen_width() - 20., screen_height() - 200.),
    //         (screen_width() - 10., screen_height() - 200.),
    //         (screen_width() - 10., screen_height() - 30.),
    //     ]
    //     .to_vec(),
    //     None,
    // );

    let mut dots: Vec<dot::Dot> = [].to_vec();
    let mut polygons: Vec<polygon::Polygon> = [].to_vec();
    let mut drawing_polygon = incomplete_polygon::IncompletePolygon::new();

    show_mouse(false);
    loop {
        is_creating = is_mouse_button_down(MouseButton::Left);

        match get_last_key_pressed() {
            Some(KeyCode::C) => {
                creating_entity = Entities::Dot;
            }
            Some(KeyCode::R) => {
                creating_entity = Entities::Polygon;
            }
            _ => {}
        }

        draw_mouse_icon(is_creating, &mut creating_entity);

        if is_mouse_button_pressed(MouseButton::Left) {
            input::spawn_entity(
                &creating_entity,
                &mut dots,
                &mut polygons,
                &mut drawing_polygon,
            );
        }
        polygons.iter().for_each(|poly| {
            poly.draw();
        });

        dots.iter_mut().for_each(|single_dot| {
            single_dot.update();
            polygons.iter().for_each(|poly| {
                single_dot.handle_collision(poly);
            });
            single_dot.draw();
        });

        drawing_polygon.draw();

        next_frame().await
    }
}
