use super::segment::*;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Polygon {
	points: Vec<(f32, f32)>,
	color: Color,
}

impl Polygon {
	pub fn new(points: Vec<(f32, f32)>, color: Option<Color>) -> Polygon {
		Polygon {
			points,
			color: color.unwrap_or(BLUE),
		}
	}

	pub fn generate_floor() -> Polygon {
		Polygon::new(
			[
				(20., screen_height() - 30.),
				(screen_width() - 20., screen_height() - 30.),
				(screen_width() - 20., screen_height() - 20.),
				(20., screen_height() - 20.),
			]
			.to_vec(),
			None,
		)
	}

	pub fn draw(&self) {
		self.segments().iter().for_each(|segment| {
			draw_line(
				segment.x1, segment.y1, segment.x2, segment.y2, 2., self.color,
			);
		});
	}

	pub fn segments(&self) -> Vec<Segment> {
		self
			.points
			.iter()
			.enumerate()
			.map(|(i, point)| {
				let point1 = point;
				let point2 = self.points[if i == self.points.len() - 1 { 0 } else { i + 1 }];
				Segment {
					x1: point1.0,
					y1: point1.1,
					x2: point2.0,
					y2: point2.1,
				}
			})
			.collect()
	}

	pub fn bounding_box(&self) -> (f32, f32, f32, f32) {
		let (mut min_hor, mut max_hor, mut min_ver, mut max_ver): (f32, f32, f32, f32) = (
			self.points[0].0,
			self.points[0].0,
			self.points[0].1,
			self.points[0].1,
		);

		for point in &self.points {
			min_hor = if min_hor < point.0 { min_hor } else { point.0 };
			max_hor = if max_hor > point.0 { max_hor } else { point.0 };
			min_ver = if min_ver < point.1 { min_ver } else { point.1 };
			max_ver = if max_ver > point.1 { max_ver } else { point.1 };
		}

		(min_hor, max_hor, min_ver, max_ver)
	}
}
