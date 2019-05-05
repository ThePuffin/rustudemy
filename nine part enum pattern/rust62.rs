//the placeholder


fn main() {
    let five=Some(5);
    let six=plus_one(five);
    let none=plus_one(None);
    println!("{:?}, {:?}", six, none);
}

fn plus_one(x:Option<i32>)-> Option<i32>{
    match x {
        //  // error because we need to specify the value but we can use _ 
        // None=>None,
        Some(i)=>Some(i+1),
        // use _
        _=>None,
    }
    
}