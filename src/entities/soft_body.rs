use super::dot::*;
use super::interaction::*;
use super::polygon::*;
use macroquad::prelude::*;

const DAMPING_FACTOR: f32 = 0.8;
const RIGIDITY: f32 = 5.;

fn is_dot_on_border(dot: &Dot, corner1: &Vec2, corner2: &Vec2) -> bool {
    dot.pos[0] == corner1[0]
        || dot.pos[1] == corner1[1]
        || dot.pos[0] == corner2[0]
        || dot.pos[1] == corner2[1]
}

fn are_dots_on_border(dot1: &Dot, dot2: &Dot, corner1: &Vec2, corner2: &Vec2) -> bool {
    is_dot_on_border(dot1, corner1, corner2)
        && is_dot_on_border(dot2, corner1, corner2)
        && (dot1.pos[0] == dot2.pos[0] || dot1.pos[1] == dot2.pos[1])
}
#[derive(Copy, Clone)]
pub struct Spring {
    stiffness: f32,
    damping_factor: f32,
    rest_length: f32,
    index_1: usize,
    index_2: usize,
    is_on_border: bool,
}

impl Spring {
    pub fn new(
        dots: &Vec<Dot>,
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
        return (temp_hor_sub, temp_ver_sub);
    } else {
        if hor_distance > ver_distance {
            return get_subdivisions(
                (hor_distance * temp_hor_sub as f32) / (temp_hor_sub as f32 + 1.),
                ver_distance,
                Some(temp_hor_sub + 1),
                ver_sub,
            );
        } else {
            return get_subdivisions(
                hor_distance,
                (ver_distance * temp_ver_sub as f32) / (temp_ver_sub as f32 + 1.),
                hor_sub,
                Some(temp_ver_sub + 1),
            );
        }
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
    let mut dots: Vec<Dot> = [].to_vec();
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
    dots: &Vec<Dot>,
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
    dots.clone()
        .iter()
        .enumerate()
        .flat_map(|(index, dot)| {
            dots.clone()
                .iter()
                .enumerate()
                .map(|(inner_index, _)| inner_index)
                .filter(|&inner_index| {
                    let distance = (dots[inner_index].pos - dot.pos).abs();
                    let (distance_x, distance_y) = (round(distance[0]), round(distance[1]));
                    distance.length() > 0.
                        && ((distance_x == 0. && distance_y == vertical_distance)
                            || (distance[1] == 0. && distance_x == horizontal_distance)
                            || (distance_x == horizontal_distance
                                && distance_y == vertical_distance))
                })
                .map(|inner_index| {
                    Spring::new(
                        &dots,
                        index,
                        inner_index,
                        RIGIDITY
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
    points: Vec<Dot>,
    springs: Vec<Spring>,
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

    pub fn update_euler(&mut self) {
        self.springs.iter().for_each(|spring| {
            if self.points[spring.index_1].pos != self.points[spring.index_2].pos {
                let spring_acceleration = spring.get_force(
                    self.points[spring.index_1].pos,
                    self.points[spring.index_2].pos,
                    self.points[spring.index_1].vel,
                    self.points[spring.index_2].vel,
                );

                let points = &mut self.points;

                points[spring.index_1].add_acceleration(-spring_acceleration);
                points[spring.index_2].add_acceleration(spring_acceleration);
            }
        });

        self.points.iter_mut().for_each(|point| point.add_gravity());
        self.points.iter_mut().for_each(|point| point.update(false));
    }

    pub fn update_runge_kutta(&mut self) {
        self.springs.clone().iter().for_each(|spring| {
            if self.points[spring.index_1].pos != self.points[spring.index_2].pos {
                self.update_spring_runge_kutta(&spring);
            }
        });
        self.points.iter_mut().for_each(|point| point.update(true));
        self.springs.iter().for_each(|spring| {
            handle_point_point_collision(&mut self.points, spring.index_1, spring.index_2);
        });
    }

    fn update_spring_runge_kutta(&mut self, spring: &Spring) {
        let get_acceleration = |position_1: Vec2,
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

        let mut point1 = self.points[spring.index_1];
        let mut point2 = self.points[spring.index_2];

        let (k1_1, k1_2) = get_acceleration(point1.pos, point2.pos, point1.vel, point2.vel);
        let update = handle_temp_point_point_collision(point1.pos, point2.pos);
        if update.is_some() {
            point1.pos = update.unwrap();
            point2.pos = -update.unwrap();
        }
        let k1_1_halved = k1_1 / 2.;
        let k1_2_halved = k1_2 / 2.;
        let (k2_1, k2_2) = get_acceleration(
            point1.pos + (k1_1_halved * DELTA_T_RUNGE_KUTTA),
            point2.pos + (k1_2_halved * DELTA_T_RUNGE_KUTTA),
            point1.vel + k1_1_halved,
            point2.vel + k1_2_halved,
        );
        let k2_1_halved = k2_1 / 2.;
        let k2_2_halved = k2_2 / 2.;
        let (k3_1, k3_2) = get_acceleration(
            point1.pos + (k2_1_halved * DELTA_T_RUNGE_KUTTA),
            point2.pos + (k2_2_halved * DELTA_T_RUNGE_KUTTA),
            point1.vel + k2_1_halved,
            point2.vel + k2_2_halved,
        );

        let push_vec_1 = (DELTA_T_RUNGE_KUTTA / 6.) * (k1_1 + (0.2 * k2_1) + (0.2 * k3_1));
        let push_vec_2 = (DELTA_T_RUNGE_KUTTA / 6.) * (k1_2 + (0.2 * k2_2) + (0.2 * k3_2));
        self.points[spring.index_1].add_acceleration(push_vec_1);
        self.points[spring.index_2].add_acceleration(push_vec_2);
    }

    pub fn handle_collision(&mut self, polygon: &Polygon) {
        self.points
            .iter_mut()
            .for_each(|point| handle_point_polygon_collision(point, polygon));
    }
}
