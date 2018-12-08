//typecasting

fn main () {

    //on assigne une valeur à a et on attribue la valeur de a à b on a une erreur car a et b ne sont pas de même type
    let a:i32=10;
    let b:i64=a;
    println!("{}", a);

    //alors que ok
    let c:i32=10;
    let d:i32=c;
    println!("{}", d);

    //ce qu'il faut faire
    let e:i32=10;
    let f:i64=e as i64;
    println!("{}", f);

    //ou encore 
    let g:i32=10;
    let h:i64=g.into();
    println!("{}", h);

    // dans le même genre
    let j:i32=10;
    //pas possible
    let k:i64=j +10;
    //possible
    let k:i64=j as i64 +10;
    println!("{}", k);
    //pas possible
    let k:i64=j.into()+10;
    println!("{}", k);

    //pour réutilisé même variable mais en ajoutant une valeur
    let l:i32=10;
    //pas possible
    let l:i64=l + 10;
    //possible
    let l:i64=l as i64 + 10;

}