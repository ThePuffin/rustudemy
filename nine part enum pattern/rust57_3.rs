// store values inside our enum
#[derive(Debug)]
enum IpAddrKind {
  V4Bis (u8,u8,u8,u8),
    V4 (String),
    V6(String),
}


fn main() {
    let home=IpAddrKind::V4( String::from("127.0.0.1"));

    let loopback=IpAddrKind::V6( String::from("::1"));

    //we can store multiple type also
    let homebis=IpAddrKind::V4Bis( 127,0,0,1);

    println!("{:?}",home);
    println!("{:?}",loopback);
    println!("{:?}",homebis);

}
