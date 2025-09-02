use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
  #[test]
  fn test() {
  }
}

#[snippet("negv")]
pub mod neg_vec {
  use std::ops::{Index, IndexMut};

  #[derive(Debug, Clone, PartialEq, Eq)]
  pub struct NegVec<V> {
    vec_pos: Vec<V>,
    vec_neg: Vec<V>,
  }

  impl<V> NegVec<V> {
    pub fn new() -> Self {
      NegVec {
        vec_pos: vec![],
        vec_neg: vec![],
      }
    }
    pub fn len(&self) -> usize {
      self.vec_pos.len() + self.vec_neg.len()
    }
    pub fn len_pos(&self) -> usize {
      self.vec_pos.len()
    }
    pub fn len_neg(&self) -> usize {
      self.vec_neg.len()
    }
    pub fn is_empty(&self) -> bool {
      self.vec_pos.is_empty() && self.vec_neg.is_empty()
    }
    pub fn push_back(&mut self, v: V) {
      self.vec_pos.push(v);
    }
    pub fn push_front(&mut self, v: V) {
      self.vec_neg.push(v);
    }
    /// Note that this is NOT deque. Returning `None` does not mean the vector is empty.
    pub fn pop_back(&mut self) -> Option<V> {
      self.vec_pos.pop()
    }
    /// Note that this is NOT deque. Returning `None` does not mean the vector is empty.
    pub fn pop_front(&mut self) -> Option<V> {
      self.vec_neg.pop()
    }
    pub fn iter(&self) -> impl Iterator<Item = &V> {
      self.vec_neg.iter().rev().chain(self.vec_pos.iter())
    }
  }

  impl<V: Clone> NegVec<V> {
    pub fn new_init(len_pos: usize, len_neg: usize, init: V) -> Self {
      NegVec {
        vec_pos: vec![init.clone(); len_pos],
        vec_neg: vec![init.clone(); len_neg],
      }
    }
  }

  impl<V> Index<isize> for NegVec<V> {
    type Output = V;
    fn index(&self, i: isize) -> &Self::Output {
      if i >= 0 {
        &self.vec_pos[i as usize]
      } else {
        &self.vec_neg[(-i - 1) as usize]
      }
    }
  }

  impl<V> IndexMut<isize> for NegVec<V> {
    fn index_mut(&mut self, i: isize) -> &mut Self::Output {
      if i >= 0 {
        &mut self.vec_pos[i as usize]
      } else {
        &mut self.vec_neg[(-i - 1) as usize]
      }
    }
  }

}