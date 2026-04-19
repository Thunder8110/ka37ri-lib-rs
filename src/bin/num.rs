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

#[snippet("num")]
pub mod num {
  use std::ops::{Add, Sub, Mul, Div, Rem};

  pub trait Num: PartialEq + Zero + One + NumOps {}

  pub trait NumOps<Rhs = Self, Output = Self>:
    Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output> + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> {}

  impl<T, Rhs, Output> NumOps<Rhs, Output> for T where
    T: Add<Rhs, Output = Output> + Sub<Rhs, Output = Output> + Mul<Rhs, Output = Output>
      + Div<Rhs, Output = Output> + Rem<Rhs, Output = Output> {}

  pub trait Zero: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;
    fn is_zero(&self) -> bool;
  }

  pub trait One: Sized + Mul<Self, Output = Self> {
    fn one() -> Self;
    fn is_one(&self) -> bool;
  }

  macro_rules! zeroone_impl {
    ($t:ty, $z:expr, $o:expr) => {
      impl Zero for $t {
        #[inline]
        fn zero() -> $t {
          $z
        }
        #[inline]
        fn is_zero(&self) -> bool {
          *self == $z
        }
      }
      impl One for $t {
        #[inline]
        fn one() -> $t {
          $o
        }
        #[inline]
        fn is_one(&self) -> bool {
          *self == $o
        }
      }
    };
  }

  zeroone_impl!(isize, 0, 1);
  zeroone_impl!(i8, 0, 1);
  zeroone_impl!(i16, 0, 1);
  zeroone_impl!(i32, 0, 1);
  zeroone_impl!(i64, 0, 1);
  zeroone_impl!(i128, 0, 1);
  zeroone_impl!(usize, 0, 1);
  zeroone_impl!(u8, 0, 1);
  zeroone_impl!(u16, 0, 1);
  zeroone_impl!(u32, 0, 1);
  zeroone_impl!(u64, 0, 1);
  zeroone_impl!(u128, 0, 1);
}