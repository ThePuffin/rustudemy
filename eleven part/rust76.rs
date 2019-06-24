fn main () {
 let a=1;
 //transform number in string
 let mut s=a.to_string();

 //add strings
 s.push_str(" Hello");

 //add a character (simple quote not double)
 s.push('o');


 println!("{}", s);

}