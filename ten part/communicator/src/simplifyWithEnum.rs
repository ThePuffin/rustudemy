#[allow(unused_variables)]

enum TrafficLight{
    Red,Yellow,Green,
}


// use  TrafficLight::{Red,Yellow};
// fn main () {
//     let red=Red;
//     let yellow=Yellow;
//     let green=TrafficLight::Green; // because we don't import green with use TrafficLight
// }

use TrafficLight::*;
fn main () {
    let red=Red;
    let yellow=Yellow;
    let green=Green;
}
