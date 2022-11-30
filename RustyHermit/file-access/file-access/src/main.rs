#[cfg(target_os = "hermit")]
extern crate hermit_sys;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("/etc/passwd");
    let path_display = path.display();
    
    println!("try to open {} now...", path_display);

    let mut file = match File::open(&path) {
        Err(err) => panic!("Could not open {} because of \"{}\"", path_display, err),
        Ok(file) => file,
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Err(err) => println!("Could not read {} because of \"{}\"", path_display, err),
        Ok(_) => println!("File content: \n {}", content),
    };
}
