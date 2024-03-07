use crate::entities::segment::*;
use macroquad::prelude::{vec2, Vec2};

pub fn point_diff(point1: &Vec2, point2: &Vec2) -> Vec2 {
    vec2(point2.x - point1.x, point2.y - point1.y)
}

fn negative_diff(val1: f32, val2: f32) -> bool {
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
        return !all_equal(vec![
            negative_diff(segment1.p1.x, segment2.p1.x),
            negative_diff(segment1.p1.x, segment2.p2.x),
            negative_diff(segment1.p2.x, segment2.p1.x),
            negative_diff(segment1.p2.x, segment2.p2.x),
        ]) || !all_equal(vec![
            negative_diff(segment1.p1.y, segment2.p1.y),
            negative_diff(segment1.p1.y, segment2.p2.y),
            negative_diff(segment1.p2.y, segment2.p1.y),
            negative_diff(segment1.p2.y, segment2.p2.y),
        ]);
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

pub const DELTA_T_RUNGE_KUTTA: f32 = 0.25;
pub fn runge_kutta_integration(
    motion_func: &(dyn Fn(Vec2, Vec2, Vec2, Vec2) -> (Vec2, Vec2)),
    pos_1: Vec2,
    pos_2: Vec2,
    vel_1: Vec2,
    vel_2: Vec2,
) -> (Vec2, Vec2) {
    // Slope 1
    let (f1_1, f1_2) = motion_func(pos_1, pos_2, vel_1, vel_2);
    let (k1_1, k1_2) = (f1_1 * DELTA_T_RUNGE_KUTTA, f1_2 * DELTA_T_RUNGE_KUTTA);

    // Slope 2
    let k1_1_halved = k1_1 / 2.;
    let k1_2_halved = k1_2 / 2.;
    let (f2_1, f2_2) = motion_func(
        pos_1 + (k1_1_halved * DELTA_T_RUNGE_KUTTA),
        pos_2 + (k1_2_halved * DELTA_T_RUNGE_KUTTA),
        vel_1 + k1_1_halved,
        vel_2 + k1_2_halved,
    );

    let (k2_1, k2_2) = (f2_1 * DELTA_T_RUNGE_KUTTA, f2_2 * DELTA_T_RUNGE_KUTTA);

    // Slope 3
    let k2_1_halved = k2_1 / 2.;
    let k2_2_halved = k2_2 / 2.;
    let (f3_1, f3_2) = motion_func(
        pos_1 + (k2_1_halved * DELTA_T_RUNGE_KUTTA),
        pos_2 + (k2_2_halved * DELTA_T_RUNGE_KUTTA),
        vel_1 + k2_1_halved,
        vel_2 + k2_2_halved,
    );
    let (k3_1, k3_2) = (f3_1 * DELTA_T_RUNGE_KUTTA, f3_2 * DELTA_T_RUNGE_KUTTA);

    // Slope 4
    let (f4_1, f4_2) = motion_func(
        pos_1 + (k3_1 * DELTA_T_RUNGE_KUTTA),
        pos_2 + (k3_2 * DELTA_T_RUNGE_KUTTA),
        vel_1 + k3_1,
        vel_2 + k3_2,
    );
    let (k4_1, k4_2) = (f4_1 * DELTA_T_RUNGE_KUTTA, f4_2 * DELTA_T_RUNGE_KUTTA);
    let push_vec_1 = (1. / 6.) * (k1_1 + (2. * k2_1) + (2. * k3_1) + k4_1);
    let push_vec_2 = (1. / 6.) * (k1_2 + (2. * k2_2) + (2. * k3_2) + k4_2);

    (push_vec_1, push_vec_2)
}
