//principe de shadowing

fn main () {

    //il y a une erreur de compilation mais le résultat donne 20
    let a=10;
    let a=20;
    println!("{}",a );

    //il y a erreur de compilation car on veut changer une variable immutable
    let b=10;
    println!("{}",b );
    b=20;
    println!("{}",b );


    //il n'y a pas d'erreur de compilation car on utilise la premiere valeur de c 
    let c=10;
    println!("{}",c );
    let c=20;
    println!("{}",c );
    // pour chaque a une partie différente de la mémoire va être utilisée

    //il est possible de réatribuer le type de d
    let d:i32=10;
    println!("{}",d );
    let d:char='c';
    println!("{}",d );

    //meme si on rend mutable e il n'est pas possible de changer son type car on touche la même partie de la mémoire
    let mut e:i32=10;
    println!("{}",e );
    e:char='c';
    println!("{}",e );

}
