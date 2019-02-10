fn main () {
    // add();
    // anotherfunction(1,2);
    // let res = otherfunction(1,8);
    // println!("{}",res );
    // //ou
    // println!("{}",otherfunction(4,3));
    
// println!("{:?}", sub_add(4, 2));
addd();
fn addd(){
    println!("coucou" );
    println!("{:?}", sub_add(4, 1));
}


}

// fn add() {
//     println!("{}", 2+3 )
// }

// fn anotherfunction(a:i32,b:i32){
// println!("{} + {} = {}", a,b, a+b )
// }

// fn otherfunction(a:i32,b:i32)->i32{
    // // return est optionnel
//     return a+b
// }

fn sub_add(a:i32,b:i32)->(i32,i32){
    (a+b,a-b)
}