#[allow(dead_code)] 
// #[allow(non_camel_case_types)]
#[derive(Debug)]
enum Fruits {
  Apple=0,
  Mango=10,
  Watermelon=20,
}



fn main() {
    let f=Fruits::Mango;
    println!("{:?}",f); //print the key not the value


    println!("{:?}",f as i32); //print the value
    //only work for integer not for boolean, string...
}
