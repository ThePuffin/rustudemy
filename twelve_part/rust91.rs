use std::io;
use std::io::Read;
use std::fs::File;

fn main () {
    let output=read();
    match output{
        Ok(fi)=>println!("{:?}", fi),
        Err(e)=>println!("{:?}", e),
    };
}

fn read()-> Result<String,io::Error> {
    let mut f=File::open("shello.txt")?;

    let mut s=String::new();
    f.read_to_string(& mut s)?;

    Ok(s)
}