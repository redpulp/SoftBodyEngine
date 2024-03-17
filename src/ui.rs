use super::entities::*;
use egui::text::{LayoutJob, TextFormat, TextStyle};
use macroquad::prelude::*;

pub enum Entities {
    Dot,
    Polygon,
}

pub fn draw_mouse_icon(creating_entity: &mut Entities) {
    match creating_entity {
        Entities::Dot => {
            draw_circle_lines(
                mouse_position().0 + 20.,
                mouse_position().1 + 20.,
                10.,
                1.,
                YELLOW,
            );
        }
        Entities::Polygon => {
            draw_rectangle_lines(
                mouse_position().0 + 15.,
                mouse_position().1 + 15.,
                20.,
                20.,
                3.,
                BLUE,
            );
        }
    }
}

pub fn spawn_entity(
    entity_type: &Entities,
    polygons: &mut Vec<polygon::Polygon>,
    drawing_polygon: &mut incomplete_polygon::IncompletePolygon,
    body: &mut soft_body::SoftBody,
) {
    match entity_type {
        Entities::Dot => {
            *body = soft_body::SoftBody::new(mouse_position().0, mouse_position().1);
        }
        Entities::Polygon => {
            if !drawing_polygon.is_intersecting_with_polygons(polygons)
                && !drawing_polygon.is_intersecting_with_soft_body(body)
            {
                drawing_polygon.add_point(vec2(mouse_position().0, mouse_position().1), polygons);
            }
        }
    }
}

pub fn spawn_buttons() -> [LayoutJob; 6] {
    let mut polygon_button = LayoutJob::default();
    polygon_button.append(
        "⬜ ",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::from_rgb(99, 75, 255),
            ..Default::default()
        },
    );
    polygon_button.append(
        "Create Polygon",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::WHITE,
            ..Default::default()
        },
    );

    let mut stop_drawing_button = egui::text::LayoutJob::default();
    stop_drawing_button.append(
        "❌ ",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::RED,
            ..Default::default()
        },
    );
    stop_drawing_button.append(
        "Stop Drawing",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::RED,
            ..Default::default()
        },
    );

    let mut reset_button = egui::text::LayoutJob::default();
    reset_button.append(
        "⟲ ",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::RED,
            ..Default::default()
        },
    );
    reset_button.append(
        "Reset Canvas",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::WHITE,
            ..Default::default()
        },
    );

    let mut soft_body_button = egui::text::LayoutJob::default();
    soft_body_button.append(
        "⭕ ",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::YELLOW,
            ..Default::default()
        },
    );
    soft_body_button.append(
        "Create Soft-body",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::WHITE,
            ..Default::default()
        },
    );

    let mut show_skeleton_button = LayoutJob::default();
    show_skeleton_button.append(
        "❇ ",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::WHITE,
            ..Default::default()
        },
    );
    show_skeleton_button.append(
        "Show Soft-body Skeleton",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::WHITE,
            ..Default::default()
        },
    );

    let mut show_border_button = LayoutJob::default();
    show_border_button.append(
        "❇ ",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::WHITE,
            ..Default::default()
        },
    );
    show_border_button.append(
        "Show Soft-body Border",
        0.0,
        TextFormat {
            style: TextStyle::Heading,
            color: egui::Color32::WHITE,
            ..Default::default()
        },
    );

    [
        polygon_button,
        stop_drawing_button,
        reset_button,
        soft_body_button,
        show_skeleton_button,
        show_border_button,
    ]
}
