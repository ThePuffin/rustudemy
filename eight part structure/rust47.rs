// la structure peut être créée à l'interieur de la fonction mais ne sera utilisable qu'à l'intérieur de la fonction
#[derive(Debug)]
struct User {
    age: i32
}


fn main() {
    let u1= User {age: 25};
    println!("{:?}", u1.age);
     // on a une erreur
    // let u2= User {age: 39};
    // si on veut changer la valeur penser à ajouter mut
    let mut u2= User {age: 39};
    u2.age=40;
    println!("{:?}", u2.age);

    if u1.age >u2.age {
        println!("u1 is elder")
    } else if u1.age <u2.age {
        println!("u2 is elder")
    } else {
        println!("same age");
    }


}