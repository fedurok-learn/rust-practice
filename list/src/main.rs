type Address = Option<Box<Node>>;

fn some_addr(val: i32, addr: Address) -> Address {
  Some(Box::new(Node { val: val, next: addr }))
} 
fn null_addr() -> Address { None }

struct Node {
  val: i32,
  next: Address
}

impl Node {
  fn add_to_end(&mut self, val: i32) {
    match &mut self.next {
      None => self.next = some_addr(val, null_addr()),
      Some(nd) => nd.add_to_end(val),
    }
  }

  fn print(&self) {
    self._print();
    println!();
  }

  fn _print(&self) {
    print!("{} ", self.val);
    match &self.next {
      None => return,
      Some(nd) => nd._print(),
    }
  }
}

fn main() {
  let mut root = Node { val: 5, next: None };
  root.add_to_end(6);
  root.add_to_end(6);
  root.add_to_end(6);
  root.add_to_end(6);
  root.add_to_end(6);
  root.add_to_end(6);
  root.print();
}

