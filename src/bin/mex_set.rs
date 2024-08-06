use cargo_snippet::snippet;

#[cfg(test)]
mod tests {
  use super::mexset::MexSet;

  #[test]
  fn test() {
    let mut ms = MexSet::new();
    assert_eq!(ms.mex(), 0);
    ms.insert(0); // { 0 }
    assert_eq!(ms.mex(), 1);
    ms.insert(2); // { 0, 2 }
    assert_eq!(ms.mex(), 1);
    ms.insert(1); // { 0, 1, 2 }
    assert_eq!(ms.mex(), 3);

    ms.insert(3); // { 0, 1, 2, 3 }
    ms.insert(4); // { 0, 1, 2, 3, 4 }
    ms.remove(2).unwrap(); // { 0, 1, 3, 4 }
    assert_eq!(ms.mex(), 2);
    ms.remove(3).unwrap(); // { 0, 1, 4 }
    assert_eq!(ms.mex(), 2);

    ms.insert(0); // { 0, 0, 1, 4 }
    assert_eq!(ms.mex(), 2);
    ms.remove(0).unwrap(); // { 0, 1, 4 }
    assert_eq!(ms.mex(), 2);

    assert_eq!(ms.remove(3), Err(0)); // { 0, 1, 4 }
    assert_eq!(ms.mex(), 2);
    assert_eq!(ms.remove(100), Err(0)); // { 0, 1, 4 }
    assert_eq!(ms.mex(), 2);

    ms.insert_multi(2, 3); // { 0, 1, 2, 2, 2, 4 }
    assert_eq!(ms.mex(), 3);
    ms.remove_multi(2, 2).unwrap(); // { 0, 1, 2, 4 }
    assert_eq!(ms.mex(), 3);
    ms.insert_multi(3, 4); // { 0, 1, 2, 3, 3, 3, 3, 4 }
    assert_eq!(ms.mex(), 5);
    ms.remove(3).unwrap(); // { 0, 1, 2, 3, 3, 3, 4 }
    assert_eq!(ms.remove_multi(3, 100), Err(3)); // { 0, 1, 2, 4 }
    assert_eq!(ms.mex(), 3);

    ms.remove_multi(0, 1).unwrap(); // { 1, 2, 4 }
    assert_eq!(ms.mex(), 0);
    assert_eq!(ms.remove_multi(0, 2), Err(0)); // { 1, 2, 4 }
  }
}

fn main() {
  unimplemented!();
}

#[snippet("mexset")]
pub mod mexset {
  use std::collections::{BTreeSet, HashMap};

  #[derive(Debug, Clone, Eq)]
  pub struct MexSet {
    bound: usize,
    excluded: BTreeSet<usize>,
    included: HashMap<usize, usize>,
  }
  
  impl MexSet {
    pub fn new() -> Self {
      Self {
        bound: 1,
        excluded: BTreeSet::from([0]),
        included: HashMap::new(),
      }
    }

    pub fn mex(&self) -> usize {
      *self.excluded.iter().next().unwrap()
    }

    pub fn insert(&mut self, x: usize) {
      self.insert_multi(x, 1);
    }

    pub fn insert_multi(&mut self, x: usize, n: usize) {
      self.excluded.insert(self.bound);
      self.bound += 1;

      if *self.included.get(&x).unwrap_or(&0) == 0 {
        self.excluded.remove(&x);
      }

      self.included.entry(x).and_modify(|e| *e += n).or_insert(n);
    } 

    pub fn remove(&mut self, x: usize) -> Result<(), usize> {
      self.remove_multi(x, 1)
    }

    pub fn remove_multi(&mut self, x: usize, n: usize) -> Result<(), usize> {
      let c = *self.included.get(&x).unwrap_or(&0);

      if let Some(count) = self.included.get_mut(&x) {
        *count = c.saturating_sub(n);
        if *count == 0 {
          self.excluded.insert(x);
        }
      }

      if c < n { Err(c) } else { Ok(()) }
    }
  }

  impl PartialEq for MexSet {
    fn eq(&self, other: &Self) -> bool {
      self.included == other.included
    }
  }
  
}
