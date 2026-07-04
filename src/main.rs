#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn main() {


    loop {
        let mut command = String::new();
        let path = env::var("PATH").unwrap();
        let folders = path.split(":");
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap().to_string();
        command = command.trim().to_string();
        let mut parts = command.splitn(2, ' ');
        let cmd = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("").trim_start();
        if cmd == "exit" {
            break;
        } else if cmd == "type" {
            let command_name = rest;
            if command_name == "echo" {
                println!("echo is a shell builtin");
            } else if command_name == "type" {
                println!("type is a shell builtin");
            } else if command_name == "exit" {
                println!("exit is a shell builtin");
            } 
            else {
                let mut found = false;
                for folder in folders {
                    let path_check = Path::new(folder);
                    let final_path = path_check.join(command_name);
                    if let Ok(metadata) = fs::metadata(&final_path) {
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
        } else if cmd == "echo" {
            println!("{}", &command[5..]);
        }
        else {
            //check file execution
            let mut found = false;
            for folder in folders {
                let path_check = Path::new(folder);
                let final_path = path_check.join(cmd);
                if let Ok(metadata) = fs::metadata(&final_path) {
                    if metadata.permissions().mode() & 0o111 != 0 {
                        let arguements = rest.split_whitespace();
                        let raw_output = Command::new(final_path).args(arguements).output().unwrap();
                        let output = String::from_utf8_lossy(&raw_output.stdout);
                        if !output.is_empty() {
                            print!("{}", output);
                            found = true;
                            break;
                        }
                    }  
                }
            }
            if !found {
                println!("{}: command not found", command.trim());
            }
        }
    }

}
