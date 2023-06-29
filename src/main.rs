use std::io::{stdout, Write};
use std::process::exit;
use std::str;
use std::fs;
use file::Configuration;
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
    let loaded_file: Vec<String> = fs::read_to_string(data_filepath)
    .expect("err: array load error")
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
                    let array_bounds_limiter: usize = mutex_guard.len() - 1;
                    mutex_guard.remove(input::input_handle_integer(&array_bounds_limiter));
                    file::overwrite_file(data_filepath, &mutex_guard);
                    load_array(data_filepath, mutex_guard);
                },
                "index-encrypt" => {
                    println!("index: encrypt an entry"); 
                    index::index_table_display(&mutex_guard);
                    let filepath = input::input_handle("new file path",false);
                    let passphrase = input::password_input_handle();
                    file::create_file(&filepath, true);
                    let success = gpg::gpg_encrypt_handle(&passphrase, &filepath);
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
                            file::delete_temp_file(&filepath);
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
                    let array_bounds_limiter: usize = mutex_guard.len() - 1;
                    let selection = input::input_handle_integer(&array_bounds_limiter);
                    let filepath = mutex_guard.get(selection);
                    let temp_file_bool = input::confirmation_bool(&String::from("produce output file?"));
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
                                false => {
                                    println!("err: decrypt process failed");
                                    return;
                                },
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

fn boot_sequence(data_filepath: &String, configuration_filepath: &String) {
    let result = ARRAY.lock();
    let _result = match result {
        Ok(mutex_guard) => {
            clear_screen();
            load_array(data_filepath, mutex_guard);
            println!("welcome to memoryspace");
            if !(file::validate_file_bool(data_filepath)) {
                file::create_file(data_filepath, true);
            }
            if !(file::validate_file_bool(configuration_filepath)) {
                file::create_file(configuration_filepath, true);
            }
        }
        Err(error) => panic!("panic! table display error: {:?}", error)
    };
}

fn thread_main(configuration: &Configuration) {
    boot_sequence(&configuration.get_data_filepath(), &configuration.get_configuration_path());
    loop {
        let command: String = input::input_handle("memoryspace", true);
        command_proc(&command, &configuration.get_data_filepath(), VERSION);
    }
}

fn main() {
    let previous_config: bool = input::confirmation_bool(&String::from("load a non-default config?"));
    match previous_config {
        false => {
            let configuration = file::Configuration::new(
               &String::from("./data/data.ms"),
               &String::from("./config/config.yml"),
               &String::from("/usr/bin/gpg"),
               &false,
            );
            thread_main(&configuration);
        },
        true => {
            let configuration_filepath: String = input::input_handle("configuration filepath", true);
            match file::validate_file_bool(&configuration_filepath) {
                false => {
                    println!("err: invalid filepath provided.");
                    match input::confirmation_bool(&String::from("try again?")) {
                        true => main(),
                        false => exit(0),
                    }
                },
                true => {
                    let configuration = file::Configuration::parse_from_file(&configuration_filepath);
                    thread_main(&configuration);
                }
            };
        }
    };
}
