// fn main () {
//     let s=String::from("Hello");
//     let r1=&s;
//     let r2=&s;
//     let r3=&s;
//     //on peut créer autant de référence non mutable que l'on veut

// }

/*----------------------------*/

// fn main () {
//     let mut s=String::from("Hello");
//     let r1=&mut s;
//     // impossible d'avoir deux mutations à la suite
//      let r2=&mut s;
// }

// /*----------------------------*/

// fn main () {
//     let mut s=String::from("Hello");
//     {
//     let r1=&mut s;
//     }
//     //pas d'erreur car elle ne sont pas dans la même scope
//      let r2=&mut s;
// }

/*----------------------------*/

// fn main () {
//     let mut s=String::from("Hello");
//     let r1=&mut s;
//     //erreur car sont dans la même scope (même problème que précédemment)
//      let r2=&s;
// }

/*----------------------------*/

fn main () {
    let mut s=String::from("Hello");
    {
        let r1=&mut s;
        }
    //pas d'erreur car elle ne sont pas dans la même scope
     let r2=&s;
}

