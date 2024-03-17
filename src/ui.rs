use super::entities::*;
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
                2.,
                YELLOW,
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
            drawing_polygon.add_point(vec2(mouse_position().0, mouse_position().1), polygons);
        }
    }
}

pub const BUTTON_SIZE: f32 = 150.;
pub const BUTTON_OFFSET: f32 = 20.;
