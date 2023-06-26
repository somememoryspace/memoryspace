use std::process::Command;
use std::str;
use std::fs;

use crate::ARRAY;
use crate::index;
use crate::input;
use crate::input::confirmation_bool;

pub fn unlock_and_read() {
    index::index_table_display();
    let selection = input::input_handle("selection:", false);
    match &selection.parse::<usize>() {
        Err(error) => {
            println!("err: invalid entry. {}", error);
            return;
        },
        Ok(value) => {
            let result = ARRAY.lock();
            let _result = match result {
                Err(error) => panic!("panic! table display error: {:?}", error),
                Ok(mg) => {
                    let filepath = mg.get(*value);
                    match filepath {
                        None => panic!("panic! array error"),
                        Some(index_item) => {
                            let linkage = index_item.get_system_linkage();
                            if linkage.contains("dead") {
                                println!("err: attempting to decrypt a dead file");
                                return;
                            }
                            let filepath = index_item.get_system_path();
                            print!("produce temp file? ");
                            let confirmation = confirmation_bool();
                            let passphrase = input::password_input_handle();
                            //gpg
                            gpg_decrypt_handle(passphrase, filepath.to_string());
                            match confirmation {
                                true => {
                                    return;
                                },
                                false => {
                                    let filepath = filepath.replace(".gpg", "");
                                    output_temp_file(&filepath);
                                    delete_temp_file(&filepath);
                                    return;
                                }
                            }
                        },
                    }
                },
            };
        }
    };
}

pub fn gpg_decrypt_handle(passphrase: String, filepath: String) {
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
}

fn delete_temp_file(filepath: &String) {
    let init_file = fs::remove_file(filepath);
    let _result = match init_file {
        Ok(()) => (),
        Err(error) => panic!("panic! delete file error: {:?}", error)
    };
}

fn output_temp_file(filepath: &String) {
    let init_file = fs::read_to_string(filepath);
    let result = match init_file {
        Ok(file) => file,
        Err(error) => panic!("panic! read file error: {:?}", error)
    };
    println!("{}",result);
}