use super::super::entities::segment::*;
use macroquad::prelude::vec2;

pub fn coords_to_segment((x1, y1): (f32, f32), (x2, y2): (f32, f32)) -> Segment {
    Segment {
        p1: vec2(x1, y1),
        p2: vec2(x2, y2),
    }
}
