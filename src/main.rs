#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {


    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap().to_string();
        command = command.trim().to_string();
        if command == "exit" {
            break;
        } else if command.starts_with("type "){
            if command.contains("echo") {
                println!("echo is a shell builtin");
            } else if command.contains("exit"){
                println!("exit is a shell builtin");
            } else if command.starts_with("type type"){ //wrong
                println!("type is a shell builtin");
            } else {
                println!("{}: not found", &command[5..]);
            }
        } else if command.starts_with("echo ") {
            println!("{}", &command[5..]);
        } else {
            println!("{}: command not found", command.trim());
        }
    }

}
