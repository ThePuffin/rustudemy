#[derive(Debug)]
struct User {
    age: i32
}

fn build (age:i32)->User{
    User {
        age: age
    }
}


fn main() {
    let u1= build(20);
    println!("{:?}", u1);
}