use super::dot::*;
use super::polygon::*;
use macroquad::prelude::*;

const DAMPING_FACTOR: f32 = 0.5;

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
    return (self.get_hooke_force(dot1, dot2) + self.get_damping(dot1, dot2))
      * ((dot1.pos - dot2.pos).normalize());
  }
}

pub struct SoftBody {
  points: Vec<Dot>,
  springs: Vec<Spring>,
}

impl SoftBody {
  pub fn new() -> SoftBody {
    fn get_all_springs(dots: &Vec<Dot>) -> Vec<Spring> {
      dots
        .clone()
        .iter()
        .enumerate()
        .flat_map(|(index, _)| {
          dots
            .clone()
            .iter()
            .enumerate()
            .filter(|&(inner_index, _)| inner_index > index)
            .map(|(inner_index, _)| Spring::new(&dots, index, inner_index, 0.5))
            .collect::<Vec<Spring>>()
        })
        .collect::<Vec<Spring>>()
    }

    let pos1 = screen_width() / 2.;
    let pos2 = screen_height() / 2.;

    let dots = [
      Dot::new(Some(vec2(pos1, pos2 - 20.)), None),
      Dot::new(Some(vec2(pos1 + 18., pos2 + 12.)), None),
      Dot::new(Some(vec2(pos1 - 18., pos2 + 12.)), None),
    ]
    .to_vec();

    SoftBody {
      springs: get_all_springs(&dots),
      points: dots,
    }
  }

  pub fn draw(&self) {
    self.points.iter().for_each(|point| point.draw());
    self
      .springs
      .iter()
      .for_each(|spring| spring.draw(&self.points[spring.index_1], &self.points[spring.index_2]));
  }

  fn add_force_to_points(&mut self, spring_force: Vec2, indexes: (usize, usize)) {
    self.points[indexes.0].add_force(spring_force);
    self.points[indexes.0].add_force(-spring_force);
  }

  pub fn update(&mut self) {
    self.springs.iter().for_each(|(spring)| {
      let spring_force =
        spring.get_force(&self.points[spring.index_1], &self.points[spring.index_2]);

      self.add_force_to_points(spring_force, (spring.index_1, spring.index_2));
    });
  }

  pub fn handle_collision(&mut self, polygon: &Polygon) {
    self
      .points
      .iter_mut()
      .for_each(|point| point.handle_collision(polygon));
  }
}