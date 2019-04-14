//associated functions
#[derive(Debug)]
struct Rectangle {
    width: i32,
    height: i32,
}


impl Rectangle {
    fn build(width:i32, height:i32)->Rectangle{
        Rectangle {width,height}
    }
}


fn main() {
    let r1= Rectangle::build(2,3);
    println!("{:?}", r1);
    }