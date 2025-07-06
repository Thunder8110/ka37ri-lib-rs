#[cfg(test)]
mod tests {
  use proconio::{input, source};
  use crate::macros::queries;

  queries!(
    1 => Add(x: usize, y: u32),
    2 => Remove(x: usize),
    3 => Query(),
  );
  
  #[test]
  fn test_queries() {
    let _source_string = "\
    8
    1 1 10
    1 2 20
    2 1
    3
    1 2 30
    3
    1 3 40
    3
    ";
    input! {
      q: usize,
    }
    let queries = input_queries(q);
    assert_eq!(queries.len(), q);
    let exp = vec![
      Query::Add(1, 10),
      Query::Add(2, 20),
      Query::Remove(1),
      Query::Query(),
      Query::Add(2, 30),
      Query::Query(),
      Query::Add(3, 40),
      Query::Query(),
    ];
    assert_eq!(queries, exp);
  }
}

mod macros {
  use cargo_snippet::snippet;
  
  
  #[snippet("queries")]
  #[macro_export]
  macro_rules! queries {
    ($($query_type:expr => $name:ident ($($element:ident : $values:tt),*)),*,) => {
      #[derive(Debug, Clone, Eq, PartialEq)]
      enum Query {
        $(
          $name($($values),*),
        )*
      }
      fn input_query() -> Query {
        input!(query_type: u8);
        match query_type {
          $(
            $query_type => {
              input!(
                $(
                  $element: $values,
                )*
              );
              Query::$name(
                $(
                  $element,
                )*
              )
            }
          ),*,
          _ => unreachable!(),
        }
      }
      fn input_queries(q: usize) -> Vec<Query> {
        let mut queries = Vec::with_capacity(q);
        for _ in 0..q {
          queries.push(input_query());
        }
        queries
      }
    };
  }

  pub(crate) use queries;
}

fn main() {
  unimplemented!();
}