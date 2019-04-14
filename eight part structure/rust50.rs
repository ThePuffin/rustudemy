//create the structure
#[derive(Debug)]
struct Rectangle {
    width:u32,
    height: u32,
}

//implemente structure
impl Rectangle {
     fn area(&self)->u32{
         self.width*self.height
     }
}


fn main() {
    let rect1= Rectangle{width:30, height:50};
    let area=rect1.area();
    println!("area is: {}",area);

    //block if we use rect1 again:
    // let area = rect1.area();
}