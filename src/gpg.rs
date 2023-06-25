use std::process::Command;
use std::str;
use std::fs;

use crate::index;
use crate::input;

pub fn unlock_and_read() {
    let index = index::index_file_load();
    let selection = input::input_handle("selection:", false);
    match &selection.parse::<usize>() {
        Err(error) => {
            println!("err: invalid entry. {}", error);
            return;
        },
        Ok(value) => {
            let filepath = &index[*value];
            let passphrase = input::password_input_handle();
            //gpg
            let run_command = Command::new("sh")
                .arg("-c")
                .arg(format!("gpg --batch --pinentry-mode loopback --cipher-algo AES256 --passphrase {passphrase} {filepath}"))
                .output();
            let result = match run_command {
                Ok(output) => output.stderr,
                Err(error) => panic!("panic! error running gpg: {:?}", error)    
            };
            let string = match str::from_utf8(&result) {
                Ok(value) => value,
                Err(error) => panic!("panic! invalid utf-8 {}", error),
            };
            if string.contains("gpg: decryption failed: Bad session key") {
                println!("err: decryption error. bad password.");
                return;
            }
            println!("decrypt: unlock complete.");
            let filepath = filepath.replace(".gpg", "");
            output_temp_file(&filepath);
            delete_temp_file(&filepath);
        }
    };
}

pub fn delete_temp_file(filepath: &String) {
    let init_file = fs::remove_file(filepath);
    let _result = match init_file {
        Ok(()) => (),
        Err(error) => panic!("panic! delete file error: {:?}", error)
    };
}

pub fn output_temp_file(filepath: &String) {
    let init_file = fs::read_to_string(filepath);
    let result = match init_file {
        Ok(file) => file,
        Err(error) => panic!("panic! read file error: {:?}", error)
    };
    println!("{}",result);
}