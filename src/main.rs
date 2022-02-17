use egui::{color::*, *};
use entities::*;
use macroquad::prelude::*;
use ui::*;

pub mod entities {
    pub mod dot;
    pub mod incomplete_polygon;
    pub mod interaction;
    pub mod polygon;
    pub mod segment;
    pub mod soft_body;
}

pub mod ui;

#[macroquad::main("Soft Body Simulation")]
async fn main() {
    let mut creating_entity: Entities = Entities::Dot;

    let buttons_window_dimensions = (
        BUTTON_SIZE + BUTTON_OFFSET,
        ([1].len() as f32 * BUTTON_SIZE) + BUTTON_OFFSET,
    );

    let mut polygons: Vec<polygon::Polygon> = [polygon::Polygon::generate_floor()].to_vec();
    let mut drawing_polygon = incomplete_polygon::IncompletePolygon::new();

    let mut soft_body = soft_body::SoftBody::new(screen_width() / 2., screen_height() / 2.);

    // Forgive this ugly button setup
    let mut polygon_button = egui::text::LayoutJob::default();
    polygon_button.append(
        "⬜ ",
        0.0,
        TextFormat {
            style: TextStyle::Body,
            color: Color32::from_rgb(99, 75, 255),
            ..Default::default()
        },
    );
    polygon_button.append(
        "Create Polygon",
        0.0,
        TextFormat {
            style: TextStyle::Body,
            color: Color32::WHITE,
            ..Default::default()
        },
    );

    let mut reset_button = egui::text::LayoutJob::default();
    reset_button.append(
        "⟲ ",
        0.0,
        TextFormat {
            style: TextStyle::Body,
            color: Color32::RED,
            ..Default::default()
        },
    );
    reset_button.append(
        "Reset Canvas",
        0.0,
        TextFormat {
            style: TextStyle::Body,
            color: Color32::WHITE,
            ..Default::default()
        },
    );

    let mut soft_body_button = egui::text::LayoutJob::default();
    soft_body_button.append(
        "⭕ ",
        0.0,
        TextFormat {
            style: TextStyle::Body,
            color: Color32::YELLOW,
            ..Default::default()
        },
    );
    soft_body_button.append(
        "Create Soft-body",
        0.0,
        TextFormat {
            style: TextStyle::Body,
            color: Color32::WHITE,
            ..Default::default()
        },
    );

    loop {
        clear_background(BLACK);
        let is_mouse_on_buttons = mouse_position().0 < buttons_window_dimensions.0
            && mouse_position().1 < buttons_window_dimensions.1;
        let is_creating_polygon = drawing_polygon.points.len() > 0;

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("Controller")
                .fixed_rect(egui::Rect {
                    min: egui::Pos2::new(0., 0.),
                    max: egui::Pos2::new(200., 100.),
                })
                .collapsible(false)
                .show(egui_ctx, |ui| {
                    // let mut scalar = 12;
                    // ui.add(egui::Slider::new(&mut scalar, 0..=12).suffix("°"));

                    if is_creating_polygon {
                        if ui
                            .button(RichText::new("❌ Stop Drawing").color(Color32::RED))
                            .clicked()
                        {
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
                    if ui.button(reset_button.clone()).clicked() {
                        drawing_polygon.reset();
                        polygons = [].to_vec();
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
        drawing_polygon.draw();

        soft_body.update();
        polygons.iter().for_each(|poly| {
            soft_body.handle_collision(poly);
        });
        soft_body.draw();

        draw_mouse_icon(&mut creating_entity);

        // std::thread::sleep(std::time::Duration::from_millis(300));
        next_frame().await
    }
}
