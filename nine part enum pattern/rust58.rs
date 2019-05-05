//option Enum

fn main () {
 let num:Option<i32>=Some(5);

 let text:Option<&str>=Some("Hello");

 let other_variable_num:Option<i32>= None;

// //not possible because value could be None
//  let num_add:Option<i32>=Some(5)+5;

 println!("{:?} ; {:?} ; {:?}", num, text, other_variable_num);



}