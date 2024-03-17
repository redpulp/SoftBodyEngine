use super::polygon::*;
use macroquad::prelude::*;

const STD_COLOR: Color = WHITE;
// const ERROR_COLOR: Color = RED;
// const OK_COLOR: Color = GREEN;

pub struct IncompletePolygon {
  pub points: Vec<Vec2>,
}

impl IncompletePolygon {
  pub fn new() -> Self {
    Self {
      points: [].to_vec(),
    }
  }

  pub fn draw(&self) {
    self.points.iter().enumerate().for_each(|(i, point)| {
      let ending_point: Vec2;
      if i != self.points.len() - 1 {
        ending_point = self.points[i + 1]
      } else {
        if self.is_on_end() {
          ending_point = self.points[0]
        } else {
          ending_point = vec2(mouse_position().0, mouse_position().1)
        }
      };

      draw_line(
        point[0],
        point[1],
        ending_point[0],
        ending_point[1],
        2.,
        STD_COLOR,
      );
    });
  }

  pub fn is_on_end(&self) -> bool {
    if self.points.len() > 1 {
      let mouse_ending_distance =
        (vec2(mouse_position().0, mouse_position().1) - self.points[0]).length();
      mouse_ending_distance < 50.
    } else {
      false
    }
  }

  pub fn add_point(&mut self, pos: Vec2, polygons: &mut Vec<Polygon>) {
    if !self.is_on_end() {
      self.points.push(pos);
    } else {
      polygons.push(Polygon::new(
        self
          .points
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
