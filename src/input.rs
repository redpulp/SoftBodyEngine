use super::entities::*;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

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
	dots: &mut Vec<dot::Dot>,
	polygons: &mut Vec<polygon::Polygon>,
	drawing_polygon: &mut incomplete_polygon::IncompletePolygon,
) {
	match entity_type {
		Entities::Dot => {
			dots.push(dot::Dot::new(
				Some(vec2(mouse_position().0, mouse_position().1)),
				None,
			));
		}
		Entities::Polygon => {
			drawing_polygon.add_point(vec2(mouse_position().0, mouse_position().1), polygons);
		}
	}
}

#[derive(Copy, Clone, Debug)]
pub enum Actions {
	SetDotCreation,
	SetPolygonCreation,
	ResetCanvas,
	StopDrawing,
}

pub const BUTTON_SIZE: f32 = 150.;
pub const BUTTON_OFFSET: f32 = 20.;

pub struct Button {
	pub action_type: Actions,
	skin: Skin,
	order: usize,
}

impl Button {
	pub fn new(
		order: usize,
		action_type: Actions,
		background: Vec<u8>,
		background_hovered: Vec<u8>,
		background_active: Vec<u8>,
	) -> Button {
		let button_style = root_ui()
			.style_builder()
			.background(Image::from_file_with_format(&background, None))
			.background_margin(RectOffset::new(0., BUTTON_SIZE, 67., 67.))
			.margin(RectOffset::new(0., 0., 0., 0.))
			.background_hovered(Image::from_file_with_format(&background_hovered, None))
			.background_clicked(Image::from_file_with_format(&background_active, None))
			.build();
		let skin = Skin {
			button_style,
			..root_ui().default_skin()
		};
		Button { action_type, skin, order }
	}

	pub fn render_and_action(&self) -> Option<Actions> {
		let skin = &self.skin;
		root_ui().push_skin(skin);
		let mut action: Option<Actions> = None;
		root_ui().window(
			hash!(self.order),
			vec2(
				BUTTON_OFFSET,
				BUTTON_OFFSET + ((self.order as f32) * BUTTON_SIZE),
			),
			vec2(BUTTON_SIZE, BUTTON_SIZE),
			|ui| {
				if widgets::Button::new("").position(vec2(-1., -1.)).ui(ui) {
					action = Some(self.action_type);
				}
			},
		);
		root_ui().pop_skin();
		action
	}

	pub fn set_dot_creation(creating_entity: &mut Entities) {
		*creating_entity = Entities::Dot;
	}
}
