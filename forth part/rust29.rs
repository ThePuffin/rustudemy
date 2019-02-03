fn main () {
    let nombre=1;
    let mut sum=1;
if nombre>1 {
    for number in 2 .. nombre+1 {
        sum = fact(number,sum);
    } 
}
    println!("Pour {}, on obtient : {}", nombre,sum);
}

fn fact(number:i32,sum:i32)->i32{
 sum*number
}