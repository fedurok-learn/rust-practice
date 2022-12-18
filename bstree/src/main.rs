use std::fmt;
use std::mem;

type Link = Option<Box<Node>>;

struct Node {
  age: u32,
  name: String,
  left: Link,
  right: Link,
}

impl Node {
  pub fn new(age: u32, name: &String) -> Node {
    Self { age, name: name.clone(), left: Link::None, right: Link::None }
  }

  pub fn link(age: u32, name: &String) -> Link {
    Link::Some(Box::new(Self::new(age, name)))
  }

  pub fn equal(&self, age: u32, name: &String) -> bool {
    self.age == age && self.name == *name
  }
}

pub struct Tree {
  root: Link,
}

impl Tree {
  pub fn empty() -> Tree {
    Self { root: Link::None }
  }

  pub fn with_root(age: u32, name: &String) -> Tree{
    Self { root: Node::link(age, name) } 
  }

  pub fn insert(&mut self, age: u32, name: &String) { 
    let location = Self::locate_mut(&mut self.root, age, name);
    let inserted = match location.as_ref() {
      Some(_) => false, 
      None => {
        *location = Node::link(age, name);
        true
      }
    };
    
    if inserted { self.balance() }
  }

  pub fn erase(&mut self, age: u32, name: &String) {
    let location = Self::locate_mut(&mut self.root, age, name);
    let removed = match location.take() {
      Some(mut nd) => {
        match (nd.left.take(), nd.right.take()) {
          (None, None) => *location = None,
          (None, Some(ndr)) => *location = Some(ndr),
          (Some(ndl), None) => *location = Some(ndl),
          (Some(ndl), Some(ndr)) => {
            let node = &mut *nd;
            (node.left, node.right) = (Some(ndl), Some(ndr));
            let suc_link = Self::successor(&mut node.right);
            let suc_box = &mut suc_link.as_mut().unwrap();
            mem::swap(&mut node.age, &mut suc_box.age);
            mem::swap(&mut node.name, &mut suc_box.name);
            *suc_link = suc_box.right.take(); 
          },
        };
        true
      },
      None => false,
    };

    if removed { self.balance() }
  }

  pub fn balance(&mut self) {
    
  }


  pub fn contains(&self, age: u32, name: &String) -> bool {
    Self::locate(&self.root, age, name).is_some()
  }

  pub fn delete(&mut self) {
    self.root = Link::None;
  }
   
  // Helpers

  fn locate<'a>(link: &'a Link, age: u32, name: &String) -> &'a Link {
    match link {
      Some(nd) if !nd.equal(age, name) => {
        if age < nd.age || age == nd.age {
          Self::locate(&nd.left, age, name)
        } else {
          Self::locate(&nd.right, age, name)
        }
      }
      _ => link
    }
  }

  fn locate_mut<'a>(link: &'a mut Link, age: u32, name: &String) -> &'a mut Link {
    match link.as_ref() {
      Some(nd) if !nd.equal(age, name) => {
        let node = link.as_mut().unwrap();
        if age < node.age || age == node.age {
          Self::locate_mut(&mut node.left, age, name)
        } else {
          Self::locate_mut(&mut node.right, age, name)
        }  
      }
      _ => link,
    } 
  }

  fn successor(mut node: &mut Link) -> &mut Link {
    while let Some(nd) = node {
      node = &mut nd.left;
    }
    node
  }
}

// Print implementation
impl Tree {
  fn _print(link: &Link) -> String {
    match link {
      None => String::from("null"),
      Some(nd) => 
        format!("[{{\"{}\":\"{}\"}},{},{}]", 
          nd.age, nd.name, Self::_print(&nd.left), Self::_print(&nd.right)), 
    }
  }
}

impl fmt::Display for Tree  {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", Self::_print(&self.root))
  }
}

// Exec
fn main() {
  let mut root = Tree::with_root(32, &"Erik".to_string());
  root.insert(55, &"Mykola".to_string());  
  root.insert(12, &"Crypto".to_string());  
  root.insert(34, &"Luna".to_string());  
  root.insert(19, &"Terra".to_string());  
  root.insert(32, &"Victor".to_string());  

  println!("{root}\n");

  for (age, name) in [(55, "Mykola"), (12, "Crypto"), (55, "Skola")] {
    let contains = root.contains(age, &name.to_string());
    let msg = if contains { "contains" } else { "doesn't contain" };
    println!("Tree {} element {{{}:{}}}", msg, age, name);
  }

  root.erase(55, &"Mykola".to_string()); 
  println!("\n{root}\n");

  println!("The tree is {}", if is_sorted(&root.root) { "sorted" } else { "not sorted" });

  fn is_sorted(lnk: &Link) -> bool {
    match lnk {
      Some(nd) => {
        let res = match (&nd.left, &nd.right) {
          (None, None) => true,
          (Some(ndl), None) => ndl.age <= nd.age,
          (None, Some(ndr)) => nd.age < ndr.age,
          (Some(ndl), Some(ndr)) => ndl.age <= nd.age && nd.age < ndr.age,
        };
        res && is_sorted(&nd.left) && is_sorted(&nd.right)
      }
      None => true,
    } 
  }
}
