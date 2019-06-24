fn main () {
let v=vec![1,2,3];
//first method to get a value
let value=v[1];
//but the program crash with that even if compile
// let value=v[10];
println!("{:?}", value);

//second method to get a value
let values=v.get(1);
//the program compile and the value equal none
let valuess=v.get(10);

println!("{:?} - {:?}", values, valuess);
}