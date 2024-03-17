use super::super::utils::math::*;
use macroquad::prelude::*;

#[derive(Clone, Debug)]
pub struct Segment {
    pub p1: Vec2,
    pub p2: Vec2,
}

impl Segment {
    fn slope(&self) -> f32 {
        (self.p2.y - self.p1.y) / (self.p2.x - self.p1.x)
    }

    fn intercept(&self) -> f32 {
        self.p1.y - (self.slope() * self.p1.x)
    }

    fn half_point(&self) -> Vec2 {
        vec2(self.p2.x - self.p1.x, self.p2.y - self.p1.y)
    }

    // Get the projection of a point on the segment
    pub fn get_projection(&self, point: &Vec2) -> Option<Vec2> {
        let e1 = self.half_point();
        let e2 = point_diff(&self.p1, &point);
        let rect_area = e1.dot(e1);
        let value = e1.dot(e2);
        if value >= 0. && value <= rect_area {
            // Solving for vertical and horizontal segments
            if self.p1.x == self.p2.x {
                return Some(vec2(self.p1.x, point[1]));
            }
            if self.p1.y == self.p2.y {
                return Some(vec2(point[0], self.p1.y));
            }

            let slope = self.slope();

            // Solving for diagonal segments
            let perpendicular_intercept = point[1] + (point[0] / slope);
            let x_intersection =
                (slope * (perpendicular_intercept - self.intercept())) / (1. + (slope * slope));
            Some(vec2(
                x_intersection,
                (slope * x_intersection) + self.intercept(),
            ))
        } else {
            None
        }
    }

    // Cast a vertical ray from the given point
    // Returns true if the ray passes through the segment
    pub fn vertical_raycast(&self, point: &Vec2) -> bool {
        let Segment { p1, p2 } = self;
        let x_range = std::ops::Range {
            start: p1.x.min(p2.x),
            end: p1.x.max(p2.x),
        };

        x_range.contains(&point[0])
            && (if p1.x == p2.x {
                p1.y > point[1]
            } else {
                (((self.slope()) * &point[0]) + p1.y - ((self.slope()) * p1.x)) > point[1]
            })
    }
}
