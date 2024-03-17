use super::super::entities::segment::*;
use macroquad::prelude::vec2;

fn negative_difference(val1: f32, val2: f32) -> bool {
    val1 - val2 < 0.
}

fn all_equal(conditions: Vec<bool>) -> bool {
    if conditions.len() > 0 {
        !conditions
            .iter()
            .any(|condition| *condition != conditions[0])
    } else {
        false
    }
}

pub fn do_segments_intersect(segment1: &Segment, segment2: &Segment) -> bool {
    let common_point = vec2(segment2.x1 - segment1.x1, segment2.y1 - segment1.y1);
    let origin_segment_1 = vec2(segment1.x2 - segment1.x1, segment1.y2 - segment1.y1);
    let origin_segment_2 = vec2(segment2.x2 - segment2.x1, segment2.y2 - segment2.y1);

    let numerator = (common_point.x * origin_segment_1.y) - (common_point.y * origin_segment_1.x);
    let denominator =
        (origin_segment_1.x * origin_segment_2.y) - (origin_segment_1.y * origin_segment_2.x);

    if numerator == 0. && denominator == 0. {
        return !all_equal(
            [
                negative_difference(segment1.x1, segment2.x1),
                negative_difference(segment1.x1, segment2.x2),
                negative_difference(segment1.x2, segment2.x1),
                negative_difference(segment1.x2, segment2.x2),
            ]
            .to_vec(),
        ) || !all_equal(
            [
                negative_difference(segment1.y1, segment2.y1),
                negative_difference(segment1.y1, segment2.y2),
                negative_difference(segment1.y2, segment2.y1),
                negative_difference(segment1.y2, segment2.y2),
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
    return val1 >= 0. && val1 <= 1. && val2 >= 0. && val2 <= 1.;
}
