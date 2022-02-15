use super::dot::*;
use super::polygon::*;

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
