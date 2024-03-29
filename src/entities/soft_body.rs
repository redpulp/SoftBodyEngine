use super::dot::*;
use super::interaction::*;
use super::polygon::*;
use crate::utils::math::close_to_equal;
use crate::utils::math::runge_kutta_integration;
use macroquad::prelude::*;

const DAMPING_FACTOR: f32 = 0.8;
const STIFFNESS: f32 = 10.;

fn is_dot_on_border(dot: &Dot, corner1: &Vec2, corner2: &Vec2) -> bool {
    close_to_equal(dot.pos[0], corner1[0])
        || close_to_equal(dot.pos[1], corner1[1])
        || close_to_equal(dot.pos[0], corner2[0])
        || close_to_equal(dot.pos[1], corner2[1])
}

fn are_dots_on_border(dot1: &Dot, dot2: &Dot, corner1: &Vec2, corner2: &Vec2) -> bool {
    is_dot_on_border(dot1, corner1, corner2)
        && is_dot_on_border(dot2, corner1, corner2)
        && (close_to_equal(dot1.pos[0], dot2.pos[0]) || close_to_equal(dot1.pos[1], dot2.pos[1]))
}
#[derive(Copy, Clone)]
pub struct Spring {
    stiffness: f32,
    damping_factor: f32,
    rest_length: f32,
    pub index_1: usize,
    pub index_2: usize,
    pub is_on_border: bool,
}

impl Spring {
    pub fn new(
        dots: &[Dot],
        index_1: usize,
        index_2: usize,
        stiffness: f32,
        is_on_border: bool,
    ) -> Spring {
        Spring {
            index_1,
            index_2,
            rest_length: (dots[index_1].pos - dots[index_2].pos).length(),
            stiffness,
            damping_factor: DAMPING_FACTOR,
            is_on_border,
        }
    }

    pub fn draw(&self, dot1: &Dot, dot2: &Dot, color: Option<Color>) {
        draw_line(
            dot1.pos[0],
            dot1.pos[1],
            dot2.pos[0],
            dot2.pos[1],
            2.,
            color.unwrap_or(WHITE),
        );
    }

    fn get_hooke_force(&self, pos1: Vec2, pos2: Vec2) -> f32 {
        ((pos1 - pos2).length() - self.rest_length) * self.stiffness
    }

    fn get_damping(&self, pos1: Vec2, pos2: Vec2, vel1: Vec2, vel2: Vec2) -> f32 {
        (pos1 - pos2).normalize().dot(vel1 - vel2) * self.damping_factor
    }

    pub fn get_force(&self, pos1: Vec2, pos2: Vec2, vel1: Vec2, vel2: Vec2) -> Vec2 {
        (self.get_hooke_force(pos1, pos2) + self.get_damping(pos1, pos2, vel1, vel2))
            * ((pos1 - pos2).normalize())
    }
}

fn get_subdivisions(
    hor_distance: f32,
    ver_distance: f32,
    hor_sub: Option<i32>,
    ver_sub: Option<i32>,
) -> (i32, i32) {
    let temp_hor_sub = hor_sub.unwrap_or(1);
    let temp_ver_sub = ver_sub.unwrap_or(1);
    if (hor_distance / ver_distance).abs() < 1.1 && (hor_distance / ver_distance).abs() > 0.9 {
        (temp_hor_sub, temp_ver_sub)
    } else if hor_distance > ver_distance {
        get_subdivisions(
            (hor_distance * temp_hor_sub as f32) / (temp_hor_sub as f32 + 1.),
            ver_distance,
            Some(temp_hor_sub + 1),
            ver_sub,
        )
    } else {
        get_subdivisions(
            hor_distance,
            (ver_distance * temp_ver_sub as f32) / (temp_ver_sub as f32 + 1.),
            hor_sub,
            Some(temp_ver_sub + 1),
        )
    }
}

fn generate_dots(top_left_corner: Vec2, bottom_right_corner: Vec2) -> (Vec<Dot>, (f32, f32)) {
    // Get subdivisions
    let (horizontal_subdivisions, vertical_subdivisions) = get_subdivisions(
        bottom_right_corner[0] - top_left_corner[0],
        bottom_right_corner[1] - top_left_corner[1],
        None,
        None,
    );
    let mut dots: Vec<Dot> = vec![];
    let horizontal_step =
        (bottom_right_corner[0] - top_left_corner[0]) / (horizontal_subdivisions as f32);
    let vertical_step =
        (bottom_right_corner[1] - top_left_corner[1]) / (vertical_subdivisions as f32);
    for horizontal_pos in 0..horizontal_subdivisions + 1 {
        for vertical_pos in 0..vertical_subdivisions + 1 {
            dots.push(Dot::new(Some(
                top_left_corner
                    + vec2(
                        (horizontal_pos as f32) * horizontal_step,
                        (vertical_pos as f32) * vertical_step,
                    ),
            )));
        }
    }
    (dots, (horizontal_step, vertical_step))
}

// Generates the spring using dots proximity
fn generate_springs(
    dots: &[Dot],
    mut horizontal_distance: f32,
    mut vertical_distance: f32,
    (corner1, corner2): (&Vec2, &Vec2),
) -> Vec<Spring> {
    // Rounding position to account of f32 approximations
    fn round(number_to_round: f32) -> f32 {
        let accuracy = 1000.;
        (number_to_round * accuracy).round() / accuracy
    }
    horizontal_distance = round(horizontal_distance);
    vertical_distance = round(vertical_distance);

    // This is horrible
    dots.iter()
        .enumerate()
        .flat_map(|(index, dot)| {
            dots.iter()
                .enumerate()
                .map(|(inner_index, _)| inner_index)
                .filter(|&inner_index| {
                    let distance = (dots[inner_index].pos - dot.pos).abs();
                    let (distance_x, distance_y) = (round(distance[0]), round(distance[1]));
                    distance.length() > 0.
                        && ((close_to_equal(distance_x, 0.)
                            && close_to_equal(distance_y, vertical_distance))
                            || (close_to_equal(distance[1], 0.)
                                && close_to_equal(distance_x, horizontal_distance))
                            || (close_to_equal(distance_x, horizontal_distance)
                                && close_to_equal(distance_y, vertical_distance)))
                })
                .map(|inner_index| {
                    Spring::new(
                        dots,
                        index,
                        inner_index,
                        STIFFNESS
                            + if are_dots_on_border(
                                &dots[index],
                                &dots[inner_index],
                                corner1,
                                corner2,
                            ) {
                                2.
                            } else {
                                0.
                            },
                        are_dots_on_border(&dots[index], &dots[inner_index], corner1, corner2),
                    )
                })
                .collect::<Vec<Spring>>()
        })
        .collect::<Vec<Spring>>()
}

pub struct SoftBody {
    pub points: Vec<Dot>,
    pub springs: Vec<Spring>,
}

impl SoftBody {
    pub fn new(pos1: f32, pos2: f32) -> SoftBody {
        let (corner1, corner2) = (vec2(pos1 - 80., pos2 - 100.), vec2(pos1 + 80., pos2 + 20.));
        let (dots, (horizontal_step, vertical_step)) = generate_dots(corner1, corner2);
        SoftBody {
            springs: generate_springs(
                &dots,
                horizontal_step.abs(),
                vertical_step.abs(),
                (&corner1, &corner2),
            ),
            points: dots,
        }
    }

    pub fn draw(&self) {
        self.points.iter().for_each(|point| point.draw());
        self.springs.iter().for_each(|spring| {
            spring.draw(
                &self.points[spring.index_1],
                &self.points[spring.index_2],
                None,
            )
        });
    }

    pub fn draw_border(&self) {
        self.springs.iter().for_each(|spring| {
            if spring.is_on_border {
                spring.draw(
                    &self.points[spring.index_1],
                    &self.points[spring.index_2],
                    Some(YELLOW),
                )
            }
        });
    }

    pub fn update_runge_kutta(&mut self) {
        self.springs.clone().iter().for_each(|spring| {
            if self.points[spring.index_1].pos != self.points[spring.index_2].pos {
                self.update_masses_acceleration(spring);
            }
        });
        self.points.iter_mut().for_each(|point| point.update());
        self.springs.iter().for_each(|spring| {
            handle_point_point_collision(&mut self.points, spring.index_1, spring.index_2);
        });
    }

    fn update_masses_acceleration(&mut self, spring: &Spring) {
        let get_acceleration = move |position_1: Vec2,
                                     position_2: Vec2,
                                     velocity_1: Vec2,
                                     velocity_2: Vec2|
              -> (Vec2, Vec2) {
            let spring_force = spring.get_force(position_1, position_2, velocity_1, velocity_2);

            (
                -spring_force + vec2(0., 0.68),
                spring_force + vec2(0., 0.68),
            )
        };

        let point1 = self.points[spring.index_1];
        let point2 = self.points[spring.index_2];

        let (push_vec_1, push_vec_2) = runge_kutta_integration(
            &get_acceleration,
            point1.pos,
            point2.pos,
            point1.vel,
            point2.vel,
        );

        self.points[spring.index_1].add_acceleration(push_vec_1);
        self.points[spring.index_2].add_acceleration(push_vec_2);
    }

    pub fn handle_collision(&mut self, polygon: &Polygon) {
        self.points
            .iter_mut()
            .for_each(|point| handle_point_polygon_collision(point, polygon));
    }
}
