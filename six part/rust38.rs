////////////////////////////////////////////////

// fn main () {
//     let s=String::from("Hello");
//     // & casse les regles de possessions
//     print(&s);
//         println!("{}", s)

// }


// fn print (s1: &String){
//     println!("{}", s1);
// }

///////////////////////////////////////////////////

// fn main () {
//     let s=String::from("Hello");
//     // & casse les regles de possessions
//     print(&s);
//         println!("{}", s)

// }


// fn print (s1: &String){
//     println!("{}", s1);
//     // //impossible de modifier s1 car on prete seulement la valeur à la fonction , elle n'a pas le droit de la modifier
//      s1.push_str("World");

// }

///////////////////////////////////////////////////

fn main () {
    let mut s=String::from("Hello");
    // & casse les regles de possessions
    print(&mut s);
        println!("{}", s)

}


fn print (s1: &mut String){
    println!("{}", s1);
   //  on a le droit de modifier car nous avons fait passer la mutabilité
    s1.push_str(" World");

}