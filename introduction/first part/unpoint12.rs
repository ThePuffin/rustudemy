// input from user

//permet d'ajouter le package io
use std::io;
fn main () {
    //on definit une variable
    let mut a=String::new();
    println!("enter a string");
    //on attribue le texte entré à la variable   stdin=standard input
    io::stdin().read_line (&mut a).expect("Failed");
    //on affiche juste la valeur
    println!("{}", a)
    //on ajoute world mais pas sur la même ligne à cause de l'entrée que nous faisons pour valider et qui est prise en compte
    println!("{}World", a)

    //on converti la valeur obtenue et on enlève l'entrée
    let a:String=a.trim().parse().expect("Failed");
    //on ne peut convertir le texte en numérique on aura une erreur si on ne met pas un chiffre
    let a:i32=a.trim().parse().expect("Failed");
    //on ne peut convertir le texte ou un chiffre en boolean (true,false) on aura une erreur si on ne met pas un true ou false
    let a:bool=a.trim().parse().expect("Failed");
      //on affiche la valeur à laquelle on ajoute world \t permet l'ajout d'un espace \n si on veut retour chariot
    println!("{}\tWorld", a)
}