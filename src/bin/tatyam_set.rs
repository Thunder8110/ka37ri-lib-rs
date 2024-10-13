fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
  use crate::tatyam_set::TatyamSet;
  #[test]
  fn test() {
  }
}

pub mod tatyam_set {
  use std::ops::Index;

  // rewrote https://github.com/tatyam-prime/SortedSet/blob/main/SortedSet.py in Rust
  
  const BUCKET_RATIO: usize = 16;
  const SPLIT_RATIO: usize = 24;

  #[derive(Debug, Eq)]
  pub struct TatyamSet<T: Ord + Clone> {
    a: Vec<Vec<T>>,
    len: usize,
  }

  impl<T: Ord + Clone> TatyamSet<T> {
    pub fn new() -> Self {
      TatyamSet { a: vec![vec![]], len: 0 }
    }
    pub fn from_iter<I: Iterator<Item = T>>(iter: I) -> Self {
      let mut ls = iter.collect::<Vec<_>>();
      if ls.iter().zip(ls.iter().skip(1)).any(|(a, b)| a > b) {
        ls.sort();
      }
      ls.dedup();
      let len = ls.len();
      let chunk_size = (len as f64 * BUCKET_RATIO as f64).sqrt().floor() as usize;
      let a = ls.chunks(chunk_size).map(|x| x.to_vec()).collect();
      TatyamSet { a, len }
    }
    pub fn iter(&self) -> impl Iterator<Item = &T> {
      self.a.iter().flat_map(|x| x.iter())
    }
    pub fn len(&self) -> usize {
      self.len
    }
    pub fn is_empty(&self) -> bool {
      self.len == 0
    }
    fn position(&self, x: &T) -> (usize, usize) {
      let i = self.a.iter().position(|b| x <= b.last().unwrap()).unwrap_or(self.a.len() - 1);
      (i, self.a[i].partition_point(|y| x > y))
    }
    pub fn contains(&self, x: &T) -> bool {
      if self.is_empty() {
        return false;
      }
      let (i, j) = self.position(x);
      i != self.a[i].len() && &self.a[i][j] == x
    }
    pub fn insert(&mut self, x: T) -> bool {
      if self.is_empty() {
        self.a[0].push(x);
        self.len += 1;
        return true;
      }
      let (i, j) = self.position(&x);
      let b = &mut self.a[i];
      if i != b.len() && &b[j] == &x {
        return false;
      }
      b.insert(j, x);
      self.len += 1;
      if b.len() > SPLIT_RATIO {
        let b1 = b.split_off(b.len() / 2);
        self.a.insert(i + 1, b1);
      }
      true
    }
    fn pop_inner(&mut self, i: usize, j: usize) -> T {
      let x = self.a[i].remove(j);
      self.len -= 1;
      if self.a[i].is_empty() {
        self.a.remove(i);
      }
      x
    }
    pub fn remove(&mut self, x: &T) -> bool {
      if self.is_empty() {
        return false;
      }
      let (i, j) = self.position(x);
      if j == self.a[i].len() || &self.a[i][j] != x {
        return false;
      }
      self.pop_inner(i, j);
      true
    }
    pub fn lt(&self, x: &T) -> Option<&T> {
      if let Some(i) = self.a.iter().rev().position(|b| b.first().unwrap() < x) {
        let j = self.a[self.a.len()-i].partition_point(|y| y < x);
        Some(&self.a[i][j-1])
      } else {
        None
      }
    }
    pub fn le(&self, x: &T) -> Option<&T> {
      if let Some(i) = self.a.iter().rev().position(|b| b.first().unwrap() <= x) {
        let j = self.a[self.a.len()-i].partition_point(|y| y <= x);
        Some(&self.a[i][j-1])
      } else {
        None
      }
    }
    pub fn gt(&self, x: &T) -> Option<&T> {
      if let Some(i) = self.a.iter().position(|b| b.last().unwrap() > x) {
        let j = self.a[i].partition_point(|y| y <= x);
        Some(&self.a[i][j])
      } else {
        None
      }
    }
    pub fn ge(&self, x: &T) -> Option<&T> {
      if let Some(i) = self.a.iter().position(|b| b.last().unwrap() >= x) {
        let j = self.a[i].partition_point(|y| y < x);
        Some(&self.a[i][j])
      } else {
        None
      }
    }
    pub fn pop(&mut self, i: usize) -> Option<T> {
      let mut i = i;
      for (j, b) in self.a.iter().enumerate() {
        if i < b.len() {
          return Some(self.pop_inner(j, i));
        }
        i -= b.len();
      }
      None
    }
  }

  impl<T: Ord + Clone> PartialEq for TatyamSet<T> {
    fn eq(&self, other: &Self) -> bool {
      self.iter().eq(other.iter())
    }
  }

  impl<T: Ord + Clone> Index<usize> for TatyamSet<T> {
    type Output = T;
    fn index(&self, i: usize) -> &T {
      if i >= self.len() {
        panic!("index out of range");
      }
      let mut i = i;
      for b in self.a.iter() {
        if i < b.len() {
          return &b[i];
        }
        i -= b.len();
      }
      panic!("index out of range");
    }
  }
}