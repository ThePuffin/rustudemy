use std::fs::File;

fn main () {

    // let f=File::open("abc.txt").unwrap();
    let ff = File::open("abc.txt").expect("Failed");

}