use std::fmt;

#[derive(Clone)]
enum Address {
  None,
  Some(Box<Node>),
}

impl Address {
  pub fn val_addr(val: i32, addr: Address) -> Address {
    Self::Some(Box::new(Node { val: val, next: addr }))
  } 

  pub fn val(val: i32) -> Address {
    Self::val_addr(val, Self::None)
  }

  pub fn empty() -> Address { Self::None }
}

#[derive(Clone)]
struct Node {
  val: i32,
  next: Address
}

impl Node {
  pub fn push(&mut self, val: i32) {
    match self.next {
      Address::None => self.next = Address::val(val),
      Address::Some(ref mut next) => next.push(val),
    }
  }

  pub fn contains(&self, val: i32) -> bool {
    if val == self.val { 
      true 
    } else {
      match self.next {
        Address::None => false,
        Address::Some(ref next) => next.contains(val),
      }
    }
  }

  pub fn remove(&mut self, val: i32) -> bool {
    match self.next {
      Address::None => false,
      Address::Some(ref mut next) => {
        if val == next.val {
          self.next = next.next.clone();
          true
        } else {
          next.remove(val);   
          false
        }
      }
    }
  }

  pub fn print(&self) {
    self._print();
    println!();
  }

  fn _print(&self) {
    print!("{} ", self.val);
    match self.next {
      Address::None => return,
      Address::Some(ref next) => next._print(),
    }
  }
}

impl fmt::Display for Node {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "(todo Node print)")
  }
}

fn main() {
  let mut root = Node { val: 1, next: Address::None }; 
  root.push(2);
  root.push(3);
  root.push(4);
  root.push(5);
  root.print();
  root.remove(4);
  root.print();

  for el in 0..=5 {
    println!("el: {}, contains: {}", el, root.contains(el));
  }

  println!("{root}");
}
