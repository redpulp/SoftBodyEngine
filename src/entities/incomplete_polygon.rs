use super::super::utils::conversion::*;
use super::super::utils::math::*;
use super::polygon::*;
use macroquad::prelude::*;

const STD_COLOR: Color = WHITE;
const ERROR_COLOR: Color = RED;
const OK_COLOR: Color = GRAY;

pub struct IncompletePolygon {
    pub points: Vec<Vec2>,
}

impl IncompletePolygon {
    pub fn new() -> Self {
        Self {
            points: [].to_vec(),
        }
    }

    pub fn draw(&self, polygons: &Vec<Polygon>) {
        self.points.iter().enumerate().for_each(|(i, point)| {
            let is_last_segment = i == self.points.len() - 1;
            let is_on_end = self.is_on_end();
            let mouse_position = vec2(mouse_position().0, mouse_position().1);

            let ending_point = if !is_last_segment {
                self.points[i + 1]
            } else {
                if is_on_end {
                    self.points[0]
                } else {
                    mouse_position
                }
            };

            if !is_last_segment {
                draw_line(
                    point[0],
                    point[1],
                    ending_point[0],
                    ending_point[1],
                    2.,
                    STD_COLOR,
                );
            } else {
                if self.is_intersecting_with_polygons(polygons) {
                    draw_line(
                        point[0],
                        point[1],
                        ending_point[0],
                        ending_point[1],
                        2.,
                        ERROR_COLOR,
                    );
                } else {
                    draw_line(
                        point[0],
                        point[1],
                        ending_point[0],
                        ending_point[1],
                        2.,
                        OK_COLOR,
                    );
                }
            }
        });
    }

    pub fn is_intersecting_with_polygons(&self, polygons: &Vec<Polygon>) -> bool {
        if self.points.len() > 0 {
            polygons.iter().any(|poly| {
                poly.segments().iter().any(|segment| {
                    do_segments_intersect(
                        &vec2_to_segments(
                            self.points[self.points.len() - 1],
                            vec2(mouse_position().0, mouse_position().1),
                        ),
                        &segment,
                    )
                })
            })
        } else {
            false
        }
    }

    pub fn is_on_end(&self) -> bool {
        if self.points.len() > 1 {
            let mouse_ending_distance =
                (vec2(mouse_position().0, mouse_position().1) - self.points[0]).length();
            mouse_ending_distance < 30.
        } else {
            false
        }
    }

    pub fn add_point(&mut self, pos: Vec2, polygons: &mut Vec<Polygon>) {
        if !self.is_on_end() {
            self.points.push(pos);
        } else {
            polygons.push(Polygon::new(
                self.points
                    .iter()
                    .map(|vector| (vector[0], vector[1]))
                    .collect(),
                None,
            ));
            self.reset();
        }
    }

    pub fn reset(&mut self) {
        self.points = [].to_vec();
    }
}
