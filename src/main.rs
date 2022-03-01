use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {

    // Options are preferred, to show that they either contain a type, or nothing. no more nulls :p
    let hello: Option<&str> = Some("Hello, web browsers!");

    // Use mutable when you know if the variable will be change beyon compile time
    let mut result: &str = match hello {
        Some(ref x) => x,
        None => ""
    };
    // Since its not being dynamically used, it is unneeded and you can simply reassign it
    // by simply having let be declared for the same variable again.
    // like below:
    // let result: &str = match hello {
    //     Some(ref x) => x,
    //     None => ""
    // };

    // This will work too, may be even simpler just for single strings
    result = hello.map(|m| &m[..]).unwrap_or("");

    // requires a string of type: &str
    println!("{}\n\n", result);

    let file_path = "./hello.txt";
    read_from_file(&file_path);
}

fn read_from_file(_path: &str) -> () {
    let path = Path::new(_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't open file at {}: {}", path.display(), err)
    };
    
    let mut result = String::new();
    match file.read_to_string(&mut result) {
        Ok(_) => print!("{} contains:\n{}", path.display(), result),
        Err(err) => panic!("Couldn't read from file to string at {}: {}", path.display(), err)
    }
}
