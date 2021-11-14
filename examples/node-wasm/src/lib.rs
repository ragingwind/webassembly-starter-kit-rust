use std::fs;
use std::io::prelude::*;

// export name with no mangling
#[no_mangle]
pub extern "C" fn print_hello() {
    // use system stdio
    println!("Hello, Node!");

    // use system file io
    let mut file = fs::File::create("/host/helloworld.txt").unwrap();

    // write the text to the file we created
    write!(file, "Hello Node!\n").unwrap();
}
