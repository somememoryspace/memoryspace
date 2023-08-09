use std::io::{self, stdout, Write};
use std::process::Command;
use std::str;
use rpassword;

pub fn password_input_handle() -> String {
    let password = rpassword::prompt_password("password: ");
    match password {
        Err(error) => panic!("panic! with input: {:?}", error),
        Ok(string) => return string,
    };
}

pub fn input_handle(prompt: &str, lowercase: bool) -> String {
    let mut input_string = String::new();
    
    loop {            
        print!("{} > ", prompt);
        input_string.clear();
        stdout().flush().ok();
        let stdin_result = io::stdin().read_line(&mut input_string);
        let _stdin = match stdin_result {
            Err(error) => panic!("panic! with input: {:?}", error),
            Ok(usize) => usize,
        };
        break;
    }
    match lowercase {
        true => return input_string.trim().to_string().to_lowercase(),
        false => return input_string.trim().to_string(),
    }
}

pub fn input_handle_integer(array_bounds_limiter: &usize, array_total_length: &usize) -> usize {
    let base_input = input_handle("enter a number:", false);
    match &base_input.parse::<usize>() {
        Err(error) => {
            println!("err: invalid entry. {}", error);
            let value = input_handle_integer(array_bounds_limiter, array_total_length);
            return value;
        },
        Ok(value) => {
            if value >= array_total_length {
                println!("err: invalid entry");
                let value = input_handle_integer(array_bounds_limiter, array_total_length);
                return value;
            }
            if array_bounds_limiter.eq(&0) {
                return *value;
            }
            if array_bounds_limiter <= value {
                println!("err: invalid entry");
                let value = input_handle_integer(array_bounds_limiter, array_total_length);
                return value;
            }
            return *value;
        }
    }
}

pub fn clear_screen() {
    stdout().flush().ok();
    let clear_result = Command::new("clear").status();
    match clear_result {
        Err(error) => panic!("panic! stdout error: {:?}", error),
        Ok(exit) => exit,
    };
}

pub fn confirmation_bool(base_prompt: &String) -> bool {
    let confirmation_string = base_prompt.clone() + " (yes/no)";
    let command: String = input_handle(&confirmation_string, true);
    match command.as_str() {
        "yes" => {
            return true;
        },
        "no" => {
            return false;
        },
        &_=> {
            let confirm = confirmation_bool(&base_prompt);
            return confirm;
        },
    }
}