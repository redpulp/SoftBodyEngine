use super::segment::*;
use crate::utils::conversion::*;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub min_hor: f32,
    pub max_hor: f32,
    pub min_ver: f32,
    pub max_ver: f32,
}

impl BoundingBox {
    pub fn new(points: &[(f32, f32)]) -> Self {
        let (mut min_hor, mut max_hor, mut min_ver, mut max_ver): (f32, f32, f32, f32) =
            (points[0].0, points[0].0, points[0].1, points[0].1);
        for point in points {
            min_hor = if min_hor < point.0 { min_hor } else { point.0 };
            max_hor = if max_hor > point.0 { max_hor } else { point.0 };
            min_ver = if min_ver < point.1 { min_ver } else { point.1 };
            max_ver = if max_ver > point.1 { max_ver } else { point.1 };
        }
        Self {
            min_hor,
            max_hor,
            min_ver,
            max_ver,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Polygon {
    points: Vec<(f32, f32)>,
    color: Color,
    pub bounding_box: BoundingBox,
}

impl Polygon {
    pub fn new(points: Vec<(f32, f32)>, color: Option<Color>) -> Polygon {
        Polygon {
            bounding_box: BoundingBox::new(&points),
            points,
            color: color.unwrap_or(BLUE),
        }
    }

    pub fn generate_floor() -> Polygon {
        Polygon::new(
            vec![
                (20., screen_height() - 50.),
                (screen_width() - 20., screen_height() - 50.),
                (screen_width() - 20., screen_height() - 20.),
                (20., screen_height() - 20.),
            ],
            None,
        )
    }

    pub fn draw(&self) {
        self.segments().iter().for_each(|segment| {
            draw_line(
                segment.p1.x,
                segment.p1.y,
                segment.p2.x,
                segment.p2.y,
                2.,
                self.color,
            );
        });
    }

    pub fn segments(&self) -> Vec<Segment> {
        self.points
            .iter()
            .enumerate()
            .map(|(i, p1)| {
                let p2 = self.points[if i == self.points.len() - 1 { 0 } else { i + 1 }];
                coords_to_segment(*p1, p2)
            })
            .collect()
    }
}
