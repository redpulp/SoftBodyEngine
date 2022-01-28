use macroquad::prelude::*;

pub struct Segment {
	pub x1: f32,
	pub y1: f32,
	pub x2: f32,
	pub y2: f32,
}

impl Segment {
	fn slope(&self) -> f32 {
		(self.y2 - self.y1) / (self.x2 - self.x1)
	}

	fn intercept(&self) -> f32 {
		self.y1 - (self.slope() * self.x1)
	}

	fn half_segment(&self) -> Vec2 {
		vec2(self.x2 - self.x1, self.y2 - self.y1)
	}

	// Get the projection of a point on the segment
	// The parameter `force` will indicate to also get
	// the closest extremity to the point if no projection exists
	pub fn get_projection(&self, point: &Vec2, force: bool) -> Option<Vec2> {
		let e1 = self.half_segment();
		let e2 = vec2(point[0] - self.x1, point[1] - self.y1);
		let rect_area = e1.dot(e1);
		let value = e1.dot(e2);
		if value > 0. && value < rect_area {
			let slope = self.slope();

			// Solving for vertical and horizontal segments
			if slope.abs() == f32::INFINITY {
				return Some(vec2(self.x1, point[1]));
			}
			if slope == 0. {
				return Some(vec2(point[0], self.y1));
			}

			// Solving for diagonal segments
			let perpendicular_intercept = point[1] + (point[0] / slope);
			let x_intersection =
				(slope * (perpendicular_intercept - self.intercept())) / (1. + slope * slope);
			Some(vec2(
				x_intersection,
				(slope * x_intersection) + self.intercept(),
			))
		} else {
			if force {
				return Some(self.get_extremity_projection(point));
			} else {
				return None;
			}
		}
	}

	pub fn get_extremity_projection(&self, point: &Vec2) -> Vec2 {
		let point1 = vec2(self.x1, self.y1);
		let point2 = vec2(self.x2, self.y2);
		if (point1 - point.clone()).length() < (point2 - point.clone()).length() {
			point1
		} else {
			point2
		}
	}

	pub fn vertical_raycast(&self, point: &Vec2) -> bool {
		let Segment { x1, y1, x2, y2: _ } = self;
		let x_range = std::ops::Range {
			start: x1.min(*x2),
			end: x1.max(*x2),
		};

		x_range.contains(&point[0])
			&& (if x1 == x2 {
				y1 > &point[1]
			} else {
				(((self.slope()) * &point[0]) + y1 - ((self.slope()) * x1)) > point[1]
			})
	}
}
