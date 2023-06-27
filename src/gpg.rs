use std::process::Command;
use std::str;

pub fn gpg_decrypt_handle(passphrase: String, filepath: String) -> bool {
    if !(filepath.contains(".gpg")) {
        println!("err: attempting to decrypt a non gpg instance.");
        return false;
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
        return false;
    }
    println!("decrypt: complete.");
    return true;
}

pub fn gpg_encrypt_handle(password: String, filepath: String) -> bool {
    let run_command = Command::new("sh")
    .arg("-c")
    .arg(format!("gpg -c --batch --pinentry-mode loopback --cipher-algo AES256 --passphrase {password} {filepath}"))
    .output();
    let _result = match run_command {
        Ok(output) => output.stderr,
        Err(error) => panic!("panic! error running gpg: {:?}", error)    
    };
    println!("encrypt: complete.");
    return true;
}