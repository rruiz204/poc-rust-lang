struct Record {
  pub active: bool,
  pub prototype: i32,
}

impl Record {
  pub fn new(active: bool, prototype: i32) -> Self {
    Self { active, prototype }
  }

  pub fn set_active(&mut self, active: bool) {
    self.active = active;
  }

  pub fn set_prototype(&mut self, prototype: i32) {
    self.prototype = prototype;
  }

  pub fn display(&self) {
    println!("Active: {}", self.active);
    println!("Prototype: {}", self.prototype);
  }
}

pub fn showcase() {
  println!("=== Structs and Implementation Blocks");
  let mut rc1: Record = Record::new(true, 101);

  rc1.display();
  rc1.set_active(false);
  
  rc1.set_prototype(102);
  rc1.display();
}