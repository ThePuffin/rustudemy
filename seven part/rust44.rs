/*

to add rand, we use :
extern crate rand;
before fn main()

and also add the fonctionality:

use rand::Rng;
use std::io;

to generate the random number we use :
let secret=rand::thread_rng().gen_range(1,10);

thread_rng permit to generate number
gen_range determin the number autorized

 */