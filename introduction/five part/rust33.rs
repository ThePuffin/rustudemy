fn main (){
/* le tableau ne pas être plus long ou court que ce qui sera définit au début*/

    let arr=[22,34,65,3];
    //ne pas oublier le :?
    println!("{:?}", arr);

    //on définit le type puis le nombre de fois que ce type doit être appliqué
    let arra:[i32;4]=[22,34,65,3];
    println!("{:?}", arra);

    //on remplit le tableau avec des zéros
     let ar:[i32;5]=[0;5];
    println!("{:?}", ar);

    //la première valeur de l'array sera 1 et la troisième un 5
    let mut a:[i32;5]=[0;5];
    a[0]=1;
    a[2]=5;
    println!("{:?}", a);
    print(a);
    printbis(a);

}

fn print(x:[i32;5]){
    //len() permet de connaitre la longueur de l'array
for n in 0 .. x.len() {
    println!("{}", x[n] );
}
}

//autre méthode pour afficher chaque valeur
fn printbis(x:[i32;5]){
for n in x.iter() {
    println!("{}", n );
}
}