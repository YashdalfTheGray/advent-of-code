use crate::utils;

pub struct PathDetails {
  pub hslope: u32,
  pub vslope: u32,
  pub computed_path: Vec<char>,
}

impl PathDetails {
  pub fn new(hslope: u32, vslope: u32, grid: &[Vec<char>]) -> PathDetails {
    PathDetails {
      hslope,
      vslope,
      computed_path: utils::traverse(&grid, hslope, vslope),
    }
  }

  pub fn number_of_trees(&self) -> usize {
    self.computed_path.iter().filter(|&e| *e == 'X').count()
  }
}
