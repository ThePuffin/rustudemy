pub mod a {
    pub mod series {
        pub mod of {
            pub fn nested_module() {}
        }
    }
}

// fn main () {
//     a::series::of::nested_module();
// }
 /*or*/
use  a::series::of;
fn main () {
    of::nested_module();
}
 /*or*/
use  a::seriesof::nested_module;
fn main () {
    nested_module();
}