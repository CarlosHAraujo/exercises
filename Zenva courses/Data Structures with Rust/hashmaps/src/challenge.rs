use std::collections::HashMap;

pub fn challenge() {
  let mut grades = HashMap::new();

  grades.insert("Bob", 5);
  grades.insert("Alice", 2);
  grades.insert("John", 4);

  match grades.get("Alice") {
    Some(&grade) => println!("Alice grade is: {}", grade),
    None => println!("Grade not found")
  }

  grades.insert("Alice", 5);

  grades.remove("John");

  for (key, val) in grades {
    println!("Student {} has grade {}", key, val);
  }
}
