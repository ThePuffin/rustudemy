fn main () {
    let s1=String::from("Hello");

    // let n=s1[0];    /* impossible */
    let n=&s1[0..1];
    let nn=&s1[0..2];
    println!("{}     {}", n, nn);


    for m in s1.chars(){
    println!("{}", m);
    };

 for m in " World".chars(){
    println!("{}", m);
    };
    }