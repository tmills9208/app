/*
    Project: tmills-file
    Author: tmills9208
    Date: 03-14-2022 (pi, lol)
    Description: 
        So far, a simple CLI application that can perform reading, writing and over-writing to files.
    While using no other dependencies to keep the application limited to the standard rust library.
*/

mod lib;

use std::env;
use lib::file_controller;

fn main() {
    let args: Vec<String> = env::args().collect();

    let flag = &args[1];
    let file_path = &args[2];
    let input = &args[3];

    // Options are preferred, to show that they either contain a type, or nothing. no more nulls :p
    // let hello: Option<&str> = Some("Hello, web browsers!");

    // Use mutable when you know if the variable will be change beyon compile time
    // let mut result: &str = match hello {
    //     Some(ref x) => x,
    //     None => ""
    // };
    
    // Since its not being dynamically used, it is unneeded and you can simply reassign it
    // by simply having let be declared for the same variable again.
    // like below:
    // let result: &str = match hello {
    //     Some(ref x) => x,
    //     None => ""
    // };

    // This will work too, may be even simpler just for single strings
    // result = hello.map(|m| &m[..]).unwrap_or("");

    // requires a string of type: &str
    // println!("{}\n\n", result);

    // let file_path = "./hello.txt";
    if (flag == "read" || flag == "-r") {
        file_controller::read_from_file(&file_path);
    }
    if (flag == "write" || flag == "-w") {
        let new_line = format!("{}", &input);
        file_controller::write_to_file(&file_path, &new_line);
    }
    if (flag == "overwrite" || flag == "-ow") {
        let new_line = format!("{}", &input);
        file_controller::overwrite_to_file(&file_path, &new_line);
    }
    if (flag == "writeline" || flag == "-wl") {
        let new_line = format!("{}\n", &input);
        file_controller::write_to_file(&file_path, &new_line);
    }
}
