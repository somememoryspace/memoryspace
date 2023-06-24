use std::io::{self, stdout, Write};
use std::process::exit;
use std::process::Command;

const VERSION: f32 = 0.1;

fn input_handle(prompt: &str, lowercase: bool) -> String {
    let mut input_string = String::new();
    
    loop {
        print!("{} > ", prompt);
        input_string.clear();
        stdout().flush().ok();
        let stdin_result = io::stdin().read_line(&mut input_string);
        let _stdin = match stdin_result {
            Ok(usize) => usize,
            Err(error) => panic!("panic! with input: {:?}", error),
        };
        break;
    }
    match(lowercase) {
        true => return input_string.trim().to_string().to_lowercase(),
        false => return input_string.trim().to_string(),
    }
}
fn clear_screen() {
    stdout().flush().ok();
    let clear_result = Command::new("clear").status();
    let _clear = match clear_result {
        Ok(exit) => exit,
        Err(error) => panic!("panic! stdout error: {:?}", error)
    };
}
fn command_proc(command: &str, version: f32) {
    println!();
    match command {
        "sys-exit" => {
            println!("--system-exit--"); 
            exit(0);
        },
        "clear" => {
            clear_screen();
        },
        &_=> {
            println!("--invalid-command--");
        },
    };
    println!();
}

fn main() {
    loop {
        let command = input_handle("root", true);
        command_proc(&command, VERSION)
    }
}
