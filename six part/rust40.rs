fn main () {

let a=dangle();

}

fn dangle ()->&String {
    let d=String::from("Hello");
    &d
    //d disparait à la fin de l'utilisation de la fonction, la variable a pointe vers rien
}