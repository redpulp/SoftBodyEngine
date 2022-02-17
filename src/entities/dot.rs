use super::polygon::*;
use macroquad::prelude::*;

const DELTA_T: f32 = 0.1;
const RADIUS: f32 = 5.;
const MASS: f32 = 1.;

#[derive(Copy, Clone, Debug)]
pub struct Dot {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    pub mass: f32,
    prev_pos: Vec2,
    acceleration: Vec2,
    freeze: bool,
}

impl Dot {
    pub fn new(pos: Option<Vec2>, mass: Option<f32>) -> Dot {
        let initial_position = pos.unwrap_or(vec2(screen_width() / 2., screen_height() / 2.));
        Dot {
            pos: initial_position,
            prev_pos: initial_position,
            mass: mass.unwrap_or(MASS),
            radius: RADIUS,
            vel: vec2(0., 0.),
            acceleration: vec2(0., 0.),
            freeze: false,
        }
    }
    pub fn is_out_of_bounds(&self) -> bool {
        (self.pos[0]).abs() > screen_width() * 2. || (self.pos[1]).abs() > screen_height() * 2.
    }

    fn add_gravity(&mut self) {
        self.acceleration += vec2(0., 9.8);
    }

    pub fn update(&mut self) {
        self.add_gravity();
        self.prev_pos = self.pos;
        if !self.freeze {
            self.vel += self.acceleration * DELTA_T;
            self.vel = if self.vel.length() < 200. {
                self.vel
            } else {
                self.vel.normalize() * 10.
            };
            self.pos += self.vel * DELTA_T;
        }
        self.acceleration = vec2(0., 0.);
    }

    pub fn add_acceleration(&mut self, acceleration: Vec2) {
        self.acceleration += acceleration;
    }

    pub fn push(&mut self, push_vec: &Vec2) {
        // self.acceleration += *push_vec;
        if !self.freeze {
            self.vel += *push_vec;
            self.pos += *push_vec;
        }
    }

    pub fn draw(&self) {
        draw_circle(self.pos[0], self.pos[1], self.radius, YELLOW);
    }

    fn is_in_bounding_box(&self, polygon: &Polygon) -> bool {
        let BoundingBox {
            min_hor,
            max_hor,
            min_ver,
            max_ver,
        } = polygon.bounding_box;
        (self.pos[0] - self.radius) < max_hor
            && (self.pos[0] + self.radius) > min_hor
            && (self.pos[1] - self.radius) < max_ver
            && (self.pos[1] + self.radius) > min_ver
    }

    pub fn is_center_inside_polygon(&self, polygon: &Polygon) -> bool {
        let counter = polygon
            .segments()
            .iter()
            .fold(0, |acc, cur| acc + cur.vertical_raycast(&self.pos) as i32);

        counter % 2 != 0
    }

    // Gets the closest projection of the Dot on any segment of a Polygon
    fn get_closest_projection(&self, polygon: &Polygon) -> Vec2 {
        let projections = polygon
            .segments()
            .iter()
            .map(|segment| segment.get_projection(&self.pos))
            .filter(|projection| projection.is_some())
            .map(|projection| projection.unwrap() - self.pos)
            .collect::<Vec<Vec2>>();

        projections.iter().fold(
            vec2(f32::INFINITY, f32::INFINITY),
            |distance1, &distance2| {
                if distance1.length() < distance2.length() {
                    distance1
                } else {
                    distance2
                }
            },
        )
    }

    // Calculates push vector to move Dot out of a Polygon
    // If the Polygon doesn't intersects the Dot, None is returned
    pub fn get_push_vector(&self, polygon: &Polygon) -> Option<Vec2> {
        if !self.is_in_bounding_box(polygon) {
            return None;
        }

        let closest_projection = self.get_closest_projection(polygon);
        let radius_projection = closest_projection.normalize() * self.radius;

        let is_center_inside = self.is_center_inside_polygon(polygon);
        let is_partially_inside = closest_projection.length() < self.radius;

        if is_center_inside {
            return Some(closest_projection + radius_projection);
        }

        if is_partially_inside {
            return Some(closest_projection - radius_projection);
        }

        None
    }

    pub fn handle_collision(&mut self, polygon: &Polygon) {
        match self.get_push_vector(polygon) {
            None => (),
            Some(vector) => {
                self.push(&vector);
            }
        }
    }

    pub fn handle_point_collision(&mut self, collision_point: &Dot) {
        let distance = self.pos - collision_point.pos;
        if distance.length() < self.radius * 2.5 && distance.length() > 0. {
            self.push(&(distance.normalize() * self.radius));
        }
    }
}
