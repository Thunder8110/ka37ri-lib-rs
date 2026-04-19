use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
  #[test]
  fn test() {
  }

  #[test]
  fn test_groups() {
    use crate::iterators::Groups;
    let res0 = Groups::groups(1).collect::<Vec<_>>();
    let exp0 = vec![vec![vec![0]]];
    assert_eq!(res0, exp0);

    let mut res1 = Groups::groups(2).collect::<Vec<_>>();
    res1.sort();
    let mut exp1 = vec![
      vec![vec![0], vec![1]],
      vec![vec![0, 1]],
    ];
    exp1.sort();
    assert_eq!(res1, exp1);

    let mut res2 = Groups::groups(3).collect::<Vec<_>>();
    res2.sort();
    let mut exp2 = vec![
      vec![vec![0], vec![1], vec![2]],
      vec![vec![0, 1], vec![2]],
      vec![vec![0, 2], vec![1]],
      vec![vec![0], vec![1, 2]],
      vec![vec![0, 1, 2]],
    ];
    exp2.sort();
    assert_eq!(res2, exp2);

    let res3 = Groups::groups(4).count();
    let res4 = Groups::groups(5).count();
    let res5 = Groups::groups(6).count();
    let res6 = Groups::groups(7).count();
    assert_eq!(res3, 15);
    assert_eq!(res4, 52);
    assert_eq!(res5, 203);
    assert_eq!(res6, 877);
  }
}

pub mod iterators {
  use cargo_snippet::snippet;

  #[snippet("groups")]
  pub struct Groups {
    n: usize,
    stack: Vec<(bool, usize, usize)>,
    groups: Vec<Vec<usize>>,
    right: usize,
  }
  #[snippet("groups")]
  impl Groups {
    pub fn groups(n: usize) -> Self {
      assert!(n > 0, "n must be greater than 0");
      Self {
        n,
        stack: vec![(false, 0, 0), (true, 0, 0)],
        groups: vec![vec![]; n],
        right: 1,
      }
    }
  }
  #[snippet("groups")]
  impl Iterator for Groups {
    type Item = Vec<Vec<usize>>;
    fn next(&mut self) -> Option<Self::Item> {
      while let Some((dfs, i, k)) = self.stack.pop() {
        if dfs {
          if self.groups[k].len() == 0 {
            self.right += 1;
          }
          self.groups[k].push(i);
          if i == self.n - 1 {
            let mut res = self.groups.clone();
            while res.last().is_some_and(|v| v.len() == 0) {
              res.pop();
            }
            return Some(res);
          } else {
            for j in 0..self.right {
              if j > self.n - 1 {
                continue;
              }
              self.stack.push((false, i + 1, j));
              self.stack.push((true, i + 1, j));
            }
          }
        } else {
          self.groups[k].pop();
          if k == self.right - 2 && self.groups[k].len() == 0 {
            self.right -= 1;
          }
        }
      }
      None
    }
  }
}