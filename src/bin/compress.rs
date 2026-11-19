use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[snippet("comp")]
pub mod compress {
  use std::ops::Index;


  #[derive(Debug)]
  /// A structure for compressing and indexing elements.
  /// # Examples
  /// ```
  /// use compress::Compress;
  /// let mut c = Compress::new();
  /// c.add(34);
  /// c.add(12);
  /// c.build();
  /// assert_eq!(c.get_index(&12), Some(0));
  /// assert_eq!(c.get_index(&34), Some(1));
  /// assert_eq!(c.get_index(&23), None);
  /// assert_eq!(c[0], 12);
  /// assert_eq!(c[1], 34);
  /// ```
  pub struct Compress<T: Ord> {
    elements: Vec<T>,
    isbuilt: bool,
  }

  impl<T: Ord> Compress<T> {
    pub fn new() -> Self {
      Self {
        elements: Vec::new(),
        isbuilt: false,
      }
    }
    pub fn from_vec(elements: Vec<T>) -> Self {
      Self {
        elements,
        isbuilt: false,
      }
    }
    pub fn add(&mut self, element: T) {
      self.elements.push(element);
      self.isbuilt = false;
    }
    pub fn build(&mut self) {
      if !self.isbuilt {
        self.elements.sort_unstable();
        self.elements.dedup();
        self.isbuilt = true;
      }
    }
    pub fn get_vec(&mut self) -> &Vec<T> {
      self.build();
      &self.elements
    }
    pub fn get_index(&mut self, element: &T) -> Option<usize> {
      assert!(self.isbuilt, "Compress must be built before getting index");
      self.elements.binary_search(element).ok()
    }
    pub fn get_index_exact(&mut self, element: &T) -> usize {
      self.get_index(element).expect("Element not found in compress")
    }
    pub fn len(&self) -> usize {
      self.elements.len()
    }
  }

  impl<T: Ord> Index<usize> for Compress<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
      &self.elements[index]
    }
  }
}

#[cfg(test)]
mod tests {
  use super::compress::Compress;

  #[test]
  fn test_compress() {
    let mut c = Compress::new();
    c.add(314);
    c.add(159);
    c.add(265);
    c.build();
    assert_eq!(c.get_index(&159), Some(0));
    assert_eq!(c.get_index(&265), Some(1));
    assert_eq!(c.get_index(&314), Some(2));
    assert_eq!(c.get_index(&123), None);
  }

  #[test]
  fn test_compress_vec() {
    let mut c = Compress::from_vec(vec![314, 159, 265]);
    c.build();
    assert_eq!(c.get_index(&159), Some(0));
    assert_eq!(c.get_index(&265), Some(1));
    assert_eq!(c.get_index(&314), Some(2));
    assert_eq!(c.get_index(&123), None);
  }

  #[test]
  fn test_compress_index() {
    let mut c = Compress::from_vec(vec![314, 159, 265]);
    c.build();
    assert_eq!(c[0], 159);
    assert_eq!(c[1], 265);
    assert_eq!(c[2], 314);
  }

  #[test]
  fn test_compress_exact() {
    let mut c = Compress::from_vec(vec![314, 159, 265]);
    c.build();
    assert_eq!(c.get_index_exact(&159), 0);
    assert_eq!(c.get_index_exact(&265), 1);
    assert_eq!(c.get_index_exact(&314), 2);
  }

  #[test]
  fn test_compress_construct() {
    let mut c = Compress::from_vec(vec![314, 159, 265]);
    c.add(358);
    c.add(314);
    c.build();
    assert_eq!(c.get_index(&159), Some(0));
    assert_eq!(c.get_index(&265), Some(1));
    assert_eq!(c.get_index(&314), Some(2));
    assert_eq!(c.get_index(&358), Some(3));
    assert_eq!(c.get_index(&979), None);
  }
}
