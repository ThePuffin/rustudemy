fn main () {
    let mut v=vec![1,2,3,4,5];

    //without & the variable move and can not be used after the loop
    for i in & mut v {
        //we can change value because vector is mutable in declaration and in loop
        *i *=2;
        println!("{}", i );
    }
        // println!("{:?}", v );


}