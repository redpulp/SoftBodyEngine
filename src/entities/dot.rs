use super::polygon::*;
use macroquad::prelude::*;

pub const DELTA_T_RUNGE_KUTTA: f32 = 0.25;
pub const RADIUS: f32 = 10.;

#[derive(Copy, Clone, Debug)]
pub struct Dot {
    pub pos: Vec2,
    pub vel: Vec2,
    pub radius: f32,
    prev_pos: Vec2,
    pub acceleration: Vec2,
    freeze: bool,
}

impl Dot {
    pub fn new(pos: Option<Vec2>) -> Dot {
        let initial_position =
            pos.unwrap_or_else(|| vec2(screen_width() / 2., screen_height() / 2.));
        Dot {
            pos: initial_position,
            prev_pos: initial_position,
            radius: RADIUS,
            vel: vec2(0., 0.),
            acceleration: vec2(0., 0.),
            freeze: false,
        }
    }
    pub fn is_out_of_bounds(&self) -> bool {
        (self.pos[0]).abs() > screen_width() * 2. || (self.pos[1]).abs() > screen_height() * 2.
    }

    pub fn update(&mut self) {
        self.prev_pos = self.pos;
        if !self.freeze {
            self.vel += self.acceleration * DELTA_T_RUNGE_KUTTA;
            self.vel = if self.vel.length() < 100. {
                self.vel
            } else {
                self.vel.normalize() * 10.
            };
            self.pos += self.vel * DELTA_T_RUNGE_KUTTA;
        }
        self.acceleration = vec2(0., 0.);
    }

    pub fn update_runge(&mut self, push_vec: &Vec2) {
        self.pos += *push_vec;
        self.vel = (self.pos - self.prev_pos) / DELTA_T_RUNGE_KUTTA;
    }

    pub fn add_acceleration(&mut self, acceleration: Vec2) {
        self.acceleration += acceleration;
    }

    pub fn add_gravity(&mut self) {
        self.acceleration += vec2(0., 9.8);
    }

    pub fn push(&mut self, push_vec: &Vec2) {
        if self.freeze {
            return;
        }

        self.acceleration += *push_vec * 10.;
        self.pos += *push_vec;
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
}
