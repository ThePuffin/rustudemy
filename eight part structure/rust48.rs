#[derive(Debug)]
struct User {
    name:String,
    age: i32
}

fn build (name:String,age:i32)->User{
    User {
        name,
        age
        //age: age peut être aussi utiliser
    }
}


fn main() {
    let u1= build(String::from("Rob"),20);
    println!("{:?}", u1);
}