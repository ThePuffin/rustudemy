//Unrecoverable error

fn main () {
    println!("Welcome");
    let doc=document();
    if doc==String::from("Yes"){
        println!("Thank you for submitting documents");
        println!("Now submit fees");
        let fees=fees();
        if fees==String::from("Yes"){
            println!("Thank you for submitting fees, admission confirmed" );
        }
        else{
              println!("Sorry you have not submitted fees, admission cancelled" );
              panic!("");
        }
    } else {
           println!("Sorry you have not submitted documents, admission cancelled" );
              panic!("");
    }
}

fn document () ->String {
let doc="Yes";
// let doc="No";
    println!("Do you have all documents");
    if doc=="Yes" {
        println!("{}\n", doc);
        return String::from("Yes");
    }
    else {
        println!("{}\n", doc);
        return String::from("No");
    }
}

fn fees () ->String {
    let fees="No";
    // let fees="Yes";
    println!("Please submit your fees" );
    if fees=="Ok" {
        println!("{}\n", fees );
        return String::from("Yes");
    }
    else{
        println!("{}\n", fees );
        return String::from("No");  
    }
}