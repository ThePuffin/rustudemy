// introduction to enum

#[allow(dead_code)] //permit to create some variable but don't use it in our test for that learning

//variable should be in camel case but we can use to use some not camel case variable:
#[allow(non_camel_case_types)]


#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

fn main () {
    let four=IpAddrKind::V4;
    let six=IpAddrKind::V6;

    // println!("{:?}",four);
    // println!("{:?}",six);


    route(four);
    route(six);
}

fn route(ip:IpAddrKind){
    println!("{:?}", ip);
}