//pattern that bind to values

#[allow(dead_code)]
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
#[allow(dead_code)]
#[derive(Debug)]
enum UsState{
    Alaska,
    Arizona,
    //etc
}

fn value_in_cents(c:Coin)->u32{
    match c {
        //could return code or just a value
        Coin::Penny=>{
            println!("Penny");
            1
        },
        Coin::Nickel=>5,
        Coin::Dime=>10,
        Coin::Quarter(state)=>{
            println!("{:?}", state);
            25
        },
    }
}


fn main() {
 println!("{}", value_in_cents(Coin::Penny));   
 println!("{}", value_in_cents(Coin::Quarter(UsState::Alaska)));   
}