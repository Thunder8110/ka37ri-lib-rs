use ac_library::{MapMonoid, Monoid};
use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[snippet("rarsq")]
struct RAddRSumMonoid;
impl Monoid for RAddRSumMonoid {
  type S = (i64, i64);
  fn identity() -> Self::S {
    (0, 0)
  }
  fn binary_operation((a, i): &Self::S, (b, j): &Self::S) -> Self::S {
    (a + b, i + j)
  }
}

#[snippet("rarsq")]
struct RAddRSumFunction;
impl MapMonoid for RAddRSumFunction {
  type M = RAddRSumMonoid;
  type F = i64;
  fn identity_map() -> Self::F {
    0
  }
  fn mapping(f: &Self::F, (x, i): &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
    (x + f * i, *i)
  }
  fn composition(f: &Self::F, g: &Self::F) -> Self::F {
    f + g
  }
}

#[snippet("rurmq")]
struct RUpdRMinMonoid;
impl Monoid for RUpdRMinMonoid {
  type S = i64;
  fn identity() -> Self::S {
    std::i64::MAX
  }
  fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
    *a.min(b)
  }
}

#[snippet("rurmq")]
struct RUpdRMinFunction;
impl MapMonoid for RUpdRMinFunction {
  type M = RUpdRMinMonoid;
  type F = (bool, i64);
  fn identity_map() -> Self::F {
    (false, 0)
  }
  fn mapping((upd, f): &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
    if *upd { *f } else { *x }
  }
  fn composition((fupd, f): &Self::F, (gupd, g): &Self::F) -> Self::F {
    if *fupd { (*fupd, *f) } else { (*gupd, *g) }
  }
}