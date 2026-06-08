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
        }
        if command.contains("echo") {
            print!("{}", &command[5..]);
            print!("\n");
            io::stdout().flush().unwrap();
        } else {
            println!("{}: command not found", command.trim());
        }
    }

}
