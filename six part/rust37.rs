// fn main () {
//     let s=String::from("Hello");
//     take(s);
//     // //impossible de l'afficher après car elle n'est plus liée à cette fonction
//     // println!("{}", s )
// }

// fn take(s1: String){
//     println!("{}", s1)
// }

fn main () {
    let mut s=String::from("Hello");
    s=take(s);
    //possible de l'afficher après car on a retourné la valeur
    println!("{}", s );
}

fn take(s1: String)->String { 
    println!("{}", s1);
    //on retourne la valeur de s1 (attention pas de ; après)
   s1
}