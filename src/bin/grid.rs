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
    let g: Vec<Vec<char>> = g.iter().map(|line| line.chars().collect()).collect();
    let grid = grid::Grid::new(g);
    let x = grid.get(0, 2);
  }
}

#[snippet("grid")]
pub mod grid {
  const FOUR_NEIGHBOUR: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
  const EIGHT_NEIGHBOUR: [(isize, isize); 8] = [(1, 0), (-1, 0), (0, 1), (0, -1), (1, 1), (1, -1), (-1, 1), (-1, -1)];

  #[derive(Debug, PartialEq, Eq, Clone)]
  pub struct Grid<T> {
    h: usize,
    w: usize,
    grid: Vec<Vec<T>>,
  }
  impl<'a, T> Grid<T> {
    pub fn new(grid: Vec<Vec<T>>) -> Self {
      let h = grid.len();
      let w = grid[0].len();
      assert!(grid.iter().all(|line| line.len() == w));
      Grid { h, w, grid }
    }
    pub fn empty(h: usize, w: usize, init: T) -> Self
    where
      T: Clone,
      Vec<T>: Clone,
    {
      let grid = vec![vec![init.clone(); w]; h];
      Grid { h, w, grid }
    }
    pub fn empty_by(h: usize, w: usize, init: fn(usize, usize) -> T) -> Self {
      let grid = (0..h).map(|i| (0..w).map(|j| init(i, j)).collect()).collect::<Vec<_>>();
      Grid { h, w, grid }
    }
    pub fn get(&self, i: usize, j: usize) -> &T {
      &self.grid[i][j]
    }
    pub fn update(&mut self, i: usize, j: usize, x: T) {
      self.grid[i][j] = x;
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
    pub fn row(&'a self, i: usize) -> RowIter<'a, T> {
      RowIter { grid: &self, i, j: 0 }
    }
    pub fn column(&'a self, j: usize) -> ColumnIter<'a, T> {
      ColumnIter { grid: &self, i: 0, j }
    }
  }

  pub struct RowIter<'a, T> {
    grid: &'a Grid<T>,
    i: usize,
    j: usize,
  }
  impl<'a, T> Iterator for RowIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
      if self.i == self.grid.h {
        return None;
      }
      let res = &self.grid.grid[self.i][self.j];
      self.j += 1;
      Some(res)
    }
  }

  pub struct ColumnIter<'a, T> {
    grid: &'a Grid<T>,
    i: usize,
    j: usize,
  }
  impl<'a, T> Iterator for ColumnIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
      if self.j == self.grid.w {
        return None;
      }
      let res = &self.grid.grid[self.i][self.j];
      self.i += 1;
      Some(res)
    }
  }
}