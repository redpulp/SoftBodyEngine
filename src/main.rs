use entities::*;
use input::*;
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
    let mut creating_entity: Entities = Entities::Dot;
    let dot_button = input::Button::new(
        0,
        input::Actions::SetDotCreation,
        include_bytes!("../resources/dot.png").to_vec(),
        include_bytes!("../resources/dot_hover.png").to_vec(),
        include_bytes!("../resources/dot_active.png").to_vec(),
    );
    let polygon_button = input::Button::new(
        1,
        input::Actions::SetPolygonCreation,
        include_bytes!("../resources/polygon.png").to_vec(),
        include_bytes!("../resources/polygon_hover.png").to_vec(),
        include_bytes!("../resources/polygon_active.png").to_vec(),
    );
    let stop_drawing_button = input::Button::new(
        1,
        input::Actions::StopDrawing,
        include_bytes!("../resources/stop_drawing.png").to_vec(),
        include_bytes!("../resources/stop_drawing_hover.png").to_vec(),
        include_bytes!("../resources/stop_drawing_active.png").to_vec(),
    );
    let reset_button = input::Button::new(
        2,
        input::Actions::ResetCanvas,
        include_bytes!("../resources/reset.png").to_vec(),
        include_bytes!("../resources/reset_hover.png").to_vec(),
        include_bytes!("../resources/reset_active.png").to_vec(),
    );

    let buttons = [
        &dot_button,
        &polygon_button,
        &stop_drawing_button,
        &reset_button,
    ];

    let buttons_window_dimensions = (
        input::BUTTON_SIZE + input::BUTTON_OFFSET,
        (buttons.len() as f32 * input::BUTTON_SIZE) + input::BUTTON_OFFSET,
    );

    let mut dots: Vec<dot::Dot> = [].to_vec();
    let mut polygons: Vec<polygon::Polygon> = [].to_vec();
    let mut drawing_polygon = incomplete_polygon::IncompletePolygon::new();

    loop {
        clear_background(BLACK);
        draw_mouse_icon(&mut creating_entity);
        let is_mouse_on_buttons = mouse_position().0 < buttons_window_dimensions.0
            && mouse_position().1 < buttons_window_dimensions.1;

        buttons.iter().for_each(|button| {
            let is_creating_polygon = drawing_polygon.points.len() > 0;
            let mut action: Option<Actions> = None;
            match button.action_type {
                Actions::SetPolygonCreation => {
                    if !is_creating_polygon {
                        action = button.render_and_action();
                    }
                }
                Actions::StopDrawing => {
                    if is_creating_polygon {
                        action = button.render_and_action();
                    }
                }
                _ => {
                    action = button.render_and_action();
                }
            }
            match action {
                Some(input::Actions::SetDotCreation) => {
                    creating_entity = Entities::Dot;
                }
                Some(input::Actions::SetPolygonCreation) => {
                    creating_entity = Entities::Polygon;
                }
                Some(input::Actions::StopDrawing) => {
                    drawing_polygon.reset();
                }
                Some(input::Actions::ResetCanvas) => {
                    drawing_polygon.reset();
                    dots = [].to_vec();
                    polygons = [].to_vec();
                }
                None => (),
            }
        });

        if is_mouse_button_pressed(MouseButton::Left) && !is_mouse_on_buttons {
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

        // Destroying dots out of bounds
        dots.retain(|single_dot| !single_dot.is_out_of_bounds());

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
