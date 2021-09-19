
use std::env;
pub fn run() {
    let args: Vec<String> = env::args().collect(); //std::env::args().collect(); pode ser env::args().collect(); caso "use std::env" for declarado anteriormente
    let command = args[1].clone();
    let name = "KARL";
    let status = "100%";

    println!("ARGS: {:?}", args);
    println!("COMMAND: {}", command);

    if command == "hello" {
        println!("Hi {}, hou are ya? ", name);
    } else if command == "status" {
        println!("Status is {}, ", status);
    
    } else {
         println!("Not a valisd command");
    }
    
}