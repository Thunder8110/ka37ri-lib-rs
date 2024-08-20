use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
  use crate::string_ex::StringEx;

  #[test]
  fn test() {
    let s = "abracadabra";
    assert_eq!(s.reversed(), "arbadacarba");
    assert!(!s.is_parindrome());

    let t = "abcba";
    assert_eq!(t.reversed(), "abcba");
    assert!(t.is_parindrome());

    let s = "abracadabra".to_owned();
    assert_eq!(s.reversed(), "arbadacarba");
    assert!(!s.is_parindrome());

    let t = "abcba".to_owned();
    assert_eq!(t.reversed(), "abcba");
    assert!(t.is_parindrome());

    let s = "abracadabra";
    let map = [
      ('a', Some('b')),
    ].into();
    assert_eq!(s.translate(&map), "bbrbcbdbbrb");
    let map = [
      ('a', None),
      ('b', Some('a')),
    ].into();
    assert_eq!(s.translate(&map), "arcdar");
    let map = [
      ('a', Some('b')),
      ('b', Some('c')),
      ('c', Some('a')),
    ].into();
    assert_eq!(s.translate(&map), "bcrbabdbcrb");
    
  }
}

#[snippet("strex")]
pub mod string_ex {
  use std::collections::HashMap;

  pub trait StringEx {
    fn reversed(&self) -> String;
    fn is_parindrome(&self) -> bool;
    fn translate(&self, map: &HashMap<char, Option<char>>) -> String;
    fn z_algorithm(&self) -> Vec<usize>;
    fn lcp_array(&self) -> Vec<usize>;
  }

  impl StringEx for str {
    fn reversed(&self) -> String {
      self.chars().rev().collect()
    }
    fn is_parindrome(&self) -> bool {
      self.chars().eq(self.reversed().chars())
    }
    fn translate(&self, map: &HashMap<char, Option<char>>) -> String {
      self.chars().filter_map(|c| map.get(&c).copied().unwrap_or(Some(c))).collect()
    }
    fn z_algorithm(&self) -> Vec<usize> {
      todo!();
    }
    fn lcp_array(&self) -> Vec<usize> {
      todo!();
    }
  }

}