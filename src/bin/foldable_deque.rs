use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
  use super::foldable_deque::FoldableDeque;
  #[test]
  fn test_pushpop() {
    let mut fd = FoldableDeque::new(|x, y| x + y);
    fd.push_front(1); // [1]
    fd.push_front(2); // [2, 1]
    fd.push_back(3); // [2, 1, 3]
    fd.push_back(4); // [2, 1, 3, 4]
    assert_eq!(fd.pop_front(), Some(2)); // [1, 3, 4]
    assert_eq!(fd.pop_back(), Some(4)); // [1, 3]
    assert_eq!(fd.pop_back(), Some(3)); // [1]
    assert_eq!(fd.pop_back(), Some(1)); // []
    assert_eq!(fd.pop_back(), None);

    fd.push_front(5); // [5]
    fd.push_back(6); // [5, 6]
    assert_eq!(fd.pop_front(), Some(5)); // [6]
    assert_eq!(fd.pop_front(), Some(6)); // []

    fd.push_front(7); // [7]
    fd.push_front(8);  // [8, 7]
    fd.push_front(9); // [9, 8, 7]
    fd.push_front(10); // [10, 9, 8, 7]
    assert_eq!(fd.pop_back(), Some(7)); // [10, 9, 8]

    fd.push_back(11); // [10, 9, 8, 11]
    fd.push_back(12); // [10, 9, 8, 11, 12]
    assert_eq!(fd.pop_front(), Some(10)); // [9, 8, 11, 12]
    assert_eq!(fd.pop_front(), Some(9)); // [8, 11, 12]
    assert_eq!(fd.pop_front(), Some(8)); // [11, 12]
    assert_eq!(fd.pop_front(), Some(11)); // [12]
    assert_eq!(fd.pop_front(), Some(12)); // []
    assert_eq!(fd.pop_front(), None);
  }
  
  #[test]
  fn test_fold() {
    let mut fd = FoldableDeque::new(|x, y| x + y);
    fd.push_front(1); // [1]
    fd.push_front(2); // [2, 1]
    fd.push_back(3); // [2, 1, 3]
    fd.push_back(4); // [2, 1, 3, 4]
    assert_eq!(fd.fold_all(), Some(10));
    fd.pop_front(); // [1, 3, 4]
    fd.pop_back(); // [1, 3]
    assert_eq!(fd.fold_all(), Some(4));
    fd.pop_back(); // [1]
    fd.pop_back(); // []
    assert_eq!(fd.fold_all(), None);

    fd.push_front(5); // [5]
    assert_eq!(fd.fold_all(), Some(5));

    fd.pop_front(); // []
    fd.push_back(6); // [6]
    assert_eq!(fd.fold_all(), Some(6));
  }
}

#[snippet("fd")]
pub mod foldable_deque {
  use std::collections::VecDeque;

  pub struct FoldableDeque<T> {
    data: VecDeque<T>,
    front_fold: Vec<T>,
    back_fold: Vec<T>,
    func: fn(T, T) -> T,
  }
  impl<T: Clone> FoldableDeque<T> {
    pub fn new(func: fn(T, T) -> T) -> Self {
      FoldableDeque {
        data: VecDeque::new(),
        front_fold: Vec::new(),
        back_fold: Vec::new(),
        func,
      }
    }
    pub fn push_front(&mut self, x: T) {
      self.front_fold.push(self.front_fold.last().map_or(
        x.clone(),
        |last| (self.func)(x.clone(), last.clone())
      ));
      self.data.push_front(x);
    }
    pub fn push_back(&mut self, x: T) {
      self.back_fold.push(self.back_fold.last().map_or(
        x.clone(),
        |last| (self.func)(last.clone(), x.clone())
      ));
      self.data.push_back(x);
    }
    fn split(&mut self, p: usize) {
      self.front_fold.clear();
      self.back_fold.clear();
      for x in self.data.iter().take(p).rev() {
        self.front_fold.push(self.front_fold.last().map_or(
          x.clone(),
          |last| (self.func)(last.clone(), x.clone())
        ));
      }
      for x in self.data.iter().skip(p) {
        self.back_fold.push(self.back_fold.last().map_or(
          x.clone(),
          |last| (self.func)(last.clone(), x.clone())
        ));
      }
    }
    pub fn pop_front(&mut self) -> Option<T> {
      if self.front_fold.is_empty() {
        self.split((self.data.len() + 1) / 2);
      }
      self.front_fold.pop();
      self.data.pop_front()
    }
    pub fn pop_back(&mut self) -> Option<T> {
      if self.back_fold.is_empty() {
        self.split(self.data.len() / 2);
      }
      self.back_fold.pop();
      self.data.pop_back()
    }
    pub fn fold_all(&self) -> Option<T> {
      match (self.front_fold.last(), self.back_fold.last()) {
        (Some(f), Some(b)) => Some((self.func)(f.clone(), b.clone())),
        (Some(f), None) => Some(f.clone()),
        (None, Some(b)) => Some(b.clone()),
        (None, None) => None,
      }
    }
  }
}