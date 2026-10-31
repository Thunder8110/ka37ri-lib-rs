use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[snippet]
pub mod arg_point {
  //! Sort points by their argument (angle) from the origin. [0 degrees, 360 degrees)
  //! Points at the origin are considered incomparable.
  //! # Example
  //! ```
  //! use crate::arg_point::ArgPoint;
  //! let deg0 = ArgPoint::from((1, 0));
  //! let deg45 = ArgPoint::from((1, 1));
  //! let deg90 = ArgPoint::from((0, 1));
  //! assert_eq!(deg0.partial_cmp(&deg45), std::cmp::Ordering::Less);
  //! assert_eq!(deg90.partial_cmp(&deg45), std::cmp::Ordering::Greater);
  //! assert_eq!(deg0.partial_cmp(&deg0), std::cmp::Ordering::Equal);
  //! ```
  use std::{cmp::Ordering, fmt::Debug};
  #[derive(Clone, Copy, Eq, Ord)]
  pub struct ArgPoint {
    x: i64,
    y: i64,
  }
  impl ArgPoint {
    fn is_origin(&self) -> bool {
      Into::<(i64, i64)>::into(*self) == (0, 0)
    }
    pub fn to_tuple(&self) -> (i64, i64) {
      (self.x, self.y)
    }
  }
  impl From<(i64, i64)> for ArgPoint {
    fn from(value: (i64, i64)) -> Self {
      Self { x: value.0, y: value.1 }
    }
  }
  impl Into<(i64, i64)> for ArgPoint {
    fn into(self) -> (i64, i64) {
      (self.x, self.y)
    }
  }
  impl PartialEq for ArgPoint {
    fn eq(&self, other: &Self) -> bool {
      self.partial_cmp(other) == Some(Ordering::Equal)
    }
  }
  impl PartialOrd for ArgPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      if self.is_origin() || other.is_origin() {
        return None;
      }
      let (x0, y0) = (self.x, self.y);
      let (x1, y1) = (other.x, other.y);
      Some(((y0, x0) < (0, 0))
        .cmp(&((y1, x1) < (0, 0)))
        .then_with(|| (x1 * y0).cmp(&(x0 * y1))))
    }
  }
  impl Debug for ArgPoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "@{:?}", Into::<(i64, i64)>::into(*self))
    }
  }
}

#[cfg(test)]
mod arg_point_test {
  use std::cmp::Ordering::{Equal, Greater, Less};
  use itertools::Itertools;

  use crate::arg_point::ArgPoint;

  #[test]
  fn test() {
    
    let z = ArgPoint::from((0, 0));
    let p = make_arg_point_vec(&[(0, 1), (1, 1), (-1, 1), (0, -1), (2, 2), (0, -3), (0, -4), (2, 0), (1, 2), (1, 1)]);

    // let mut s = p.clone();
    // s.sort_by(|x, y| x.partial_cmp(y).unwrap());

    // eprintln!("{:?}", s);
    assert_eq!(p[0].partial_cmp(&p[1]), Some(Greater));
    assert_eq!(p[1].partial_cmp(&p[0]), Some(Less));
    assert_eq!(p[2].partial_cmp(&p[3]), Some(Less));
    assert_eq!(p[3].partial_cmp(&p[3]), Some(Equal));
    assert_eq!(p[1].partial_cmp(&p[4]), Some(Equal));
    assert!(p[3] == p[3]);
    assert!(p[1] == p[9]);
    assert!(p[1] == p[4]);
    assert!(p[4] != p[8]);
    assert_eq!(p[5].partial_cmp(&z), None);
    assert_eq!(z.partial_cmp(&p[6]), None);
    assert_eq!(z.partial_cmp(&z), None);
  }

  #[test]
  fn sort_test() {
    let mut p = Vec::new();
    p.push(ArgPoint::from((-3, 0)));
    p.push(ArgPoint::from((3, 0)));
    for x in -2i64..=2 {
      p.push(ArgPoint::from((x, 3 - x.abs())));
      p.push(ArgPoint::from((x, -3 + x.abs())));
    }
    p.sort();
    let exp: Vec<ArgPoint> = make_arg_point_vec(&[(3, 0), (2, 1), (1, 2), (0, 3), (-1, 2), (-2, 1), (-3, 0), (-2, -1), (-1, -2), (0, -3), (1, -2), (2, -1)]);
    assert_eq!(p, exp);
  }

  #[test]
  fn group_test() {
    let mut p = Vec::new();
    for x in -2i64..=2 {
      for y in -2i64..=2 {
        if x == 0 && y == 0 {
          continue;
        }
        p.push(ArgPoint::from((x, y)));
      }
    }
    p.sort();
    let mut groups = p.chunk_by(|a, b| a == b).map(|g| g.to_vec()).collect_vec();
    groups.iter_mut().for_each(|g| g.sort_by_key(|e| e.to_tuple()));
    let exp: Vec<Vec<ArgPoint>> = vec![
      make_arg_point_vec(&[(1, 0), (2, 0)]),
      make_arg_point_vec(&[(2, 1)]),
      make_arg_point_vec(&[(1, 1), (2, 2)]),
      make_arg_point_vec(&[(1, 2)]),
      make_arg_point_vec(&[(0, 1), (0, 2)]),
      make_arg_point_vec(&[(-1, 2)]),
      make_arg_point_vec(&[(-2, 2), (-1, 1)]),
      make_arg_point_vec(&[(-2, 1)]),
      make_arg_point_vec(&[(-2, 0), (-1, 0)]),
      make_arg_point_vec(&[(-2, -1)]),
      make_arg_point_vec(&[(-2, -2), (-1, -1)]),
      make_arg_point_vec(&[(-1, -2)]),
      make_arg_point_vec(&[(0, -2), (0, -1)]),
      make_arg_point_vec(&[(1, -2)]),
      make_arg_point_vec(&[(1, -1), (2, -2)]),
      make_arg_point_vec(&[(2, -1)]),
    ];
    assert_eq!(groups, exp);
  }

  fn make_arg_point_vec(points: &[(i64, i64)]) -> Vec<ArgPoint> {
    points.iter().copied().map_into::<ArgPoint>().collect::<Vec<_>>()
  }
}
