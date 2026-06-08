#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim();
        if command == "exit" {
            break;
        } else if command.contains("echo") {
            println!("{}", &command[5..]);
        } else {
            println!("{}: command not found", command);
        }
    }

}
