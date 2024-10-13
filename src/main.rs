use macroquad::prelude::*;
use soft_body_simulation::entities::*;
use soft_body_simulation::ui::*;

#[macroquad::main("Soft Body Simulation")]
async fn main() {
    let mut creating_entity: Entities = Entities::Dot;
    let mut show_skeleton = false;

    let buttons_window_dimensions = (265., 155.);

    let mut polygons: Vec<polygon::Polygon> = vec![polygon::Polygon::generate_floor()];
    let mut drawing_polygon = incomplete_polygon::IncompletePolygon::new();

    let mut soft_body = soft_body::SoftBody::new(screen_width() / 2., screen_height() / 2.);

    let [polygon_button, stop_drawing_button, reset_button, soft_body_button, show_skeleton_button, show_border_button] =
        spawn_buttons();

    loop {
        clear_background(BLACK);
        let is_mouse_on_buttons = mouse_position().0 < buttons_window_dimensions.0
            && mouse_position().1 < buttons_window_dimensions.1;
        let is_creating_polygon = !drawing_polygon.points.is_empty();

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Controller")
                .fixed_rect(egui::Rect {
                    min: egui::Pos2::new(0., 0.),
                    max: egui::Pos2::new(250., 100.),
                })
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    // let mut scalar = 12;
                    // ui.add(egui::Slider::new(&mut scalar, 0..=12).suffix("°"));

                    if is_creating_polygon {
                        if ui.button(stop_drawing_button.clone()).clicked() {
                            drawing_polygon.reset();
                        }
                    } else {
                        if ui.button(soft_body_button.clone()).clicked() {
                            creating_entity = Entities::Dot;
                        }
                        if ui.button(polygon_button.clone()).clicked() {
                            creating_entity = Entities::Polygon;
                        }
                    }
                    ui.separator();
                    if show_skeleton {
                        if ui.button(show_border_button.clone()).clicked() {
                            show_skeleton = false;
                        }
                    } else if ui.button(show_skeleton_button.clone()).clicked() {
                        show_skeleton = true;
                    }
                    if ui.button(reset_button.clone()).clicked() {
                        drawing_polygon.reset();
                        polygons = vec![polygon::Polygon::generate_floor()];
                    }
                });
        });

        // Draw things before egui

        egui_macroquad::draw();

        // Listening for user events
        if is_mouse_button_pressed(MouseButton::Left) && !is_mouse_on_buttons {
            spawn_entity(
                &creating_entity,
                &mut polygons,
                &mut drawing_polygon,
                &mut soft_body,
            );
        }

        // Drawing polygons
        polygons.iter().for_each(|poly| poly.draw());

        // Drawing In-progress polygon
        drawing_polygon.draw(&polygons, &soft_body);

        soft_body.update_runge_kutta();

        polygons.iter().for_each(|poly| {
            soft_body.handle_collision(poly);
        });
        if show_skeleton {
            soft_body.draw();
        } else {
            soft_body.draw_border();
        }

        draw_mouse_icon(&mut creating_entity);

        // For lower FPS
        //std::thread::sleep(std::time::Duration::from_millis(50));

        next_frame().await
    }
}
