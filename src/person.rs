use serde::{Serialize, Deserialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Person {
  name: String,
  age: u16,
  email: String,
}

impl Person {
  pub fn new(name: String, age: u16, email: String) -> Person {
    Person { name, age, email }
  }
}

// impl std::fmt::Debug for Person {
//   fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//     write!(f, "{} ({}); {}", self.name, self.age, self.email)
//   }
// }

impl std::fmt::Display for Person {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name)
  }
}
