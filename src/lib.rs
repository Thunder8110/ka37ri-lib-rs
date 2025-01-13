use std::{cmp::{Ordering, Reverse}, collections::{BinaryHeap, HashMap}};

use cargo_snippet::snippet;
use num::{Num, One};

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
fn run_length<T: Eq + Clone>(seq: impl IntoIterator<Item = T>) -> Vec<(T, usize)> {
  let mut itr = seq.into_iter().peekable();
  let mut res = Vec::new();
  let mut curr = itr.peek().unwrap().clone();
  let mut num = 0;
  for c in itr {
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
fn binary_search_integer<T>(left: T, right: T, pred: impl Fn(T) -> bool) -> T
  where T: Num + PartialOrd + One + Copy
{
  let one = T::one();
  let two = one + one;
  let mut l = left;
  let mut r = right;
  while r - l > one {
    let m = l + (r - l) / two;
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

#[snippet("acc")]
fn accumulate<T>(iter: impl Iterator<Item = T>, init: T, unit: T, func: impl Fn(T, T) -> T) -> impl Iterator<Item = T>
  where T: Clone
{
  std::iter::once(unit).chain(iter).scan(init, move |acc, x| {
    *acc = func(acc.clone(), x);
    Some(acc.clone())
  })
}

#[snippet("prs")]
fn pattern_range_sum(patt: &[(i64, usize)], n: usize) -> i64 {
  let sm = patt.iter().map(|&(x, y)| x * y as i64).sum::<i64>();
  let sz = patt.iter().map(|&(_, y)| y).sum::<usize>();
  let mut res = sm * (n as i64 / sz as i64);
  let mut rem = n % sz;
  for &(x, y) in patt.iter() {
    if rem < y {
      res += x * rem as i64;
      break;
    }else {
      res += x * y as i64;
      rem -= y;
    }
  }
  res
}

#[snippet("2dps")]
fn two_dimensional_prefix_sum(arr: &[Vec<i64>]) -> Vec<Vec<i64>> {
  let h = arr.len();
  let w = arr[0].len();
  let mut psum = vec![vec![0; w + 1]; h + 1];
  for i in 0..h {
    for j in 0..w {
      psum[i + 1][j + 1] = arr[i][j] + psum[i + 1][j] + psum[i][j + 1] - psum[i][j];
    }
  }
  psum
}

#[snippet("pas", include = "2dps")]
fn pattern_area_sum(psum: &[Vec<i64>], h: usize, w: usize) -> i64 {
  let ph = psum.len() - 1;
  let pw = psum[0].len() - 1;
  let area1 = (h / ph) as i64 * (w / pw) as i64 * psum[ph][pw];
  let area2 = (h / ph) as i64 * psum[ph][w % pw];
  let area3 = (w / pw) as i64 * psum[h % ph][pw];
  let area4 = psum[h % ph][w % pw];
  area1 + area2 + area3 + area4
}

#[snippet("i2s", include = "i2sp")]
fn int_to_str(n: i64, b: i64) -> String {
  assert!(2 <= b && b <= 36, "base must be between 2 and 36");
  if n < 0 {
    format!("-{}", int_to_str_pos(-n, b))
  } else {
    int_to_str_pos(n, b)
  }
}

#[snippet("i2sp", include = "rds")]
fn int_to_str_pos(n: i64, b: i64) -> String {
  assert!(2 <= b && b <= 36, "base must be between 2 and 36");
  assert!(n >= 0, "n must be non-negative");
  const DIGITS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  if n == 0 {
    "0".to_string()
  } else {
    radix_seq(n, b).iter().rev().map(|&x| DIGITS[x as usize] as char).collect()
  }
}

#[snippet("rds")]
fn radix_seq(n: i64, b: i64) -> Vec<i64> {
  let mut x = n;
  std::iter::from_fn(move || {
    if x == 0 {
      None
    } else {
      let r = x % b;
      x /= b;
      Some(r)
    }
  }).collect::<Vec<_>>()
}

#[snippet("s2i", include = "s2ip")]
fn str_to_int(s: String, b: i64) -> i64 {
  if s.starts_with('-') {
    -str_to_int_pos(&s[1..], b)
  } else if s.starts_with('+') {
    str_to_int_pos(&s[1..], b)
  } else {
    str_to_int_pos(&s, b)
  }
}

#[snippet("s2ip")]
fn str_to_int_pos(s: &str, b: i64) -> i64 {
  assert!(2 <= b && b <= 36, "base must be between 2 and 36");
  assert!(s.chars().all(|c| c.is_digit(b as u32) || c.is_ascii_alphabetic()), "invalid character");
  let chars = s.chars();
  let mut res = 0;
  for c in chars {
    let d = match c {
      '0'..='9' => c as i64 - '0' as i64,
      'A'..='Z' => c as i64 - 'A' as i64 + 10,
      _ => panic!("invalid character"),
    };
    assert!(d < b, "invalid character");
    res = res * b + d;
  }
  res
}

#[snippet("its")]
fn intersection((a, b): (i64, i64), (c, d): (i64, i64)) -> Option<(i64, i64)> {
  assert!(a <= b && c <= d, "invalid range");
  let (l, r) = (a.max(c), b.min(d));
  if l < r {
    Some((l, r))
  } else {
    None
  }
}

#[snippet("revs")]
fn reversed_str(s: &String) -> String {
  s.chars().rev().collect()
}

#[snippet("ipar")]
fn is_parindrome(s: &String) -> bool {
  s.chars().eq(s.chars().rev())
}

#[snippet("trns")]
fn translate(s: &String, map: &HashMap<char, Option<char>>) -> String {
  s.chars().filter_map(|c| map.get(&c).copied().unwrap_or(Some(c))).collect()
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
    let res0 = run_length("aaabbccacb".to_owned().chars());
    let exp0 = vec![('a', 3), ('b', 2), ('c', 2), ('a', 1), ('c', 1), ('b', 1)];
    let res1 = run_length("1234".to_owned().chars());
    let exp1 = vec![('1', 1), ('2', 1), ('3', 1), ('4', 1)];
    let seq2 = vec![1, 1, 1, 1, 1, 2, 2, 1, 1, 1];
    let res2 = run_length(seq2);
    let exp2 = vec![(1, 5), (2, 2), (1, 3)];
    let seq3 = vec![9, 9, 8, 2, 4, 4, 3, 5, 3];
    let res3 = run_length(seq3.iter().copied());
    let exp3 = vec![(9, 2), (8, 1), (2, 1), (4, 2), (3, 1), (5, 1), (3, 1)];
    assert_eq!(res0, exp0);
    assert_eq!(res1, exp1);
    assert_eq!(res2, exp2);
    assert_eq!(res3, exp3);
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
  #[test]
  fn acc_test() {
    let res0: Vec<i64> = accumulate(0..10, 0, 0, |a, b| a + b).collect();
    let exp0 = vec![0, 0, 1, 3, 6, 10, 15, 21, 28, 36, 45];
    assert_eq!(res0, exp0);
    let res1: Vec<i64> = accumulate([2i64, 3, 5, 7].iter().copied(), 1, 0, |a, b| a + b).collect();
    let exp1 = vec![1, 3, 6, 11, 18];
    assert_eq!(res1, exp1);
    let res2: Vec<i64> = accumulate([2i64, 3, 5, 7].iter().copied(), 1, 1, |a, b| a * b).collect();
    let exp2 = vec![1, 2, 6, 30, 210];
    assert_eq!(res2, exp2);
  }

  #[test]
  fn prs_test() {
    //                  10|         16|       21|
    // 0 0 0 0 1 1 1 1 0 0 0 0 1 1 1 1 0 0 0 0 1 1 1 1 0 0 0 0 1 1 1 1
    let pat0 = vec![(0, 4), (1, 4)];
    let res0 = pattern_range_sum(&pat0, 10);
    let res1 = pattern_range_sum(&pat0, 16);
    let res2 = pattern_range_sum(&pat0, 21);
    assert_eq!(res0, 4);
    assert_eq!(res1, 8);
    assert_eq!(res2, 9);

    // 1|            8|
    // 1 1 1 2 2 3 1 1 1 2 2 3
    let pat1 = vec![(1, 3), (2, 2), (3, 1)];
    let res3 = pattern_range_sum(&pat1, 1);
    let res4 = pattern_range_sum(&pat1, 8);
    assert_eq!(res3, 1);
    assert_eq!(res4, 12);

    let pat2 = vec![(1, 1)];
    let res5 = pattern_range_sum(&pat2, 100);
    let res6 = pattern_range_sum(&pat2, 0);
    assert_eq!(res5, 100);
    assert_eq!(res6, 0);
  }

  #[test]
  fn pas_test() {
    let arr0 = vec![
      vec![1, 2, 3, 4],
      vec![5, 6, 7, 8],
      vec![9, 10, 11, 12],
    ];
    let psum0 = two_dimensional_prefix_sum(&arr0);
    let res0 = pattern_area_sum(&psum0, 3, 4);
    let res1 = pattern_area_sum(&psum0, 6, 8);
    let res2 = pattern_area_sum(&psum0, 5, 1);
    let res3 = pattern_area_sum(&psum0, 1, 5);
    let res4 = pattern_area_sum(&psum0, 4, 5);
    let res5 = pattern_area_sum(&psum0, 20, 20);
    let res6 = pattern_area_sum(&psum0, 0, 0);
    let res7 = pattern_area_sum(&psum0, 0, 4);
    let res8 = pattern_area_sum(&psum0, 3, 0);
    assert_eq!(res0, 78);
    assert_eq!(res1, 78 * 4);
    assert_eq!(res2, 15 + 6);
    assert_eq!(res3, 10 + 1);
    assert_eq!(res4, 78 + 26);
    assert_eq!(res5, 78 * 30 + 36 * 5);
    assert_eq!(res6, 0);
    assert_eq!(res7, 0);
    assert_eq!(res8, 0);
  }

  #[test]
  fn i2s_test() {
    let res0 = int_to_str(12345, 10);
    let res1 = int_to_str(12345, 2);
    let res2 = int_to_str(12345, 8);
    let res3 = int_to_str(12345, 16);
    let res4 = int_to_str(12345, 36);
    assert_eq!(res0, "12345");
    assert_eq!(res1, "11000000111001");
    assert_eq!(res2, "30071");
    assert_eq!(res3, "3039");
    assert_eq!(res4, "9IX");

    let res5 = int_to_str(54321, 16);
    let res6 = int_to_str(54321, 36);
    let res7 = int_to_str(54321, 2);
    let res8 = int_to_str(54321, 3);
    let res9 = int_to_str(54321, 4);
    let res10 = int_to_str(54321, 5);
    let res11 = int_to_str(54321, 6);
    let res12 = int_to_str(54321, 7);
    let res13 = int_to_str(54321, 8);
    let res14 = int_to_str(54321, 9);
    let res15 = int_to_str(54321, 10);
    assert_eq!(res5, "D431");
    assert_eq!(res6, "15WX");
    assert_eq!(res7, "1101010000110001");
    assert_eq!(res8, "2202111220");
    assert_eq!(res9, "31100301");
    assert_eq!(res10, "3214241");
    assert_eq!(res11, "1055253");
    assert_eq!(res12, "314241");
    assert_eq!(res13, "152061");
    assert_eq!(res14, "82456");
    assert_eq!(res15, "54321");

    let res16 = int_to_str(0, 10);
    let res17 = int_to_str(0, 2);
    let res18 = int_to_str(0, 8);
    let res19 = int_to_str(0, 16);
    let res20 = int_to_str(0, 36);
    let res21 = int_to_str(0, 3);
    let res22 = int_to_str(0, 5);
    assert_eq!(res16, "0");
    assert_eq!(res17, "0");
    assert_eq!(res18, "0");
    assert_eq!(res19, "0");
    assert_eq!(res20, "0");
    assert_eq!(res21, "0");
    assert_eq!(res22, "0");

    let res23 = int_to_str(-12345, 10);
    let res24 = int_to_str(-12345, 2);
    let res25 = int_to_str(-12345, 8);
    let res26 = int_to_str(-12345, 16);
    let res27 = int_to_str(-12345, 36);
    assert_eq!(res23, "-12345");
    assert_eq!(res24, "-11000000111001");
    assert_eq!(res25, "-30071");
    assert_eq!(res26, "-3039");
    assert_eq!(res27, "-9IX");
  }

  #[test]
  fn s2i_test() {
    let res0 = str_to_int("12345".to_string(), 10);
    let res1 = str_to_int("11000000111001".to_string(), 2);
    let res2 = str_to_int("30071".to_string(), 8);
    let res3 = str_to_int("3039".to_string(), 16);
    let res4 = str_to_int("9IX".to_string(), 36);
    assert_eq!(res0, 12345);
    assert_eq!(res1, 12345);
    assert_eq!(res2, 12345);
    assert_eq!(res3, 12345);
    assert_eq!(res4, 12345);

    let res5 = str_to_int("D431".to_string(), 16);
    let res6 = str_to_int("15WX".to_string(), 36);
    let res7 = str_to_int("1101010000110001".to_string(), 2);
    let res8 = str_to_int("2202111220".to_string(), 3);
    let res9 = str_to_int("31100301".to_string(), 4);
    let res10 = str_to_int("3214241".to_string(), 5);
    let res11 = str_to_int("1055253".to_string(), 6);
    let res12 = str_to_int("314241".to_string(), 7);
    let res13 = str_to_int("152061".to_string(), 8);
    let res14 = str_to_int("82456".to_string(), 9);
    let res15 = str_to_int("54321".to_string(), 10);
    assert_eq!(res5, 54321);
    assert_eq!(res6, 54321);
    assert_eq!(res7, 54321);
    assert_eq!(res8, 54321);
    assert_eq!(res9, 54321);
    assert_eq!(res10, 54321);
    assert_eq!(res11, 54321);
    assert_eq!(res12, 54321);
    assert_eq!(res13, 54321);
    assert_eq!(res14, 54321);
    assert_eq!(res15, 54321);

    let res16 = str_to_int("0".to_string(), 10);
    let res17 = str_to_int("0".to_string(), 2);
    let res18 = str_to_int("0".to_string(), 8);
    let res19 = str_to_int("0".to_string(), 16);
    let res20 = str_to_int("0".to_string(), 36);
    let res21 = str_to_int("0".to_string(), 3);
    let res22 = str_to_int("0".to_string(), 5);
    assert_eq!(res16, 0);
    assert_eq!(res17, 0);
    assert_eq!(res18, 0);
    assert_eq!(res19, 0);
    assert_eq!(res20, 0);
    assert_eq!(res21, 0);
    assert_eq!(res22, 0);

    let res23 = str_to_int("-12345".to_string(), 10);
    let res24 = str_to_int("-11000000111001".to_string(), 2);
    let res25 = str_to_int("-30071".to_string(), 8);
    let res26 = str_to_int("-3039".to_string(), 16);
    let res27 = str_to_int("-9IX".to_string(), 36);
    assert_eq!(res23, -12345);
    assert_eq!(res24, -12345);
    assert_eq!(res25, -12345);
    assert_eq!(res26, -12345);
    assert_eq!(res27, -12345);

    let res28 = str_to_int("+12345".to_string(), 10);
    let res29 = str_to_int("+11000000111001".to_string(), 2);
    let res30 = str_to_int("+30071".to_string(), 8);
    let res31 = str_to_int("+3039".to_string(), 16);
    let res32 = str_to_int("+9IX".to_string(), 36);
    assert_eq!(res28, 12345);
    assert_eq!(res29, 12345);
    assert_eq!(res30, 12345);
    assert_eq!(res31, 12345);
    assert_eq!(res32, 12345);
  }

  #[test]
  fn strings_test() {
    let s = "abracadabra".to_owned();
    assert_eq!(reversed_str(&s), "arbadacarba");
    assert!(!is_parindrome(&s));

    let t = "abcba".to_owned();
    assert_eq!(reversed_str(&t), "abcba");
    assert!(is_parindrome(&t));

    let s = "abracadabra".to_owned();
    let map = [
      ('a', Some('b')),
    ].into();
    assert_eq!(translate(&s, &map), "bbrbcbdbbrb");
    let map = [
      ('a', None),
      ('b', Some('a')),
    ].into();
    assert_eq!(translate(&s, &map), "arcdar");
    let map = [
      ('a', Some('b')),
      ('b', Some('c')),
      ('c', Some('a')),
    ].into();
    assert_eq!(translate(&s, &map), "bcrbabdbcrb");
    
  }
}
