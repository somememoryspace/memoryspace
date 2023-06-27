use std::process::Command;
use std::str;
use std::fs;
use std::sync::MutexGuard;
use crate::index::IndexItem;

pub fn unlock_and_read(selection: usize, password: String, temp_file_bool: bool, mutex_guard: MutexGuard<'_,Vec<IndexItem>>) {
    let filepath = mutex_guard.get(selection);
    match filepath {
        None => panic!("panic! array indexing error"),
        Some(index_item) => {
            let filepath = index_item.get_system_path();
            let linkage = index_item.get_system_linkage();
            if linkage.contains("dead") {
                println!("err: attempting to decrypt a dead file");
                return;
            }
            gpg_decrypt_handle(password, filepath.to_string());
            match temp_file_bool {
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
}

pub fn encrypt_file(filepath: String) {
   
}

pub fn gpg_decrypt_handle(passphrase: String, filepath: String) {
    if !(filepath.contains(".gpg")) {
        println!("err: attempting to decrypt a non gpg instance.");
        return;
    }
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
    println!("decrypt: complete.");
}

pub fn gpg_encrypt_handle(passphrase: String, filepath: String) {
    let run_command = Command::new("sh")
    .arg("-c")
    .arg(format!("gpg -c --batch --pinentry-mode loopback --cipher-algo AES256 --passphrase {passphrase} {filepath}.gpg"))
    .output();
    let _result = match run_command {
        Ok(output) => output.stderr,
        Err(error) => panic!("panic! error running gpg: {:?}", error)    
    };
    println!("encrypt: complete.");
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