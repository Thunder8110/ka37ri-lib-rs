use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
  use super::char_range_vec::CharRangeVec;
  #[test]
  fn test() {
    let mut v = CharRangeVec::new('a'..='z', 0);
    v['a'] = 1;
    v['b'] = 2;
    v[2] = 3; // v['c']
    v[3] = 4; // v['d']
    assert_eq!(v['a'], 1);
    assert_eq!(v[1], 2); // v['b']
    assert_eq!(v['c'], 3);
    assert_eq!(v[3], 4); // v['d']
    assert_eq!(v['e'], 0);
    assert_eq!(v[25], 0); // v['z']
    assert_eq!(v.len(), 26);
  }
}

#[snippet("crv")]
pub mod char_range_vec {
  use std::ops::{Index, IndexMut, RangeInclusive};

  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct CharRangeVec<V> {
    vec: Vec<V>,
    range: RangeInclusive<char>,
    len: usize,
  }

  impl<V> CharRangeVec<V>
  where
    V: Clone,
  {
    pub fn new(range: RangeInclusive<char>, init: V) -> Self {
      let len = *range.end() as usize - *range.start() as usize + 1;
      CharRangeVec { vec: vec![init.clone(); len], range, len }
    }
    pub fn len(&self) -> usize {
      self.len
    }
  }

  impl<V> Index<char> for CharRangeVec<V> {
    type Output = V;
    fn index(&self, c: char) -> &Self::Output {
      let idx = c as usize - *self.range.start() as usize;
      &self.vec[idx]
    }
  }
  impl<V> IndexMut<char> for CharRangeVec<V> {
    fn index_mut(&mut self, c: char) -> &mut Self::Output {
      let idx = c as usize - *self.range.start() as usize;
      &mut self.vec[idx]
    }
  }
  impl<V> Index<usize> for CharRangeVec<V> {
    type Output = V;
    fn index(&self, idx: usize) -> &Self::Output {
      &self.vec[idx]
    }
  }
  impl<V> IndexMut<usize> for CharRangeVec<V> {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
      &mut self.vec[idx]
    }
  }
}