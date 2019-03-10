// fn main () {
//     let a=10;
// }

// fn fun() {
//     //'a' a déjà été attribué à un point de mémoire en dehors de la scope donc on a une erreur 
//     a=20;
// }

// fn main () {
//     let s= String::from("Hello");
//     let s1=s;
//     //provoque une erreur car la valeur a été déplacée
//     // println!("{}", s );
//     println!("{}", s1);
// }

fn main () {
let s= String::from("Hello");
//clone() permet de faire une copie sans réattribué le pointeur vers le point de mémoire
let s1=s.clone();
 println!("{}", s );
 println!("{}", s1);
}

