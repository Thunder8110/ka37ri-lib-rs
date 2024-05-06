use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
  use super::grid;
  #[test]
  fn test() {
    let g = [".#..#.", ".#.###", ".###.#", "......", "#.###."];
    let g: Vec<Vec<char>> = g.iter().map(|&line| line.chars().collect()).collect();
    let grid = grid::Grid::new(g, '.');
    let x = grid.get(0, 2);
  }
}

#[snippet("grid")]
pub mod grid {
  const FOUR_NEIGHBOUR: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
  const EIGHT_NEIGHBOUR: [(isize, isize); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];
  pub struct Grid<T> {
    h: usize,
    w: usize,
    grid: Vec<Vec<T>>,
    transpose: Vec<Vec<T>>,
    modified_row: Vec<bool>,
    modified_column: Vec<bool>,
  }
  impl<T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>, init: T) -> Self
    where
      T: Clone,
    {
      let h = grid.len();
      let w = grid[0].len();
      assert!(grid.iter().all(|line| line.len() == w));
      let transpose = vec![vec![init.clone(); h]; w];
      Grid { h, w, grid, transpose, modified_row: vec![false; h], modified_column: vec![false; w] }
    }
    pub fn empty(h: usize, w: usize, init: T) -> Self
    where
      T: Clone,
      Vec<T>: Clone,
    {
      let grid = vec![vec![init.clone(); w]; h];
      let transpose = vec![vec![init.clone(); h]; w];
      Grid { h, w, grid, transpose, modified_row: vec![false; h], modified_column: vec![false; w] }
    }
    pub fn get(&self, i: usize, j: usize) -> T
    where
      T: Clone,
    {
      self.grid[i][j].clone()
    }
    pub fn update(&mut self, i: usize, j: usize, x: T)
    where
      T: Clone,
    {
      self.modified_row[i] = true;
      self.modified_column[j] = true;
      self.grid[i][j] = x.clone();
      self.transpose[j][i] = x.clone();
    }
    pub fn neighbour(&self, i: usize, j: usize, eight: bool) -> Vec<(usize, usize)> {
      let dirs = if eight { EIGHT_NEIGHBOUR.to_vec() } else { FOUR_NEIGHBOUR.to_vec() };
      let mut res = Vec::new();
      for (di, dj) in dirs {
        let ni = i as isize + di;
        let nj = j as isize + dj;
        if 0 > ni || ni >= self.h as isize || 0 > nj || nj >= self.w as isize { continue; }
        res.push((ni as usize, nj as usize));
      }
      res
    }
    pub fn row(&self, i: usize) -> &Vec<T> {
      &self.grid[i]
    }
    pub fn column(&self, j: usize) -> &Vec<T> {
      &self.transpose[j]
    }
    pub fn make_column(&mut self, j: usize)
    where
      T: Clone,
    {
      for i in 0..self.h {
        self.transpose[j][i] = self.grid[i][j].clone();
      }
    }
    pub fn make_transpose(&mut self)
    where
      T: Clone,
    {
      for j in 0..self.w {
        self.make_column(j);
      }
    }
  }
}