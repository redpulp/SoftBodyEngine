use super::dot::*;

pub struct SoftBody<'a> {
  dots: Vec<&'a Dot>,
  connections: Vec<Box<(&'a Dot, &'a Dot)>>,
}

impl SoftBody<'_> {
  pub fn new(dots: Vec<&Dot>) -> SoftBody {
    SoftBody {
      dots,
      connections: dots
        .iter()
        .enumerate()
        .flat_map(|(index, dot)| {
          dots
            .iter()
            .enumerate()
            .filter(|(inner_index, _inner_dot)| *inner_index == index)
            .map(|(_inner_index, inner_dot)| Box::new((*dot, *inner_dot)))
            .collect::<Vec<Box<(&Dot, &Dot)>>>()
        })
        .collect::<Vec<Box<(&Dot, &Dot)>>>()
        .to_vec(),
    }
  }
}
