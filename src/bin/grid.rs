use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
use itertools::Itertools;

use super::grid;
  #[test]
  fn test() {
    let g = [".#..#.", ".#.###", ".###.#", "......", "#.###."];
    let g: Vec<Vec<char>> = g.iter().map(|line| line.chars().collect()).collect();
    let grid = grid::Grid::new(g);
    println!("{:#}", grid);
    assert_eq!(grid.get(0, 2), &'.');
    assert_eq!(grid[(1, 4)], '#');
    assert_eq!(grid.row(2).copied().collect_vec(), vec!['.', '#', '#', '#', '.', '#']);
    assert_eq!(grid.column(3).copied().collect_vec(), vec!['.', '#', '#', '.', '#']);
    assert_eq!(grid.neighbour(2, 3, false).sorted().collect_vec(), vec![(1, 3), (2, 2), (2, 4), (3, 3)]);
    assert_eq!(grid.neighbour(2, 3, true).sorted().collect_vec(), vec![(1, 2), (1, 3), (1, 4), (2, 2), (2, 4), (3, 2), (3, 3), (3, 4)]);
    assert_eq!(grid.neighbour(0, 0, false).sorted().collect_vec(), vec![(0, 1), (1, 0)]);
    let mut other = grid::Grid::empty(5, 6, '.');
    for (i, j) in [
      (0, 1), (0, 4),
      (1, 1), (1, 3), (1, 4), (1, 5),
    ] {
      other.update(i, j, '#');
    }
    for ij in [
      (2, 1), (2, 2), (2, 3), (2, 5),
      (4, 0), (4, 2), (4, 3), (4, 4),
    ] {
      other[ij] = '#';
    }
    assert_eq!(grid, other);
  }
}

#[snippet("grid")]
pub mod grid {
  use std::{fmt::Display, ops::{Index, IndexMut}};

  use itertools::Itertools;

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
      let grid = (0..h).map(|i| (0..w).map(|j| init(i, j)).collect()).collect_vec();
      Grid { h, w, grid }
    }
    pub fn get(&self, i: usize, j: usize) -> &T {
      &self.grid[i][j]
    }
    pub fn update(&mut self, i: usize, j: usize, x: T) {
      self.grid[i][j] = x;
    }
    pub fn neighbour(&self, i: usize, j: usize, eight: bool) -> impl Iterator<Item = (usize, usize)> + '_ {
      let dirs = if eight { EIGHT_NEIGHBOUR.iter() } else { FOUR_NEIGHBOUR.iter() };
      dirs.filter_map(move |(di, dj)| {
        let ni = i as isize + di;
        let nj = j as isize + dj;
        if 0 > ni || ni >= self.h as isize || 0 > nj || nj >= self.w as isize {
          None
        } else {
          Some((ni as usize, nj as usize))
        }
      })
    }
    pub fn row(&'a self, i: usize) -> RowIter<'a, T> {
      RowIter { grid: &self, i, j: 0 }
    }
    pub fn column(&'a self, j: usize) -> ColumnIter<'a, T> {
      ColumnIter { grid: &self, i: 0, j }
    }
  }

  impl<T: Clone> Grid<T> {
    pub fn paste(&mut self, i: usize, j: usize, other: &Grid<T>) {
      let oh = other.h;
      let ow = other.w;
      for x in 0..oh {
        for y in 0..ow {
          self[(i + x, j + y)] = other[(x, y)].clone();
        }
      }
    }
  }

  impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;
    fn index(&self, (i, j): (usize, usize)) -> &T {
      self.get(i, j)
    }
  }
  impl<T> IndexMut<(usize, usize)> for Grid<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut T {
      &mut self.grid[i][j]
    }  
  }

  impl<T> Display for Grid<T>
  where
    T: Display,
  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      for i in 0..self.h {
        let sep = if f.alternate() { "" } else { " " };
        let line = self.row(i).map(|x| x.to_string()).join(sep);
        writeln!(f, "{}", line)?;
      }
      Ok(())
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
      if self.j == self.grid.w {
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
      if self.i == self.grid.h {
        return None;
      }
      let res = &self.grid.grid[self.i][self.j];
      self.i += 1;
      Some(res)
    }
  }
}