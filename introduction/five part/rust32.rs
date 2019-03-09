fn main () {
    // let a:(i32,bool,f64) = (220,true,8.5);
    // println!("{:?}",a );

    //on print la première valeur
      let b:(i32,bool,f64) = (220,true,8.5);
    println!("{:?}",b.0 );

    //on print la troisième valeur
    let c:(i32,bool,f64) = (220,true,8.5);
    println!("{:?}",c.2 );
    print(c);
    printbis(c);

}

//il faut déclarer le même type d'élément que ceux du paramètre
fn print(x:(i32,bool,f64)){
    println!("{:?}",x.1);
}

fn printbis(x:(i32,bool,f64)){
    //=x le paramètre déclaré au dessus
    let (g,h,i)=x;
    println!("{}, {}, {}",g,h,i);
}