//if else

use std::io;

fn main () {


//if else classique
let a=10;
if a%2==0 {
    println!("even" );
}
else {
    println!("odd");
}

////////////////////


//if else ou l'utilisateur entre une valeur
let mut ch=String::new();
println!("Are your friends comming ?");
io::stdin().read_line(&mut ch).expect("Failed");
//si on utilise pas trim il prend en compte le retour chariot dans la reponse
ch=ch.trim().to_string();
//to_string() permet de transformer la réponse en string
if ch=="y"{
    println!("yeah!" )
} else {
    println!("nope !")
}

//////////////
// comparaison de deux chiffres
let g=10;
let h=30;

if g>h {
    println!("g is greater" )
}else {
    println!("h is greater" )
}


/////////////////////

let mut name=String::new();
let mut age=String::new();
let mut ca=String::new();

println!("Enter Name and age :");
io::stdin().read_line(&mut name).expect("Failed");
io::stdin().read_line(&mut age).expect("Failed");
let age:i32=age.trim().parse().expect("Failed");

println!("Do you want to create account ?");
io::stdin().read_line(&mut ca).expect("Failed");
ca=ca.trim().to_string();

if ca=="y" {
    if age<10 {
        println!("Your age is less");
    } else {
        println!("Your account is created");
        println!("Do you want to upload photo ?");
        ca.clear();
        io::stdin().read_line(&mut ca).expect("Failed");
        ca=ca.trim().to_string();
        if ca=="y" {
            if age<13 {
                println!("You cannot upload photos")
            }
            else {
                println!("You can upload your photo")
            }
        } else {
            println!("no photo ok")
        }
    }
} else {
    println!("Bye bye")
}


}