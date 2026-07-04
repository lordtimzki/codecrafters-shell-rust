#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
use std::path::Path;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

fn handle_builtin(cmd: &str, rest: &str) -> bool {
        let path = env::var("PATH").unwrap();
        let folders = path.split(":");
        if cmd == "exit" {
            return false;
        } else if cmd == "type" {
            let command_name = rest;
            if command_name == "echo" {
                println!("echo is a shell builtin");
                return true;
            } else if command_name == "type" {
                println!("type is a shell builtin");
                return true;
            } else if command_name == "exit" {
                println!("exit is a shell builtin");
                return true;
            } else if command_name == "pwd" {
                println!("pwd is a shell builtin");
                return true;
            } else if command_name == "cd" {
                println!("cd is a shell builtin");
                return true;
            }
            else {
                for folder in folders {
                    let path_check = Path::new(folder);
                    let final_path = path_check.join(command_name);
                    if let Ok(metadata) = fs::metadata(&final_path) {
                        if metadata.permissions().mode() & 0o111 != 0 {
                            println!("{} is {}", command_name, final_path.display());
                            return true;
                        }
                    }
                }
                println!("{}: not found", command_name);
                return true;
            }
        } else if cmd == "echo" {
            println!("{}", rest);
            return true;
        } else if cmd == "pwd" {
            let current_dir = env::current_dir().unwrap();
            println!("{}", current_dir.display());
            return true;
        } else if cmd == "cd" {
            if let Err(e) = std::env::set_current_dir(rest) {
                eprintln!("{}: {}: No such file or directory", cmd, rest);
            }
            return true;
        }
        else {
            //check file execution
            for folder in folders {
                let path_check = Path::new(folder);
                let final_path = path_check.join(cmd);
                if let Ok(metadata) = fs::metadata(&final_path) {
                    if metadata.permissions().mode() & 0o111 != 0 {
                        let arguments = rest.split_whitespace();
                        let raw_output = Command::new(cmd).args(arguments).output().unwrap();
                        let output = String::from_utf8_lossy(&raw_output.stdout);
                        if !output.is_empty() {
                            print!("{}", output);
                        }
                        return true;
                    }
                }
            }
            println!("{}: command not found", cmd);
            return true;
        }
}

fn main() {


    loop {
        let mut command = String::new();
        print!("$ ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut command).unwrap();
        command = command.trim().to_string();
        let mut parts = command.splitn(2, ' ');
        let cmd = parts.next().unwrap_or("");
        let rest = parts.next().unwrap_or("").trim_start();
        if !handle_builtin(cmd,rest) {
            break;
        }
    }

}
