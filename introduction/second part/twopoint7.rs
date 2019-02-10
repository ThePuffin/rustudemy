//if let number 19

fn main () {
    let num= if 3>2 {
        println!("if block");
    1
    }else{
        println!("else");
        2
    };
    println!("{}", num );


    let num_without_else= if 3>22 {
        println!("if block");
    ()
    };
    println!("{:?}", num_without_else )
}

//les valeurs renvoyées doivent être du même type