use std::{cmp::{Ordering, Reverse}, collections::BinaryHeap};

use cargo_snippet::snippet;

#[snippet("pf")]
fn prime_factor(n: i64) -> Vec<(i64, u32)> {
  let mut x = n;
  let mut res = Vec::new();
  for i in 2..n {
    let mut e = 0;
    if i * i > n { break; }
    while x % i == 0 {
      x /= i;
      e += 1;
    }
    if e > 0 {
      res.push((i, e))
    }
  }
  if x > 1 { res.push((x, 1)); }
  res
}

#[snippet("ert")]
fn eratosthenes(n: usize) -> Vec<usize> {
  let mut arr = vec![true; n + 1];
  arr[0] = false;
  arr[1] = false;
  let mut i = 2;
  while i * i <= n {
    if arr[i] {
      for x in i..=(n/i) {
        arr[i*x] = false;
      }
    }
    i += 1;
  }
  let mut primes = Vec::new();
  for j in 0..=n {
    if arr[j] { primes.push(j); }
  }
  primes
}

#[snippet("rl")]
fn run_length(s: String) -> Vec<(char, usize)> {
  let mut chs = s.chars();
  let mut res = Vec::new();
  let mut curr = chs.nth(0).unwrap();
  let mut num = 1;
  for c in chs {
    if c == curr {
      num += 1;
    }else {
      res.push((curr, num));
      curr = c;
      num = 1;
    }
  }
  res.push((curr, num));
  res
}

#[snippet("bssr")]
fn bs_sqrt(n: i64) -> i64 {
  let mut right = 1;
  while right * right < n {
    right <<= 1;
    if right >= (1 << 31) {
      right = 3037000500;
      break;
    }
  }
  let mut left = right >> 1;
  while right - left > 1 {
    let mid = (left + right) / 2;
    let m_sq = mid * mid;
    if m_sq == n {
      return mid;
    }else if m_sq > n {
      right = mid;
    }else {
      left = mid;
    }
  }
  left
}

#[snippet("ipt_sp")]
fn input() -> String {
  let mut buffer = String::new();
  std::io::stdin().read_line(&mut buffer).unwrap();
  buffer.trim().to_string()
}

#[snippet("bsi")]
fn binary_search_integer(left: u64, right: u64, pred: fn(u64) -> bool) -> u64 {
  let mut l = left;
  let mut r = right;
  while r - l > 1 {
    let m = l + (r - l) / 2;
    if pred(m) {
      l = m;
    }else {
      r = m;
    }
  }
  l
}

#[snippet("dijk")]
fn dijkstra(n: usize, graph: &Vec<Vec<(usize, u64)>>, s: usize) -> Vec<u64> {
  let mut queue = BinaryHeap::new();
  queue.push(Reverse((0, s)));
  let mut visited = vec![false; n];
  let mut dist = vec![u64::MAX; n];
  while let Some(Reverse((cc, cv))) = queue.pop() {
    if visited[cv] {
      continue;
    }
    visited[cv] = true;
    dist[cv] = cc;
    for &(nxv, nxc) in graph[cv].iter() {
      if visited[nxv] {
        continue;
      }
      queue.push(Reverse((cc + nxc, nxv)));
    }
  }
  dist
}

#[snippet("euc", include = "eucp")]
fn ext_euc(a: i64, b: i64) -> (i64, i64, i64) {
  assert!(a != 0 && b != 0);
  let (x, y, z) = ext_euc_pos(a.abs(), b.abs());
  match (a.cmp(&0), b.cmp(&0)) {
    (Ordering::Greater, Ordering::Greater) => (x, y, z),
    (Ordering::Greater, Ordering::Less) => (x, -y, z),
    (Ordering::Less, Ordering::Greater) => (-x, y, z),
    (Ordering::Less, Ordering::Less) => (-x, -y, z),
    _ => panic!(),
  }
}

#[snippet("eucp")]
fn ext_euc_pos(a: i64, b: i64) -> (i64, i64, i64) {
  assert!(a > 0 && b > 0);
  let (mut xp, mut xc) = (1,  0);
  let (mut yp, mut yc) = (0,  1);
  let (mut zp, mut zc) = (a,  b);
  loop {
    let q = zp / zc;
    (zp, zc) = (zc, zp - q * zc);
    if zc == 0 { break (xc, yc, zp); }
    (xp, xc) = (xc, xp - q * xc);
    (yp, yc) = (yc, yp - q * yc);
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn pf_test() {
    let res0 = prime_factor(998244352);
    let exp0 = vec![(2, 23), (7, 1), (17, 1)];
    assert_eq!(res0, exp0);
  }
  #[test]
  fn era_test() {
    let res0 = eratosthenes(101);
    assert_eq!(res0[0], 2);
    assert_eq!(res0[10], 31);
    assert_eq!(*res0.last().unwrap(), 101);
    assert_eq!(res0.len(), 26);
  }
  // #[test]
  // fn era_long_test() {
  //   let res0 = eratosthenes(1000000000);
  //   assert!(!res0.is_empty());
  // }
  #[test]
  fn rl_test() {
    let res0 = run_length("aaabbccacb".to_string());
    let exp0 = vec![('a', 3), ('b', 2), ('c', 2), ('a', 1), ('c', 1), ('b', 1)];
    let res1 = run_length("1234".to_string());
    let exp1 = vec![('1', 1), ('2', 1), ('3', 1), ('4', 1)];
    assert_eq!(res0, exp0);
    assert_eq!(res1, exp1);
  }
  #[test]
  fn sr_test() {
    let res0 = bs_sqrt(25);
    assert_eq!(res0, 5);
    let res1 = bs_sqrt(1000);
    assert_eq!(res1, 31);
    let res2 = bs_sqrt(i64::MAX);
    assert!(res2 * res2 <= i64::MAX);
  }
  #[test]
  fn in_test() {
    let res0 = input();
    let res1: Vec<String> = input().split_whitespace().into_iter().map(|s| s.to_string()).collect();
    println!("{}", res0);
    println!("{:?}", res1);
  }
}
