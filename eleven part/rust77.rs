fn main () {
    let s1=String::from("Hello ");
    let s2=String::from("World");
    let s3=s1+&s2;
    println!("{}", s3);
    // println!("{}", s1); /* impossible car s1 a été déplacé */

    let s11=String::from("Hello ");
    let s12=String::from("World");
    let s13= format!("{}-{}", s11,s12);
    println!("{}", s13);
    println!("{}", s11);

}