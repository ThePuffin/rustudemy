extern crate Facebook;

use Facebook::*;

fn main() {
 let user = String::from("Vinz");
 let pass=String::from("Azerty");

 let s=login::login(user,pass);


 if s==1 {
     post::post(String::from("Hi ! from there"));
     logout::logout();
 }
 else{
     println!("Invalid access");
 }
}
