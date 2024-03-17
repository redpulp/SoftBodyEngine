use super::dot::*;
use super::polygon::*;
use macroquad::prelude::*;

pub fn handle_point_polygon_collision(point: &mut Dot, polygon: &Polygon) {
    match point.get_push_vector(polygon) {
        None => (),
        Some(vector) => {
            point.push(&vector);
        }
    }
}

pub fn handle_point_point_collision(points: &mut Vec<Dot>, index_1: usize, index_2: usize) {
    let distance = points[index_1].pos - points[index_2].pos;
    let radius = points[index_1].radius;
    if distance.length() < points[index_1].radius && distance.length() > 0. {
        points[index_1].push(&(distance.normalize() * radius));
        points[index_2].push(&(-distance.normalize() * radius));
    }
}

pub fn handle_temp_point_point_collision(pos1: Vec2, pos2: Vec2) -> Option<Vec2> {
    let distance = pos1 - pos2;
    if distance.length() < RADIUS && distance.length() > 0. {
        let push = distance.normalize() * RADIUS;
        Some(push)
    } else {
        None
    }
}
