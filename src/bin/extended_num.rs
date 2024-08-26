use std::{cmp::Ordering, ops::{Add, Sub, Mul, Div}, fmt::Debug};
use ExtendedNum::{PosInf, Finite, NegInf};

#[derive(Clone, Copy, Debug)]
pub enum ExtendedNum<T>
where T: Ord + TryFrom<i32> + Debug {
  PosInf,
  Finite(T),
  NegInf,
}
impl<T> ExtendedNum<T>
where T: Ord + TryFrom<i32> + Debug {
  pub fn zero() -> T {
    T::try_from(0).unwrap_or_else(|_| panic!())
  }
  pub fn is_finite(&self) -> bool {
    match self {
      Finite(_) => true,
      _ => false,
    }
  }
  pub fn is_infinity(&self) -> bool {
    match self {
      Finite(_) => false,
      _ => true,
    }
  }
  pub fn is_pos_inf(&self) -> bool {
    match self {
      PosInf => true,
      _ => false,
    }
  }
  pub fn is_neg_inf(&self) -> bool {
    match self {
      NegInf => true,
      _ => false,
    }
  }
  pub fn get_finite_value(&self) -> &T {
    match self {
      Finite(v) => v,
      _ => panic!("This is not finite"),
    }
  }
}
impl<T> PartialEq for ExtendedNum<T>
where T: Ord + TryFrom<i32> + Debug {
  fn eq(&self, other: &Self) -> bool {
    match (self, other) {
      (PosInf, PosInf) => true,
      (NegInf, NegInf) => true,
      (Finite(s), Finite(o)) => s.eq(o),
      (_, _) => false,
    }
  }
}
impl<T> Eq for ExtendedNum<T>
where T: Ord + TryFrom<i32> + Debug {
}
impl<T> PartialOrd for ExtendedNum<T>
where T: Ord + TryFrom<i32> + Debug {
  fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
    Some(self.cmp(other))
  }
}
impl<T> Ord for ExtendedNum<T>
where T: Ord + TryFrom<i32> + Debug {
  fn cmp(&self, other: &Self) -> Ordering {
    match (self, other) {
      (PosInf, PosInf) => Ordering::Equal,
      (PosInf, _) => Ordering::Greater,
      (NegInf, NegInf) => Ordering::Equal,
      (NegInf, _) => Ordering::Less,
      (Finite(_), PosInf) => Ordering::Less,
      (Finite(_), NegInf) => Ordering::Greater,
      (Finite(s), Finite(o)) => s.cmp(o),
    }
  }
}
impl<T> Add for ExtendedNum<T>
where T: Ord + Add<Output = T> + TryFrom<i32> + Debug {
  type Output = Self;
  fn add(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (PosInf, PosInf) => PosInf,
      (PosInf, Finite(_)) => PosInf,
      (NegInf, NegInf) => NegInf,
      (NegInf, Finite(_)) => NegInf,
      (Finite(s), Finite(r)) => Finite(s + r),
      (Finite(_), PosInf) => PosInf,
      (Finite(_), NegInf) => NegInf,
      (s, r) => panic!("Invalid addition: {s:?} + {r:?}"),
    }
  }
}
impl<T> Sub for ExtendedNum<T>
where T: Ord + Sub<Output = T> + TryFrom<i32> + Debug {
  type Output = Self;
  fn sub(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (PosInf, NegInf) => PosInf,
      (PosInf, Finite(_)) => PosInf,
      (NegInf, PosInf) => NegInf,
      (NegInf, Finite(_)) => NegInf,
      (Finite(s), Finite(r)) => Finite(s - r),
      (Finite(_), PosInf) => NegInf,
      (Finite(_), NegInf) => PosInf,
      (s, r) => panic!("Invalid subtraction: {s:?} - {r:?}"),
    }
  }
}
impl<T> Mul for ExtendedNum<T>
where T: Ord + Mul<Output = T> + TryFrom<i32> + Debug {
  type Output = Self;
  fn mul(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (PosInf, PosInf) => PosInf,
      (PosInf, NegInf) => NegInf,
      (PosInf, Finite(x)) if x > ExtendedNum::zero() => PosInf,
      (PosInf, Finite(x)) if x < ExtendedNum::zero() => NegInf,
      (NegInf, PosInf) => NegInf,
      (NegInf, NegInf) => PosInf,
      (NegInf, Finite(x)) if x > ExtendedNum::zero() => NegInf,
      (NegInf, Finite(x)) if x < ExtendedNum::zero() => PosInf,
      (Finite(s), Finite(r)) => Finite(s * r),
      (Finite(x), PosInf) if x > ExtendedNum::zero() => PosInf,
      (Finite(x), PosInf) if x < ExtendedNum::zero() => NegInf,
      (Finite(x), NegInf) if x > ExtendedNum::zero() => NegInf,
      (Finite(x), NegInf) if x < ExtendedNum::zero() => PosInf,
      (s, r) => panic!("Invalid multiplication: {s:?} * {r:?}"),
    }
  }
}
impl<T> Div for ExtendedNum<T>
where T: Ord + Div<Output = T> + TryFrom<i32> + Debug {
  type Output = Self;
  fn div(self, rhs: Self) -> Self::Output {
    match (self, rhs) {
      (PosInf, Finite(x)) if x > ExtendedNum::zero() => PosInf,
      (PosInf, Finite(x)) if x < ExtendedNum::zero() => NegInf,
      (NegInf, Finite(x)) if x > ExtendedNum::zero() => NegInf,
      (NegInf, Finite(x)) if x < ExtendedNum::zero() => PosInf,
      (Finite(_), PosInf) => Finite(ExtendedNum::zero()),
      (Finite(_), NegInf) => Finite(ExtendedNum::zero()),
      (Finite(s), Finite(r)) => Finite(s / r),
      (s, r) => panic!("Invalid division: {s:?} / {r:?}"),
    }
  }
}