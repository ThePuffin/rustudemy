//derive(debug) permet d'afficher l'objet dans le print
#[derive(Debug)]
struct User {
    email: String,
    age: i32,
}


fn main() {
    let rob = User {
        email:String::from("rob@email.com"),
        age:22
    };
    println!("{:?}", rob);
}