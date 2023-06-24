use std::fmt::{Error, write};
use std::io::{self, stdout, Write};
use std::process::exit;
use std::process::Command;
use std::fs::{self, read, create_dir};
use std::fs::File;
use std::fs::OpenOptions;
use std::str;
use rpassword;

const VERSION: f32 = 0.1;

fn password_input_handle() -> String {
    let password = rpassword::prompt_password("password: ");
    let password = match password {
        Ok(string) => string,
        Err(error) => panic!("panic! with input: {:?}", error),
    };
    return password;
}

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
    match lowercase {
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
        "init-index" => {
            println!("sys: initializing a new index file"); 
            index_file_init();
        },
        "add-index" => {
            println!("sys: adding new path entry to index"); 
            index_file_add_entry();
        },
        "decrypt-file" => {
            unlock_and_read();
        },
        "read-index" => {
            let read = index_file_load();
        },
        "sys-exit" => {
            println!("sys: exit"); 
            exit(0);
        },
        "clear" => {
            clear_screen();
        },
        &_=> {
            println!("err: invalid command");
        },
    };
    println!();
}

fn index_file_init() {
    let init_file_dir = fs::create_dir("./data");
    let _result = match init_file_dir {
        Ok(()) =>(),
        Err(error) => panic!("panic! create directory error: {:?}", error)
    };
    let init_file = File::create("./data/data.ms");
    let _result = match init_file {
        Ok(file) => file,
        Err(error) => panic!("panic! create file error: {:?}", error)
    };
}

fn index_file_add_entry() {
    let index_file_load_result = OpenOptions::new()
        .append(true)
        .open("./data/data.ms");
    let mut loaded_file = match index_file_load_result {
        Ok(file) => file,
        Err(error) => panic!("panic! opening file error: {:?}", error)    
    };
    let new_entry = input_handle("new file path", false);
    let new_entry = "\n".to_owned() + new_entry.as_str();
    let write_result = writeln!(loaded_file, "{}",new_entry);
    let _result = match  write_result {
        Ok(()) => (),
        Err(error) => panic!("panic! writing file error: {:?}", error)    
    };
}

//index file loaded into an array and printed
fn index_file_load() -> Vec<String>{
    let loaded_file: Vec<String> = fs::read_to_string("./data/data.ms")
    .expect("panic! load error")
    .split("\n")
    .map(|line| line.to_string())
    .collect();
    println!("[entry]    [path]");
    for (i,val) in loaded_file.iter().enumerate() {
        if val.len() == 0 {
            continue;
        } else {
            println!("  [{}]      {}", i,&val)
        }
    }
    return loaded_file;
}

fn unlock_and_read() {
    let index = index_file_load();
    let selection = input_handle("selection:", false);
    match &selection.parse::<usize>() {
        Err(error) => {
            println!("err: invalid entry. {}", error);
            return;
        },
        Ok(value) => {
            let filepath = &index[*value];
            let passphrase = password_input_handle();
            let run_command = Command::new("sh")
                .arg("-c")
                .arg(format!("gpg --batch --pinentry-mode loopback --cipher-algo AES256 --passphrase {passphrase} {filepath}"))
                .output();
            let result = match run_command {
                Ok(output) => output.stderr,
                Err(error) => panic!("panic! error running gpg: {:?}", error)    
            };
            let string = match str::from_utf8(&result) {
                Ok(v) => v,
                Err(e) => panic!("panic! invalid utf-8 {}", e),
            };
            if string.contains("gpg: decryption failed: Bad session key") {
                println!("err: decryption error. bad password.");
                return;
            }
        }
    };
}

fn main() {
    loop {
        let command = input_handle("root", true);
        command_proc(&command, VERSION)
    }
}
