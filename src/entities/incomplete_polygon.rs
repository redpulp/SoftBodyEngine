use super::polygon::*;
use super::segment::*;
use super::soft_body::SoftBody;
use crate::utils::math::*;
use macroquad::prelude::*;

const STD_COLOR: Color = WHITE;
const ERROR_COLOR: Color = RED;
const OK_COLOR: Color = GRAY;

pub struct IncompletePolygon {
    pub points: Vec<Vec2>,
}

impl Default for IncompletePolygon {
    fn default() -> Self {
        Self::new()
    }
}

impl IncompletePolygon {
    pub fn new() -> Self {
        Self { points: vec![] }
    }

    pub fn draw(&self, polygons: &[Polygon], soft_body: &SoftBody) {
        self.points.iter().enumerate().for_each(|(i, point)| {
            let is_last_segment = i == self.points.len() - 1;
            let is_on_end = self.is_on_start();
            let mouse_position = vec2(mouse_position().0, mouse_position().1);

            let ending_point = if !is_last_segment {
                self.points[i + 1]
            } else if is_on_end {
                // Snap to start if close enough
                self.points[0]
            } else {
                mouse_position
            };

            let drawing_color = if !is_last_segment {
                STD_COLOR
            } else if self.is_intersecting_with_polygons(polygons)
                || self.is_intersecting_with_soft_body(soft_body)
            {
                ERROR_COLOR
            } else {
                OK_COLOR
            };

            draw_line(
                point[0],
                point[1],
                ending_point[0],
                ending_point[1],
                2.,
                drawing_color,
            );
        });
    }

    pub fn is_intersecting_with_polygons(&self, polygons: &[Polygon]) -> bool {
        !self.points.is_empty()
            && polygons.iter().any(|poly| {
                poly.segments().iter().any(|segment| {
                    do_segments_intersect(
                        &Segment {
                            p1: self.points[self.points.len() - 1],
                            p2: vec2(mouse_position().0, mouse_position().1),
                        },
                        segment,
                    )
                })
            })
    }

    pub fn is_intersecting_with_soft_body(&self, body: &SoftBody) -> bool {
        !self.points.is_empty()
            && body
                .springs
                .iter()
                .filter(|spring| spring.is_on_border)
                .any(|spring| {
                    do_segments_intersect(
                        &Segment {
                            p1: body.points[spring.index_1].pos,
                            p2: body.points[spring.index_2].pos,
                        },
                        &Segment {
                            p1: self.points[self.points.len() - 1],
                            p2: vec2(mouse_position().0, mouse_position().1),
                        },
                    )
                })
    }

    // Checking if the mouse is close to the starting point
    fn is_on_start(&self) -> bool {
        self.points.len() > 2
            && (vec2(mouse_position().0, mouse_position().1) - self.points[0]).length() < 30.
    }

    pub fn add_point(&mut self, pos: Vec2, polygons: &mut Vec<Polygon>) {
        if !self.is_on_start() {
            self.points.push(pos);
            return;
        }

        polygons.push(Polygon::new(
            self.points
                .iter()
                .map(|vector| (vector[0], vector[1]))
                .collect(),
            None,
        ));
        self.reset();
    }

    pub fn reset(&mut self) {
        self.points = vec![];
    }
}
