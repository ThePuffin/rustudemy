use::std::fs::File;
use::std::io::ErrorKind;
fn main () {
    let f=File::open("hellos.txt");
    let f= match f {
            Ok(file)=>file,
            Err(ref error) if error.kind()==ErrorKind::NotFound=> { 
              match File::create("hellos.txt") {
                Ok(fc)=>fc,
                Err(e)=>{ 
                    panic!("Not able to create file {:?}", e)
                },
            }
    },
    Err(error)=> { panic!("Unable to open file {:?}",error);},
    };

}