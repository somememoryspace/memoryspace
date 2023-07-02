use std::collections::HashSet;
use std::io::{stdout, Write};
use std::process::exit;
use std::str;
use file::Configuration;
use lazy_static::lazy_static;
use std::sync::Mutex;
use std::sync::MutexGuard;
use crate::index::{IndexItemVolatile};

use input::clear_screen;

mod input;
mod index;
mod gpg;
mod file;

const VERSION: f32 = 1.0;

lazy_static! {
    static ref APPDATA: Mutex<(Vec<index::IndexItem>,HashSet<String>)> = Mutex::new(
        (   
            Vec::new(),
            HashSet::new()
        )
    );
}

fn load_data(data_filepath: &String, mut mutex_guard: MutexGuard<'_,(Vec<index::IndexItem>,HashSet<String>)>) {
    mutex_guard.0.clear(); //clear previous
    let loaded_file: Vec<String> = file::read_paths_list(&data_filepath);
    for (i,val) in loaded_file.iter().enumerate() {
        if val.len() == 0 {
            continue;
        } else {
            let index_item = index::IndexItem::new(
                i,
                &val,
                &file::validate_path_desc(&val)
            );
            mutex_guard.0.push(index_item);
            mutex_guard.1.insert(val.to_owned());
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
        "index-discover",
        "   - discover gpg files using a path",
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
    let result = APPDATA.lock();
    match result {
        Ok(mut mutex_guard) => {
            match command {
                "index-list" => {
                    println!("index: printing current index file");
                    index::index_table_display(&mutex_guard.0) 
                },
                "index-add" => {
                    println!("index: adding new path entry to index"); 
                    index::index_table_display(&mutex_guard.0);
                    let filepath = input::input_handle("new file path",false);
                    let index_item = index::IndexItem::new(
                        mutex_guard.0.len(),
                        &filepath,
                        &file::validate_path_desc(&filepath),
                    );
                    mutex_guard.0.push(index_item);
                    mutex_guard.1.insert(filepath);
                    file::overwrite_file(data_filepath, &mutex_guard.0);
                    load_data(data_filepath, mutex_guard);
                },
                "index-remove" => {
                    println!("index: removing entry");
                    index::index_table_display(&mutex_guard.0);
                    let array_bounds_limiter: usize = &mutex_guard.0.len() - 1;
                    let selection = input::input_handle_integer(&array_bounds_limiter);
                    let selection_path = &mutex_guard.0[selection].get_system_path().to_string();
                    mutex_guard.1.remove(selection_path);
                    mutex_guard.0.remove(selection);
                    file::overwrite_file(data_filepath, &mutex_guard.0);
                    load_data(data_filepath, mutex_guard);
                },
                "index-encrypt" => {
                    println!("index: encrypt an entry"); 
                    index::index_table_display(&mutex_guard.0);
                    let filepath = input::input_handle("new file path",false);
                    let passphrase = input::password_input_handle();
                    file::create_file(&filepath, true);
                    let success = gpg::gpg_encrypt_handle(&passphrase, &filepath);
                    match success {
                        false => {
                            println!("err: encrypt process failed");
                            return;
                        },
                        true => {
                                let new_filepath = filepath.clone() + ".gpg";
                                let index_item = index::IndexItem::new(
                                mutex_guard.0.len(),
                                &new_filepath,
                                &file::validate_path_desc(&filepath)
                                );
                            mutex_guard.0.push(index_item);
                            mutex_guard.1.insert(new_filepath);
                            file::overwrite_file(data_filepath, &mutex_guard.0);
                            load_data(data_filepath, mutex_guard);
                            file::delete_temp_file(&filepath);
                            return;
                        },
                    }
                },
                "index-decrypt" => {
                    println!("index: decrypt an entry");
                    index::index_table_display(&mutex_guard.0);
                    let array_bounds_limiter: usize = mutex_guard.0.len() - 1;
                    let selection = input::input_handle_integer(&array_bounds_limiter);
                    let filepath = mutex_guard.0.get(selection);
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
                "index-discover" => {
                    println!("index: discover files against a provided path");
                    let discovery = file::discover_files(
                        &input::input_handle("filepath to discover",false), 
                        &String::from("/**/*.gpg"),
                        &mutex_guard.1,
                    );
                    if &discovery.len() == &0 {
                        println!("err: no matches found");
                        return;
                    }
                    let volatile_list: Vec<IndexItemVolatile> = index::produce_volatile_list(&discovery);
                    index::index_table_display_volatile(&volatile_list);
                }   
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
    let result = APPDATA.lock();
    match result {
        Err(error) => panic!("panic! table display error: {:?}", error),
        Ok(mutex_guard) => {
            clear_screen();
            load_data(data_filepath, mutex_guard);
            println!("welcome to memoryspace");
            if !(file::validate_file_bool(data_filepath)) {
                file::create_file(data_filepath, true);
            }
            if !(file::validate_file_bool(configuration_filepath)) {
                file::create_file(configuration_filepath, true);
            }
        }
    };
}

fn thread_main(configuration: &Configuration) {
    boot_sequence(configuration.get_data_filepath(), configuration.get_configuration_path());
    loop {
        let command: String = input::input_handle("memoryspace", true);
        command_proc(&command, configuration.get_data_filepath(), VERSION);
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
