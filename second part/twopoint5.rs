//if else again numero 17-18

use std::io;

fn main (){
let mut alph=String::new();
println!("give me a letter of the alphabet, please");
io::stdin().read_line(&mut alph).expect("Failed");
let alph:char=alph.trim().parse().expect("Failed");
if alph=='a'|| alph=='e' || alph=='i' || alph=='o' || alph=='u' || alph=='y'{
    println!("{} is a vowel", alph)
}else {
    println!("{} is not a vowel", alph)
}


let mut first_num=String::new();
println!("1st number");
io::stdin().read_line(&mut first_num).expect("Failed");
let _first_num:i32=first_num.trim().parse().expect("Failed");

let mut operator=String::new();
println!("operator");
io::stdin().read_line(&mut operator).expect("Failed");
let _operator:String=operator.trim().parse().expect("Failed");

let mut second_num=String::new();
println!("2nd number");
io::stdin().read_line(&mut second_num).expect("Failed");
let _second_num:i32=second_num.trim().parse().expect("Failed");


if _operator=="+"{
    let result=_first_num+_second_num;
    println!("{} is the result", result);
}else if _operator=="-"{
    let result=_first_num-_second_num;
    println!("{} is the result", result);
}else if _operator=="*"{
    let result=_first_num*_second_num;
    println!("{} is the result", result);
}else if _operator=="/"{
    let result=_first_num/_second_num;
    println!("{} is the result", result);
} else {
    println!("error" );
}


let mut grade=String::new();
println!("enter the percentage");
io::stdin().read_line(&mut grade).expect("Failed");
let _grade:i32=grade.trim().parse().expect("Failed");
if _grade > 100{
    println!("error" )
}else if _grade>90 {
    println!("grade A");
}else if _grade>80{
    println!("grade B");
}else if _grade>70{
    println!("grade C");
}else if _grade>60{
    println!("grade D");
}
else{
    println!("Failed");
}
}