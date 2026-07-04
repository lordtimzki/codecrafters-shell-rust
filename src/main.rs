#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() {


    loop {
        let mut command = String::new();
        let path = env::var("PATH").unwrap();
        let mut folders = path.split(":");
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap().to_string();
        command = command.trim().to_string();
        if command == "exit" {
            break;
        } else if command.starts_with("type "){
            let command_name = &command[5..];
            if command_name == "echo" {
                println!("echo is a shell builtin");
            } else if command_name == "echo" {
                println!("exit is a shell builtin");
            } else if command_name == "type" {
                println!("type is a shell builtin");
            } 
            else {
                let mut found = false;
                for folder in folders {
                    let path_check = Path::new(folder);
                    let final_path = path_check.join(command_name);
                    if let Ok(metadata) = fs::metadata(&inal_path) {
                        if metadata.permissions().mode() & 0o111 != 0 {
                            println!("{} is {}", command_name, final_path.display());
                            found = true;
                            break;
                        }
                    }
                }
                if !found {
                    println!("{}: not found", command_name);
                }
            }
        } else if command.starts_with("echo ") {
            println!("{}", &command[5..]);
        } else {
            println!("{}: command not found", command.trim());
        }
    }

}
