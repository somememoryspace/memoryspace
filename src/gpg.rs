use std::process::Command;
use std::str;

pub fn gpg_decrypt_handle(passphrase: &String, filepath: &String) -> bool {
    if !(filepath.contains(".gpg")) {
        println!("err: attempting to decrypt a non gpg instance.");
        return false;
    }
    let run_command = Command::new("sh")
    .arg("-c")
    .arg(format!("gpg --batch --pinentry-mode loopback --cipher-algo AES256 --passphrase {passphrase} {filepath}"))
    .output();
    let result = match run_command {
        Err(error) => panic!("panic! error running gpg: {:?}", error),
        Ok(output) => output.stderr,   
    };
    match str::from_utf8(&result) {
        Err(error) => panic!("panic! invalid utf-8 {}", error),
        Ok(value) => {
            if value.contains("gpg: decryption failed: Bad session key") {
                println!("err: decryption error. bad password.");
                return false;
            }
            println!("decrypt: complete.");
            return true;
        }
    };
}

pub fn gpg_encrypt_handle(password: &String, filepath: &String) -> bool {
    let run_command = Command::new("sh")
    .arg("-c")
    .arg(format!("gpg -c --batch --pinentry-mode loopback --cipher-algo AES256 --passphrase {password} {filepath}"))
    .output();
    match run_command {
        Err(error) => panic!("panic! error running gpg: {:?}", error),
        Ok(_) => {
            println!("encrypt: complete.");
            return true;
        } 
    };
}