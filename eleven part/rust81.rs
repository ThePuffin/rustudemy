use std::collections::HashMap;

fn main () {
    let mut scores=HashMap::new();
    scores.insert("Blue",10);
    scores.insert("Red",20);

    let score_blue=scores.get("Blue");
    println!("{:?}", score_blue);
    let score_yellow=scores.get("Yellow"); /*yellow n'existe pas*/
    println!("{:?}", score_yellow); // response none


    for(key, value) in &scores {
        println!("{} : {}", key, value);
    }
}