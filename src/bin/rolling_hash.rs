use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
  use std::rc::Rc;

  use super::rolling_hash::{make_rhbase, RollingHash};

  #[test]
  fn test() {
    let b = 1_000_000_007;
    let s = "abracadabra";
    let rh_base = make_rhbase(b, (1 << 61) - 1, |c: char| c as u64);
    let mut rh_s = RollingHash::new_with(Rc::clone(&rh_base), &s.chars().collect::<Vec<_>>());
    assert_eq!(rh_s.get(0..4), rh_s.get(7..11));
    assert_ne!(rh_s.get(0..4), rh_s.get(1..5));
    assert_ne!(rh_s.get(0..7), rh_s.get(0..6));
    let t = "academy";
    let mut rh_t = RollingHash::new_with(Rc::clone(&rh_base), &t.chars().collect::<Vec<_>>());
    assert_ne!(rh_s.get(0..4), rh_t.get(0..4));
    assert_eq!(rh_s.get(3..7), rh_t.get(0..4));
  }
}

#[snippet("rh")]
pub mod rolling_hash {
  use std::{cell::RefCell, ops::Range, rc::Rc};

  #[derive(Debug)]
  pub struct RHBase<T> {
    base: u128,
    modulo: u128,
    power: Vec<u64>,
    func: fn(T) -> u64,
  }
  impl<T> RHBase<T>
  where T: Clone,
  {
    pub fn new(base: u128, modulo: u128, func: fn(T) -> u64) -> Self {
      RHBase { base, modulo, power: vec![1], func }
    }
    fn pow(&mut self, n: usize) -> u64 {
      while self.power.len() <= n {
        let p = (*self.power.last().unwrap() as u128 * self.base) % self.modulo;
        self.power.push(p as u64);
      }
      self.power[n]
    }
  }

  #[derive(Debug)]
  pub struct RollingHash<T> {
    rh_base: Rc<RefCell<RHBase<T>>>,
    hash: Vec<u64>,
  }
  impl<T> RollingHash<T>
  where T: Clone,
  {
    pub fn new(rh_base: Rc<RefCell<RHBase<T>>>) -> Self {
      RollingHash { rh_base, hash: vec![0] }
    }
    pub fn new_with(rh_base: Rc<RefCell<RHBase<T>>>, s: &[T]) -> Self {
      let mut rh = RollingHash::new(rh_base);
      rh.build(s);
      rh
    }
    fn build(&mut self, s: &[T]) {
      let mut hash = vec![0; s.len() + 1];
      for i in 0..s.len() {
        let h = hash[i] as u128 * self.rh_base.borrow().base + (self.rh_base.borrow().func)(s[i].clone()) as u128;
        hash[i + 1] = (h % self.rh_base.borrow().modulo) as u64;
      }
      self.hash = hash;
    }
    pub fn get(&mut self, range: Range<usize>) -> u64 {
      let st = self.hash[range.start] as u128
        * self.rh_base.borrow_mut().pow(range.end - range.start) as u128;
      let h = self.hash[range.end] as u128 + self.rh_base.borrow().modulo - (st % self.rh_base.borrow().modulo);
      (h % self.rh_base.borrow().modulo) as u64
    }
    pub fn len(&self) -> usize {
      self.hash.len() - 1
    }
    pub fn push(&mut self, x: T) {
      let h = *self.hash.last().unwrap() as u128 * self.rh_base.borrow().base + (self.rh_base.borrow().func)(x) as u128;
      self.hash.push((h % self.rh_base.borrow().modulo) as u64);
    }
  }
  pub fn make_rhbase<T>(base: u128, modulo: u128, func: fn(T) -> u64) -> Rc<RefCell<RHBase<T>>>
  where T: Clone,
  {
    Rc::new(RefCell::new(RHBase::new(base, modulo, func)))
  }
}
