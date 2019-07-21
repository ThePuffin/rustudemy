use std::collections::HashMap;

fn main () {
    let mut scores=HashMap::new();
    scores.insert("Blue", 10);

    scores.entry("Blue").or_insert(20); // si la valeur existe ne rien faire
    scores.entry("Red").or_insert(10); //sinon créer la valeur

    for (key, value) in scores{
        println!("{} : {}", key, value);
    }
}