use super::entities::*;
use macroquad::prelude::*;

pub enum Entities {
	Dot,
	Polygon,
}

pub fn draw_mouse_icon(is_creating: bool, creating_entity: &mut Entities) {
	if !is_creating {
		match creating_entity {
			Entities::Dot => {
				draw_circle_lines(
					mouse_position().0 + 15.,
					mouse_position().1 + 15.,
					10.,
					1.,
					YELLOW,
				);
			}
			Entities::Polygon => {
				draw_rectangle_lines(
					mouse_position().0 + 5.,
					mouse_position().1 + 5.,
					20.,
					20.,
					2.,
					YELLOW,
				);
			}
		}
	}
	draw_circle(mouse_position().0, mouse_position().1, 2., WHITE);
}

pub fn spawn_entity(
	entity_type: &Entities,
	dots: &mut Vec<dot::Dot>,
	polygons: &mut Vec<polygon::Polygon>,
	drawing_polygon: &mut incomplete_polygon::IncompletePolygon,
) {
	match entity_type {
		Entities::Dot => {
			dots.push(dot::Dot::new(
				Some(Vec2::new(mouse_position().0, mouse_position().1)),
				None,
			));
		}
		Entities::Polygon => {
			drawing_polygon.add_point(Vec2::new(mouse_position().0, mouse_position().1), polygons);
		}
	}
}
