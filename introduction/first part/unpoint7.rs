const NUM:i32=20;


fn main() {
  let a=22;
  //cela crée une erreur car la valeur est immutables
  a=20;
  println!("{}", a);

//mut permet de modifier la valeur de la variable
let mut b=10;
b=20;
println!("{}", b);

//le nom doit être obligatoirement en majuscule pour une constante et doit avoir son type
const MAX:i32=10;
println!("{}", MAX);

println!("{}", NUM );

//affichage d'un 'string'
let h="Hello";
println!("{}", h);

//on définit une variable puis on lui assigne une valeur
let mut g=String::new();
g=String::from("coucou");
println!("{}", g);
//ou
let gg=String::from("coucou");
println!("{}", gg);

}