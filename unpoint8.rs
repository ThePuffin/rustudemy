fn main() {
    //possibilité de faire addition, soustraction, division, multiplication, modulo
    println!("{}", 4 + 2);
    println!("{}", 4 - 2);
    println!("{}", 4 / 2);
    println!("{}", 4 * 2);
    println!("{}", 4 % 2);

    let mut a =10;
    //a++;
    //impossible de faire a++ mais a+=1 ok
    a+=2;
    // a-=2:
    // a/=2;
    // a*=2;

    println!("{}", a);    

    let dix =10;
    let vingt =20;
    println!("{}", dix>vingt);
    println!("{}", dix<vingt);
    println!("{}", dix<=vingt);
    println!("{}", dix>=vingt);
    println!("{}", dix==dix);


    //multiple comparaison possible
    println!("{}", 2>3 && 3>2); //reponse false
    println!("{}", 4>3 && 3>2); //reponse true

    println!("{}", 2>3 || 3>2); //reponse true
    println!("{}", 2>3 || 3<2); //reponse false
    println!("{}", 4>3 || 3>2); //reponse true

    //not
    println!("{}", !(3>2) ) //reponse false
    println!("{}", !(3<2) ) //reponse true


}