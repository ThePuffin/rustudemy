pub fn login(user:String, pass:String)->i32{
    if user=="Vinz" && pass=="Password"{
        println!("Logged in Successfully");
        1
    }
    else{
        0
    }
}