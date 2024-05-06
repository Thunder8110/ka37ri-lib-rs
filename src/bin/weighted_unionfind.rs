use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

// modified dsu in ac-library-rs
#[snippet("wuf")]
pub mod weightedunionfind {
  pub struct WeightedUnionFind {
    n: usize,
    parent_or_size: Vec<i32>,
    weight: Vec<i64>,
  }
  
  impl WeightedUnionFind {
    pub fn new(size: usize) -> Self {
      Self { n: size, parent_or_size: vec![-1; size], weight: vec![0; size] }
    }
    pub fn merge(&mut self, a: usize, b: usize, w: i64) -> Result<usize, ()> {
      assert!(a < self.n);
      assert!(b < self.n);
      let (mut x, mut y) = (self.leader(a), self.leader(b));
      if x == y {
        return if self.diff(a, b) == Some(w) { Ok(x) } else { Err(()) };
      }
      if -self.parent_or_size[x] < -self.parent_or_size[y] {
        std::mem::swap(&mut x, &mut y);
      }
      self.parent_or_size[x] += self.parent_or_size[y];
      self.parent_or_size[y] = x as i32;
      self.weight[y] = self.weight[a] - self.weight[b] - w;
      Ok(x)
    }
    pub fn same(&mut self, a: usize, b: usize) -> bool {
      assert!(a < self.n);
      assert!(b < self.n);
      self.leader(a) == self.leader(b)
    }
    pub fn leader(&mut self, a: usize) -> usize {
      assert!(a < self.n);
      if self.parent_or_size[a] < 0 {
        return a;
      }
      let r = self.leader(self.parent_or_size[a] as usize);
      self.weight[a] += self.weight[self.parent_or_size[a] as usize];
      self.parent_or_size[a] = r as i32;
      r
    }
    pub fn size(&mut self, a: usize) -> usize {
      assert!(a < self.n);
      let x = self.leader(a);
      -self.parent_or_size[x] as usize
    }
    pub fn groups(&mut self) -> Vec<Vec<usize>> {
      let mut leader_buf = vec![0; self.n];
      let mut group_size = vec![0; self.n];
      for i in 0..self.n {
        leader_buf[i] = self.leader(i);
        group_size[leader_buf[i]] += 1;
      }
      let mut result = vec![Vec::new(); self.n];
      for i in 0..self.n {
        result[i].reserve(group_size[i]);
      }
      for i in 0..self.n {
        result[leader_buf[i]].push(i);
      }
      result
        .into_iter()
        .filter(|x| !x.is_empty())
        .collect::<Vec<Vec<usize>>>()
    }
    pub fn diff(&mut self, a: usize, b: usize) -> Option<i64> {
      if self.leader(a) == self.leader(b) {
        Some(self.weight[a] - self.weight[b])
      } else {
        None
      }
    }
  }
}
