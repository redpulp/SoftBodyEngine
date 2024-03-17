use super::dot::*;
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

  pub fn draw(&self, dots: &Vec<Dot>) {
    draw_line(
      dots[self.index_1].pos[0],
      dots[self.index_1].pos[1],
      dots[self.index_2].pos[0],
      dots[self.index_2].pos[1],
      2.,
      WHITE,
    );
  }

  fn get_hooke_force(&self, dots: &Vec<Dot>) -> f32 {
    ((dots[self.index_1].pos - dots[self.index_2].pos).length() - self.rest_length) * self.stiffness
  }

  fn get_damping(&self, dots: &Vec<Dot>) -> f32 {
    (dots[self.index_1].pos - dots[self.index_2].pos)
      .normalize()
      .dot(dots[self.index_1].vel - dots[self.index_2].vel)
      * self.damping_factor
  }

  pub fn get_force(&self, dots: &Vec<Dot>) -> f32 {
    return self.get_hooke_force(dots) + self.get_damping(dots);
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
      .for_each(|point| point.draw(&self.points));
  }

  // pub fn update(polygon: polygon::Polygon) {}
}
