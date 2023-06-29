use std::io::{stdout, Write};
use std::process::exit;
use std::str;
use std::fs;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::sync::MutexGuard;
use crate::index::IndexItem;

use input::clear_screen;

mod input;
mod index;
mod gpg;
mod file;

const VERSION: f32 = 1.0;

lazy_static! {
    static ref ARRAY: Mutex<Vec<index::IndexItem>> = Mutex::new(vec![]);
}

fn load_array(data_filepath: &String, mut mutex_guard: MutexGuard<'_,Vec<IndexItem>>) {
    mutex_guard.clear(); //clear previous
<<<<<<< HEAD
    let loaded_file: Vec<String> = fs::read_to_string(data_filepath)
    .expect("err: array load error")
=======
    let loaded_file: Vec<String> = fs::read_to_string(&data_filepath)
    .expect("panic! load error")
>>>>>>> 5ea116b (refactor)
    .split("\n")
    .map(|line| line.to_string())
    .collect();
    for (i,val) in loaded_file.iter().enumerate() {
        if val.len() == 0 {
            continue;
        } else {
            let index_item = index::IndexItem::new(
                i,
                &val,
                &file::validate_path_desc(&val)
            );
            mutex_guard.push(index_item);
        }
    }
}


fn command_prompt() {
    let index = vec![
        "index-list",
        "   - print the index list",
        "index-add",
        "   - add entry to the index list",
        "index-remove",
        "   - remove entry from the index list",
        "index-decrypt",
        "   - decrypt entry from the index list",
        "index-encrypt",
        "   - encrypt new entry to the index list",
    ];
    let sys = vec![
        "sys-version",
        "   - print the current binary version",
        "sys-exit",
        "   - quit the program",
    ];
    let other = vec![
        "clear",
        "   - clear the screen",
        "help",
        "   - list the runnable commands",
    ];
    println!("--index--");
    for x in &index {
        println!("{x}");
    }
    println!();
    println!("--sys--");
    for x in &sys {
        println!("{x}");
    }
    println!();
    println!("--other--");
    for x in &other {
        println!("{x}");
    }
    println!();
    stdout().flush().ok();
}

fn command_proc(command: &str, data_filepath: &String, version: f32) {
    println!();
    let result = ARRAY.lock();
    let _result = match result {
        Ok(mut mutex_guard) => {
            match command {
                "index-list" => {
                    println!("index: printing current index file");
                    index::index_table_display(&mutex_guard) 
                },
                "index-add" => {
                    println!("index: adding new path entry to index"); 
                    index::index_table_display(&mutex_guard);
                    let filepath = input::input_handle("new file path",false);
                    let index_item = index::IndexItem::new(
                        mutex_guard.len(),
                        &filepath,
                        &file::validate_path_desc(&filepath),
                    );
                    mutex_guard.push(index_item);
                    file::overwrite_file(data_filepath, &mutex_guard);
                    load_array(data_filepath, mutex_guard);
                },
                "index-remove" => {
                    println!("index: removing entry");
                    index::index_table_display(&mutex_guard);
                    mutex_guard.remove(input::input_handle_integer());
                    file::overwrite_file(data_filepath, &mutex_guard);
                    load_array(data_filepath, mutex_guard);
                },
                "index-encrypt" => {
                    println!("index: encrypt an entry"); 
                    index::index_table_display(&mutex_guard);
                    let filepath = input::input_handle("new file path",false);
                    file::create_file(&filepath, true);
                    let success = gpg::gpg_encrypt_handle(&input::password_input_handle(), &filepath);
                    match success {
                        true => {
                                let new_filepath = filepath.clone() + ".gpg";
                                let index_item = index::IndexItem::new(
                                mutex_guard.len(),
                                &new_filepath,
                                &file::validate_path_desc(&filepath)
                            );
                            mutex_guard.push(index_item);
                            file::overwrite_file(data_filepath, &mutex_guard);
                            load_array(data_filepath, mutex_guard);
<<<<<<< HEAD
                            file::delete_temp_file(&filepath);
=======
>>>>>>> 5ea116b (refactor)
                            return;
                        },
                        false => {
                            println!("err: encrypt process failed");
                            return;
                        }
                    }

                },
                "index-decrypt" => {
                    println!("index: decrypt an entry");
                    index::index_table_display(&mutex_guard);
                    let temp_file_bool = input::confirmation_bool(&String::from("produce output file?"));
                    let selection = input::input_handle_integer();
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
                            let success = gpg::gpg_decrypt_handle(&input::password_input_handle(), &filepath);
                            match success {
                                true => {
                                    match temp_file_bool {
                                        true => {
                                            return;
                                        },
                                        false => {
                                            println!("{}", filepath);
                                            let filepath = filepath.replace(".gpg", "");
                                            file::output_temp_file(&filepath);
                                            file::delete_temp_file(&filepath);
                                            return;
                                        }
                                    }
                                },
                                false => {
                                    println!("err: decrypt process failed");
                                    return;
                                }
                            }

                        },
                    }
                },
                "sys-version" => {
                    println!("sys: version {}", version); 
                },
                "sys-exit" => {
                    println!("sys: exit"); 
                    exit(0);
                },
                "help" => {
                    println!("help: commands you can run");
                    command_prompt();
                },
                "clear" => {
                    input::clear_screen();
                },
                &_=> {
                    println!("err: invalid command.");
                },
            };
            println!();
        },
        Err(error) => panic!("panic! table display error: {:?}", error)    
    };

}

fn boot_sequence(data_filepath: &String) {
    let result = ARRAY.lock();
    let _result = match result {
        Ok(mutex_guard) => {
            clear_screen();
            load_array(data_filepath, mutex_guard);
            println!("welcome to memoryspace");
            if !(file::validate_file_bool(data_filepath)) {
                file::create_file(data_filepath, true);
            }
        }
        Err(error) => panic!("panic! table display error: {:?}", error)
    };
}

fn main() {
    let data_filepath = String::from("./data/data.ms");
    boot_sequence(&data_filepath);
    loop {
        let command: String = input::input_handle("memoryspace", true);
        command_proc(&command, &data_filepath, VERSION);
    }
}
