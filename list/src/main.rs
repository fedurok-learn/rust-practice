use std::fmt;
use std::mem;

type Address = Option<Box<Node>>;

struct List {
  head: Address,
}

struct Node {
  val: i32,
  next: Address
}

impl List {
  pub fn new() -> Self {
    Self { head: None }
  } 
  
  pub fn push(&mut self, val: i32) {
    self.head = Some(Box::new(Node { 
      val: val, 
      next: self.head.take() 
    }));
  }

  pub fn pop(&mut self) -> Option<i32> {
    self.head.take().map(|nd| {
      self.head = nd.next;
      nd.val
    })
  }

  pub fn print(&self) {
    let mut cur_link = &self.head;
    while let Some(ref nd) = cur_link {
      print!("{} ", nd.val);
      cur_link = &nd.next;
    }
    println!();
  }
}

fn main() {
  let mut list = List::new();
  list.push(2);
  list.push(3);
  list.push(4);
  list.push(5);
  list.print();
  list.pop();
  list.print();
  list.pop();
  list.print();
}
