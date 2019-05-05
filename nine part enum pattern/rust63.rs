//control flow  using if let

fn main() {
    let some_u8=Some(3);


    if let Some(3)=some_u8 {
        println!("Three");
    }
    else if let Some(4)=some_u8{
        println!("Four")
    }   
    else {
        println!("Some other value");
    }
}