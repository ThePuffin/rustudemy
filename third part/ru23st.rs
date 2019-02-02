use std::io;

fn main () {

// // point 1

// for v in 1 .. 101 {
//     if v%2==0 {
//         println!("{}",v );
//     }
// }


// point 2 

let mut c=1;

while c<=3{
let mut ch=String::new();
println!("Dis moi oui !");
io::stdin().read_line(&mut ch).expect("Failed");
ch=ch.trim().to_string();
 if ch=="oui"{
     println!("Merci", );
     break;
 }
 else {
 if c==3 {
     println!("Tu n'es pas sympa", );
 }
     println!("il te reste {} chances", 3-c);
     c+=1;
 }
}




}