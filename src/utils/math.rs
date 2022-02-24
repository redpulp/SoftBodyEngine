use super::super::entities::segment::*;
use macroquad::prelude::{vec2, Vec2};

pub fn point_diff(point1: &Vec2, point2: &Vec2) -> Vec2 {
    vec2(point2.x - point1.x, point2.y - point1.y)
}

fn negative_difference(val1: f32, val2: f32) -> bool {
    val1 - val2 < 0.
}

fn all_equal(conditions: Vec<bool>) -> bool {
    if !conditions.is_empty() {
        !conditions
            .iter()
            .any(|condition| *condition != conditions[0])
    } else {
        false
    }
}

pub fn do_segments_intersect(segment1: &Segment, segment2: &Segment) -> bool {
    let common_point = point_diff(&segment1.p1, &segment2.p1);
    let origin_segment_1 = point_diff(&segment1.p1, &segment1.p2);
    let origin_segment_2 = point_diff(&segment2.p1, &segment2.p2);

    let numerator = (common_point.x * origin_segment_1.y) - (common_point.y * origin_segment_1.x);
    let denominator =
        (origin_segment_1.x * origin_segment_2.y) - (origin_segment_1.y * origin_segment_2.x);

    if numerator == 0. && denominator == 0. {
        return !all_equal(
            [
                negative_difference(segment1.p1.x, segment2.p1.x),
                negative_difference(segment1.p1.x, segment2.p2.x),
                negative_difference(segment1.p2.x, segment2.p1.x),
                negative_difference(segment1.p2.x, segment2.p2.x),
            ]
            .to_vec(),
        ) || !all_equal(
            [
                negative_difference(segment1.p1.y, segment2.p1.y),
                negative_difference(segment1.p1.y, segment2.p2.y),
                negative_difference(segment1.p2.y, segment2.p1.y),
                negative_difference(segment1.p2.y, segment2.p2.y),
            ]
            .to_vec(),
        );
    }
    if denominator == 0. {
        return false;
    }
    let val1 = numerator / denominator;
    let val2 = ((common_point.x * origin_segment_2.y) - (common_point.y * origin_segment_2.x))
        / denominator;
    (0. ..=1.).contains(&val1) && (0. ..=1.).contains(&val2)
}

pub fn close_to_equal(num1: f32, num2: f32) -> bool {
    (num1 - num2).abs() < 0.001
}
