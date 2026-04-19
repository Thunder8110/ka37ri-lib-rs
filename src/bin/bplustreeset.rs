use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
  use crate::bplustreeset::BPlusTreeSet;
  #[test]
  fn test() {
    let mut bptset = BPlusTreeSet::new();
    bptset.insert(1);
    let n = bptset.find_node(&1);
    dbg!(n);
  }
}

#[snippet("bptset")]
pub mod bplustreeset {
  pub struct BPlusTreeSet<T: Ord> {
    root: Option<Box<Node<T>>>,
  }

  impl<T: Ord> BPlusTreeSet<T> {
    pub fn new() -> Self {
      BPlusTreeSet { root: None }
    }

    pub fn insert(&mut self, key: T) -> bool {
      if self.root.is_none() {
        self.root = Some(Box::new(Node {
          key: [Some(key), None],
          children: [None, None, None],
          child_len: [0, 0, 0],
          is_leaf: true,
        }));
        return true;
      } else {
        let node = &mut self.find_node(&key).unwrap();
        if node.key[1].is_some() {

        } else {
          if node.key[0].as_ref().unwrap() < &key {
            node.key[1] = Some(key);
          } else {
            node.key[1] = node.key[0];
            node.key[0] = Some(key);
          }
        }
        todo!();
      }
        
    }

    pub fn find_node(&self, key: &T) -> Option<&Box<Node<T>>> {
      if self.root.is_none() {
        return None;
      } else {
        let mut node = self.root.as_ref().unwrap();
        'look_down: loop {
          if node.is_leaf {
            return Some(node);
          }
          for i in 0..2 {
            if node.key[i].as_ref().unwrap() > key {
              node = node.children[i].as_ref().unwrap();
              continue 'look_down;
            }
          }
          node = node.children[2].as_ref().unwrap();
        }
      }
    }
    
  }

  #[derive(Debug)]
  pub struct Node<T: Ord> {
    key: [Option<T>; 2],
    children: [Option<Box<Node<T>>>; 3],
    child_len: [usize; 3],
    is_leaf: bool,
  }
}