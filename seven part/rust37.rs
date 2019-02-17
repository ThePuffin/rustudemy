fn main () {
    let s=String::from("Hello");
    take(s);
    // on ne pas l'afficher car la valeur a été assigné à la fonction take
    // println!("{}",s );

    let mut t=String::from("World");
    t = take_two(t);

    println!("{}", t);
}

fn take(s1 : String){
    println!("{}", s1);
}

fn take_two(t1 : String)->String {
    println!("{}", t1);
    t1
}
