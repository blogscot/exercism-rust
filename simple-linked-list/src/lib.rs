#[derive(Clone, PartialEq)]
struct Node<T> {
  data: T,
  next: Option<Box<Node<T>>>,
}

#[derive(Clone, PartialEq)]
pub struct SimpleLinkedList<T> {
  head: Option<Box<Node<T>>>,
}

impl<T> Default for SimpleLinkedList<T> {
  fn default() -> Self {
    SimpleLinkedList { head: None }
  }
}

impl<T> SimpleLinkedList<T> {
  pub fn new() -> Self {
    SimpleLinkedList { head: None }
  }

  pub fn len(&self) -> usize {
    match self.head {
      None => 0,
      _ => {
        let mut count = 0;
        let mut head = &self.head;
        while let Some(node) = head {
          head = &node.next;
          count += 1;
        }
        count
      }
    }
  }
  pub fn is_empty(&self) -> bool {
    self.len() == 0
  }
  pub fn push(&mut self, element: T) {
    let new_node = Some(Box::new(Node {
      data: element,
      next: self.head.take(),
    }));

    self.head = new_node;
  }

  pub fn pop(&mut self) -> Option<T>
  where
    T: Copy,
  {
    match self.head.take() {
      None => None,
      Some(node) => {
        let data = node.data;
        self.head = node.next;
        Some(data)
      }
    }
  }

  pub fn peek(&self) -> Option<&T> {
    match self.head {
      None => None,
      Some(ref node) => Some(&node.data),
    }
  }
}

impl<T: Copy + Clone> SimpleLinkedList<T> {
  pub fn rev(self) -> SimpleLinkedList<T> {
    let mut list: SimpleLinkedList<T> = SimpleLinkedList::new();
    let mut cloned_list = self.clone();

    while let Some(value) = cloned_list.pop() {
      list.push(value);
    }
    list
  }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
  fn from(item: &[T]) -> Self {
    let mut list = SimpleLinkedList::new();
    for value in item.iter().cloned() {
      list.push(value);
    }
    list
  }
}

impl<T: Copy + Clone> Into<Vec<T>> for SimpleLinkedList<T> {
  fn into(self) -> Vec<T> {
    let mut vec: Vec<T> = vec![];
    let mut cloned_list = self.clone();

    while let Some(value) = cloned_list.pop() {
      vec.push(value);
    }
    vec.reverse();
    vec
  }
}
