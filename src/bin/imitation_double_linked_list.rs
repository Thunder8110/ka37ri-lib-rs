use cargo_snippet::snippet;

fn main() {
  unimplemented!();
}

#[cfg(test)]
mod tests {
  use super::imitation_double_linked_list::ImitationDoubleLinkedList;
  #[test]
  fn test() {
    let mut list = ImitationDoubleLinkedList::new();
    list.push_back(1);
    list.push_back(2);
    list.push_front(0);
    list.insert_after(1, 1);
    list.insert_before(2, 1);
    assert_eq!(list.len(), 5);
    assert_eq!(list.remove(1), 1);
    assert_eq!(list.len(), 4);
    assert_eq!(list.iter().copied().collect::<Vec<_>>(), vec![0, 1, 1, 2]);
    assert_eq!(list.iter().rev().copied().collect::<Vec<_>>(), vec![2, 1, 1, 0]);
  }
}

#[snippet("idll")]
pub mod imitation_double_linked_list {
  use std::collections::HashMap;
  use std::hash::Hash;

  pub struct ImitationDoubleLinkedList<T> {
    data: Vec<T>,
    map: HashMap<T, usize>,
    next: Vec<Option<usize>>,
    prev: Vec<Option<usize>>,
    head: Option<usize>,
    tail: Option<usize>,
    len: usize,
  }
  
  impl<T: Clone + PartialEq + Eq + Hash> ImitationDoubleLinkedList<T> {
    pub fn new() -> Self {
      Self {
        data: Vec::new(),
        map: HashMap::new(),
        next: Vec::new(),
        prev: Vec::new(),
        head: None,
        tail: None,
        len: 0,
      }
    }
    pub fn new_data(&mut self, data: T) -> usize {
      let idx = self.data.len();
      self.map.insert(data.clone(), idx);
      self.data.push(data);
      self.next.push(None);
      self.prev.push(None);
      idx
    }
    pub fn push_back(&mut self, data: T) -> usize {
      let idx = self.new_data(data);
      match self.tail {
        Some(tail_idx) => {
          self.next[tail_idx] = Some(idx);
          self.prev[idx] = Some(tail_idx);
        },
        None => {
          self.head = Some(idx);
        },
      }
      self.tail = Some(idx);
      self.len += 1;
      idx
    }
    pub fn push_front(&mut self, data: T) -> usize {
      let idx = self.new_data(data);
      match self.head {
        Some(head_idx) => {
          self.prev[head_idx] = Some(idx);
          self.next[idx] = Some(head_idx);
        },
        None => {
          self.tail = Some(idx);
        },
      }
      self.head = Some(idx);
      self.len += 1;
      idx
    }
    pub fn insert_after(&mut self, idx: usize, data: T) -> usize {
      let new_idx = self.new_data(data);
      let next_idx = self.next[idx];
      self.next[idx] = Some(new_idx);
      self.prev[new_idx] = Some(idx);
      self.next[new_idx] = next_idx;
      if let Some(next_idx) = next_idx {
        self.prev[next_idx] = Some(new_idx);
      } else {
        self.tail = Some(new_idx);
      }
      self.len += 1;
      new_idx
    }
    pub fn insert_before(&mut self, idx: usize, data: T) -> usize {
      let new_idx = self.new_data(data);
      let prev_idx = self.prev[idx];
      self.prev[idx] = Some(new_idx);
      self.next[new_idx] = Some(idx);
      self.prev[new_idx] = prev_idx;
      if let Some(prev_idx) = prev_idx {
        self.next[prev_idx] = Some(new_idx);
      } else {
        self.head = Some(new_idx);
      }
      self.len += 1;
      new_idx
    }
    pub fn remove(&mut self, idx: usize) -> T {
      let data = self.data[idx].clone();
      self.map.remove(&data);
      let prev_idx = self.prev[idx];
      let next_idx = self.next[idx];
      if let Some(prev_idx) = prev_idx {
        self.next[prev_idx] = next_idx;
      } else {
        self.head = next_idx;
      }
      if let Some(next_idx) = next_idx {
        self.prev[next_idx] = prev_idx;
      } else {
        self.tail = prev_idx;
      }
      self.len -= 1;
      data
    }
    pub fn len(&self) -> usize {
      self.len
    }
    pub fn iter(&self) -> ImitationDoubleLinkedListIter<T> {
      ImitationDoubleLinkedListIter {
        list: self,
        idx: self.head,
        idx_back: self.tail,
      }
    }
  }
  
  pub struct ImitationDoubleLinkedListIter<'a, T> {
    list: &'a ImitationDoubleLinkedList<T>,
    idx: Option<usize>,
    idx_back: Option<usize>,
  }
  
  impl<'a, T: Clone> Iterator for ImitationDoubleLinkedListIter<'a, T> {
    type Item = &'a T;
  
    fn next(&mut self) -> Option<Self::Item> {
      self.idx.map(|idx| {
        let data = &self.list.data[idx];
        self.idx = self.list.next[idx];
        data
      })
    }
  }
  impl<'a, T: Clone> DoubleEndedIterator for ImitationDoubleLinkedListIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
      self.idx_back.map(|idx| {
        let data = &self.list.data[idx];
        self.idx_back = self.list.prev[idx];
        data
      })
    }
  }
}