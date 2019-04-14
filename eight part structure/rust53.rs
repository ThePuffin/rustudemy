// exercice

#[derive(Debug)]
struct Student {
    name: String,
    rust: i32,
    javascript: i32,
    bootstrap: i32,
}

impl Student {
     fn build_student(name:String,rust:i32,javascript:i32,bootstrap:i32)->Student{
Student{
    name, rust, javascript, bootstrap}
     }
     fn compare(&self){
  if self.rust>self.javascript && self.rust>self.bootstrap{
      println!("Highest marks in rust");
      }
  else if self.javascript>self.rust && self.javascript>self.bootstrap{
      println!("Highest mark in javascript");
      }
  else {
      println!("Highest marks in bootstrap");
      }
     }
}

fn main() {
  let new_student = Student::build_student(String::from("Bob"), 5, 8, 1);
  new_student.compare();

}