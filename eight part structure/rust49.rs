#[derive(Debug)]
struct User {
    name:String,
    age: i32,
    hobby:String
}


fn main() {
    let u1= User {
        name:String::from("Rob"),
        age: 25,
        hobby:String::from("Cricket")
    };


     let u2= User {
        name:String::from("Bob"),
        age:u1.age,
        //creer une erreur car la mémoire a été déplacée pas sur age car c'est un int
        // hobby:u1.hobby,
        hobby: u1.hobby.clone()
    };

     let u3= User {
        name:String::from("Mob"),
       ..u1
       //fonctionne si on enleve hobby dans l'objet et u1 et u2
    };

    println!("u1={:?} \nu2={:?}\nu3={:?}", u1, u2, u3);
}