//Unrecoverable error
use std::io;

fn main () {
    /*
    BEFORE without std::io and
    let mut no=String::new();
    io::stdin().read_line(&mut no).expect("Fail");
    let no::u32=no.trim().parse().expect("Fail");
   */

  //NOW
    println!("Hello, write a number");
    let mut no=String::new();
    io::stdin().read_line(&mut no).expect("Fail");
    let no:u32=match no.trim().parse() {
        Ok(num)=>num,
        Err(_)=>0,
    };
    println!("{}",no);
    println!("Hello");
}

