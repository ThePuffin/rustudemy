fn main () {
   let a= "hello";
   println!("{}",a);
    let b:i32= 1;
   println!("{}",b);


//il va y avoir un probleme car i8 ne peut stocker chiffre avec max:255
// 256 = 0; 258=2 ; 259 = 3 ...
//    let c:i8=256;
//    println!("{}",c);

   let d:f32=259.05;
   println!("{}", d);

   let e:bool=true;
   println!("{}", e);

   let f:char='a';
   println!("{}", f);

// ça ne fonctionne pas il faut une seule lettre
//    let g:char='alorsoui';
//    println!("{}", g);
}