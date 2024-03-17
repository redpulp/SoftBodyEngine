use super::dot::*;
use super::interaction::*;
use super::polygon::*;
use macroquad::prelude::*;

const DAMPING_FACTOR: f32 = 0.5;
const RIGIDITY: f32 = 8.;

pub struct Spring {
    stiffness: f32,
    damping_factor: f32,
    rest_length: f32,
    index_1: usize,
    index_2: usize,
}

impl Spring {
    pub fn new(dots: &Vec<Dot>, index_1: usize, index_2: usize, stiffness: f32) -> Spring {
        Spring {
            index_1,
            index_2,
            rest_length: (dots[index_1].pos - dots[index_2].pos).length(),
            stiffness,
            damping_factor: DAMPING_FACTOR,
        }
    }

    pub fn draw(&self, dot1: &Dot, dot2: &Dot) {
        draw_line(
            dot1.pos[0],
            dot1.pos[1],
            dot2.pos[0],
            dot2.pos[1],
            2.,
            WHITE,
        );
    }

    fn get_hooke_force(&self, dot1: &Dot, dot2: &Dot) -> f32 {
        ((dot1.pos - dot2.pos).length() - self.rest_length) * self.stiffness
    }

    fn get_damping(&self, dot1: &Dot, dot2: &Dot) -> f32 {
        (dot1.pos - dot2.pos).normalize().dot(dot1.vel - dot2.vel) * self.damping_factor
    }

    pub fn get_force(&self, dot1: &Dot, dot2: &Dot) -> Vec2 {
        (self.get_hooke_force(dot1, dot2) + self.get_damping(dot1, dot2))
            * ((dot1.pos - dot2.pos).normalize())
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
            dots.push(Dot::new(
                Some(
                    top_left_corner
                        + vec2(
                            (horizontal_pos as f32) * horizontal_step,
                            (vertical_pos as f32) * vertical_step,
                        ),
                ),
                None,
            ));
        }
    }
    (dots, (horizontal_step, vertical_step))
}

// Generates the spring using dots proximity
fn generate_springs(
    dots: &Vec<Dot>,
    mut horizontal_distance: f32,
    mut vertical_distance: f32,
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
                .map(|inner_index| Spring::new(&dots, index, inner_index, RIGIDITY))
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
        let (dots, (horizontal_step, vertical_step)) =
            generate_dots(vec2(pos1 - 80., pos2 - 100.), vec2(pos1 + 80., pos2 + 20.));
        SoftBody {
            springs: generate_springs(&dots, horizontal_step.abs(), vertical_step.abs()),
            points: dots,
        }
    }

    pub fn draw(&self) {
        self.points.iter().for_each(|point| point.draw());
        self.springs.iter().for_each(|spring| {
            spring.draw(&self.points[spring.index_1], &self.points[spring.index_2])
        });
    }

    pub fn update(&mut self) {
        self.springs.iter().for_each(|spring| {
            if self.points[spring.index_1].pos != self.points[spring.index_2].pos {
                let spring_acceleration = spring
                    .get_force(&self.points[spring.index_1], &self.points[spring.index_2])
                    / self.points[spring.index_1].mass;

                let points = &mut self.points;

                points[spring.index_1].add_acceleration(-spring_acceleration);
                points[spring.index_2].add_acceleration(spring_acceleration);
            }
        });

        self.points.iter_mut().for_each(|point| point.update());

        self.springs.iter().for_each(|spring| {
            handle_point_point_collision(&mut self.points, spring.index_1, spring.index_2);
        })
    }

    pub fn handle_collision(&mut self, polygon: &Polygon) {
        self.points
            .iter_mut()
            .for_each(|point| point.handle_collision(polygon));
    }
}
