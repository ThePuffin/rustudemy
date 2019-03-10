fn main () {
    let a=String::from("Hello_World");
    let r1=&a[0..5];
    let r2=&a[0..=5];
    let r3=&a[..5];
    let r4=&a[0..];
    let r5=&a[..];
    let r6=&a[1..8];
    let a=[1,2,3,4,5];
    let b=&a[0..3];


println!("r1:{}", r1);
println!("r2:{}", r2);
println!("r3:{}", r3);
println!("r4:{}", r4);
println!("r5:{}", r5);
println!("r6:{}", r6);
println!("b:{:?}", b);

}

/*-------------*/