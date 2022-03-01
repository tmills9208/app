use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::fs::OpenOptions;

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

    let new_line = "Oh ho ho! A new line! Where did I come from? 🦀\n";
    write_line_to_file(&file_path, &new_line);
}

fn write_line_to_file(_path: &str, _content: &str) -> () {
    let path = Path::new(_path);
    let mut file = match OpenOptions::new()
        .write(true)
        .append(true)
        .open(&path) {
            Ok(file) => file,
            Err(err) => panic!("Couldn't open file at {}: {}\n", path.display(), err)
        };
    
    match file.write(&_content.as_bytes()) {
        Ok(size) => print!("Successfully wrote to {}.\nSize of: {}\n", path.display(), size),
        Err(err) => panic!("Couldn't write string to file, at {}: {}\n", path.display(), err)
    }
}

fn read_from_file(_path: &str) -> () {
    let path = Path::new(_path);
    let mut file = match File::open(&path) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't open file at {}: {}\n", path.display(), err)
    };
    
    let mut result = String::new();
    match file.read_to_string(&mut result) {
        Ok(size) => print!("{} contains:\n{}\nSize of: {}\n", path.display(), result, size),
        Err(err) => panic!("Couldn't read from file to string at {}: {}\n", path.display(), err)
    }
}
