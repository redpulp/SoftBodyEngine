use super::super::entities::segment::*;
use macroquad::prelude::Vec2;

pub fn coords_to_segment((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> Segment {
    Segment { x1, y1, x2, y2 }
}

pub fn vec2_to_segments(point1: Vec2, point2: Vec2) -> Segment {
    coords_to_segment((point1[0], point1[1]), (point2[0], point2[1]))
}
